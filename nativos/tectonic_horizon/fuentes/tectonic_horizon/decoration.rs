// Native replacement for Tectonic's Lithostitched river decoration modifiers.
// Terrain sampling uses the same seeded river density as the terrain graph.
use super::{GeneratorData, GeneratorError, ProtoChunk};
use crate::{
    registry::BlockStateId,
    worldgen::{decode_noise, density::DensityRegistry, overrides::tectonic_horizon::config},
};
use horizon_core::{
    density::{DensitySampler, SamplerContext, graph::DensityCompiler},
    noise::Noise,
    random::{PositionalRandom, RandomSource, XoroshiroRandom},
};
use std::sync::{Arc, Mutex, OnceLock, Weak};

struct Samplers {
    data: Weak<GeneratorData>,
    seed: i64,
    river: Arc<dyn DensitySampler>,
    lanterns: Arc<dyn Noise>,
}

fn samplers(data: &Arc<GeneratorData>, seed: i64) -> Result<Arc<Samplers>, GeneratorError> {
    static CACHE: OnceLock<Mutex<Vec<Arc<Samplers>>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(Mutex::default)
        .lock()
        .map_err(|_| super::resource("Tectonic sampler cache poisoned"))?;
    cache.retain(|entry| entry.data.strong_count() > 0);
    if let Some(found) = cache
        .iter()
        .find(|entry| entry.seed == seed && entry.data.as_ptr() == Arc::as_ptr(data))
    {
        return Ok(found.clone());
    }
    let registry = DensityRegistry::from_json(&data.noises, &data.densities)?;
    let root = registry
        .reference("tectonic:underground_river/parameters")
        .ok_or_else(|| super::resource("missing Tectonic river parameters"))?;
    let river = DensityCompiler::new_seeded(registry.into_graph(), seed, false)
        .get_sampler(root)
        .map_err(crate::worldgen::DecodeError::from)?;
    let factory = XoroshiroRandom::new(seed).fork_positional();
    let lanterns = decode_noise(&data.noises["tectonic:river_lanterns"])?
        .create(&mut factory.from_hash_of("tectonic:river_lanterns"))
        .map_err(crate::worldgen::DecodeError::from)?;
    let result = Arc::new(Samplers {
        data: Arc::downgrade(data),
        seed,
        river,
        lanterns: Arc::new(lanterns),
    });
    if cache.len() >= 8 {
        cache.remove(0);
    }
    cache.push(result.clone());
    Ok(result)
}

fn state(data: &GeneratorData, name: &str, properties: &[(&str, &str)]) -> Option<BlockStateId> {
    let registry = data.registry();
    let mut value = registry.state(registry.default_state(name)?)?.clone();
    for &(key, property) in properties {
        value.properties.insert(key.into(), property.into());
    }
    registry.id(&value)
}

fn stone(chunk: &ProtoChunk, x: i32, y: i32, z: i32) -> bool {
    matches!(
        chunk.block_name(x, y, z),
        "minecraft:stone"
            | "minecraft:andesite"
            | "minecraft:diorite"
            | "minecraft:granite"
            | "minecraft:dripstone_block"
            | "minecraft:calcite"
            | "minecraft:tuff"
            | "minecraft:deepslate"
    )
}

pub(crate) fn decorate(
    data: &Arc<GeneratorData>,
    seed: i64,
    region: &mut [ProtoChunk; 9],
) -> Result<(), GeneratorError> {
    let config = config();
    if !crate::worldgen::overrides::tectonic_horizon::active() || !config.general.mod_enabled
        || !config.continents.underground_rivers
        || region[4].is_decorated()
    {
        return Ok(());
    }
    let center = region[4].position();
    // Refuse invalid neighborhoods before making any writes.
    for (i, chunk) in region.iter().enumerate() {
        if chunk.position().x != center.x + i as i32 % 3 - 1
            || chunk.position().z != center.z + i as i32 / 3 - 1
            || (chunk.terrain.generation_stage() as u8)
                < crate::worldgen::terrain::PreLightChunkStage::Terrain as u8
        {
            return Ok(());
        }
    }
    // Only the overworld has these surface biomes. Never decorate Nether/End.
    if !region[4].biome_at(8, 64, 8).starts_with("minecraft:")
        || matches!(
            region[4].biome_at(8, 64, 8),
            "minecraft:nether_wastes"
                | "minecraft:soul_sand_valley"
                | "minecraft:crimson_forest"
                | "minecraft:warped_forest"
                | "minecraft:basalt_deltas"
                | "minecraft:the_end"
                | "minecraft:end_highlands"
                | "minecraft:end_midlands"
                | "minecraft:small_end_islands"
                | "minecraft:end_barrens"
        )
    {
        return Ok(());
    }
    let samplers = samplers(data, seed)?;
    let chunk = &mut region[4];
    let mut random = XoroshiroRandom::new(
        seed ^ i64::from(center.x).wrapping_mul(341873128712)
            ^ i64::from(center.z).wrapping_mul(132897987541)
            ^ 0x746563746f6e6963,
    );
    let mut context = SamplerContext::cached();
    for _ in 0..32 {
        let x = random.next_int_bound(16).expect("positive bound");
        let z = random.next_int_bound(16).expect("positive bound");
        let y = 48 + random.next_int_bound(33).expect("positive bound");
        if chunk.block_name(x, y, z) != "minecraft:air" {
            continue;
        }
        if samplers
            .river
            .sample_value(&mut context, [center.x * 16 + x, y, center.z * 16 + z])
            .map_err(crate::worldgen::DecodeError::from)?
            < 1.0
        {
            continue;
        }
        for (dx, dy, dz, face) in [
            (0, 1, 0, "up"),
            (0, -1, 0, "down"),
            (1, 0, 0, "east"),
            (-1, 0, 0, "west"),
            (0, 0, 1, "south"),
            (0, 0, -1, "north"),
        ] {
            // Keep writes/support in the chunk so generation order cannot affect them.
            if !(0..16).contains(&(x + dx))
                || !(0..16).contains(&(z + dz))
                || !stone(chunk, x + dx, y + dy, z + dz)
            {
                continue;
            }
            if let Some(lichen) = state(
                data,
                "minecraft:glow_lichen",
                &[(face, "true"), ("waterlogged", "false")],
            ) {
                chunk
                    .terrain
                    .set_block_worldgen(x, y, z, lichen)
                    .map_err(|e| super::resource(e.to_string()))?;
            }
            break;
        }
    }
    if !config.continents.river_lanterns {
        return Ok(());
    }
    let wx = center.x * 16;
    let wz = center.z * 16;
    if samplers
        .lanterns
        .get3(f64::from(wx) * 0.17, 0.0, f64::from(wz) * 0.17)
        <= 0.9
    {
        return Ok(());
    }
    let x = 5 + random.next_int_bound(6).expect("positive bound");
    let z = 5 + random.next_int_bound(6).expect("positive bound");
    if samplers
        .river
        .sample_value(&mut context, [wx + x, 64, wz + z])
        .map_err(crate::worldgen::DecodeError::from)?
        < 1.0
    {
        return Ok(());
    }
    let Some(chain) = state(
        data,
        "minecraft:iron_chain",
        &[("axis", "y"), ("waterlogged", "false")],
    ) else {
        return Ok(());
    };
    let Some(lantern) = state(
        data,
        "minecraft:lantern",
        &[("hanging", "true"), ("waterlogged", "false")],
    ) else {
        return Ok(());
    };
    for y in 64..=94 {
        if chunk.block_name(x, y, z) != "minecraft:air" {
            break;
        }
        if !stone(chunk, x, y + 1, z) {
            continue;
        }
        let length = 2 + random.next_int_bound(7).expect("positive bound");
        if (0..=length).all(|dy| chunk.block_name(x, y - dy, z) == "minecraft:air") {
            for dy in 0..length {
                chunk
                    .terrain
                    .set_block_worldgen(x, y - dy, z, chain)
                    .map_err(|e| super::resource(e.to_string()))?;
            }
            chunk
                .terrain
                .set_block_worldgen(x, y - length, z, lantern)
                .map_err(|e| super::resource(e.to_string()))?;
        }
        break;
    }
    Ok(())
}
