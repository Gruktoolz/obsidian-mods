// Tectonic-Horizon. Adapted from Apollo's CC0-1.0 Tectonic 3.0.1.
// See THIRD_PARTY_NOTICES.md and upstream.json for provenance.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::PathBuf, sync::OnceLock};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub general: General,
    pub global_terrain: GlobalTerrain,
    pub continents: Continents,
    pub islands: Islands,
    pub oceans: Oceans,
    pub biomes: Biomes,
}

macro_rules! config_section {
    ($name:ident { $($field:ident: $ty:ty = $default:expr),* $(,)? }) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(default, deny_unknown_fields)]
        pub struct $name { $(pub $field: $ty,)* }
        impl Default for $name {
            fn default() -> Self { Self { $($field: $default,)* } }
        }
    };
}
config_section!(General {
    mod_enabled: bool = true,
    snow_start_offset: i32 = 128
});
config_section!(GlobalTerrain {
    vertical_scale: f64 = 1.125,
    increased_height: bool = false,
    lava_tunnels: bool = true,
});
config_section!(Continents {
    ocean_offset: f64 = -0.8,
    continents_scale: f64 = 0.13,
    erosion_scale: f64 = 0.25,
    ridge_scale: f64 = 0.25,
    underground_rivers: bool = true,
    river_lanterns: bool = true,
    flat_terrain_skew: f64 = 0.1,
    rolling_hills: bool = true,
    jungle_pillars: bool = true,
});
config_section!(Islands {
    enabled: bool = true,
    noise_scale: f64 = 0.11
});
config_section!(Oceans {
    ocean_depth: f64 = -0.22,
    deep_ocean_depth: f64 = -0.45,
    monument_offset: i32 = -30,
    remove_frozen_ocean_ice: bool = false,
});
config_section!(Biomes {
    temperature_multiplier: f64 = 1.0,
    temperature_scale: f64 = 0.25,
    vegetation_multiplier: f64 = 1.0,
    vegetation_scale: f64 = 0.25,
});

impl Config {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let config: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        for key in [
            "vertical_scale",
            "continents_scale",
            "erosion_scale",
            "ridge_scale",
            "islands_scale",
            "temperature_scale",
            "vegetation_scale",
        ] {
            let value = config.value(key)?;
            if !value.is_finite() || !(0.001..=100.0).contains(&value) {
                return Err(format!("{key} must be finite and in 0.001..=100"));
            }
        }
        for key in [
            "ocean_offset",
            "flat_terrain_skew",
            "ocean_depth",
            "deep_ocean_depth",
            "temperature_multiplier",
            "vegetation_multiplier",
        ] {
            let value = config.value(key)?;
            if !value.is_finite() || !(-100.0..=100.0).contains(&value) {
                return Err(format!("{key} must be finite and in -100..=100"));
            }
        }
        if !(-512..=512).contains(&config.general.snow_start_offset)
            || !(-128..=128).contains(&config.oceans.monument_offset)
        {
            return Err("snow_start_offset or monument_offset is out of bounds".into());
        }
        Ok(config)
    }

    pub fn value(&self, key: &str) -> Result<f64, String> {
        Ok(match key {
            "vertical_scale" => self.global_terrain.vertical_scale,
            "lava_tunnels" => f64::from(self.global_terrain.lava_tunnels),
            "ocean_offset" => self.continents.ocean_offset,
            "continents_scale" => self.continents.continents_scale,
            "erosion_scale" => self.continents.erosion_scale,
            "ridge_scale" => self.continents.ridge_scale,
            "underground_rivers" => -f64::from(self.continents.underground_rivers),
            "flat_terrain_skew" => self.continents.flat_terrain_skew,
            "rolling_hills" => f64::from(self.continents.rolling_hills),
            "jungle_pillars" => f64::from(self.continents.jungle_pillars),
            "ocean_depth" => self.oceans.ocean_depth,
            "deep_ocean_depth" => self.oceans.deep_ocean_depth,
            "islands_scale" => self.islands.noise_scale,
            "temperature_multiplier" => self.biomes.temperature_multiplier,
            "temperature_scale" => self.biomes.temperature_scale,
            "vegetation_multiplier" => self.biomes.vegetation_multiplier,
            "vegetation_scale" => self.biomes.vegetation_scale,
            _ => return Err(format!("unknown Tectonic configuration key: {key}")),
        })
    }
}

pub fn config_path() -> PathBuf {
    if let Some(path) = std::env::var_os("TECTONIC_HORIZON_CONFIG") {
        return path.into();
    }
    let mut args = std::env::args_os();
    while let Some(arg) = args.next() {
        if (arg == "--game-dir" || arg == "--dir")
            && let Some(dir) = args.next()
        {
            return PathBuf::from(dir).join("config/tectonic-horizon.json");
        }
    }
    PathBuf::from("config/tectonic-horizon.json")
}

static ACTIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
pub fn set_active(enabled: bool) {
    ACTIVE.store(enabled, std::sync::atomic::Ordering::Relaxed);
}
pub fn active() -> bool {
    ACTIVE.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn load_config(path: &std::path::Path) -> Config {
    use std::io::Read;
    let read = || -> Result<Config, String> {
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Config::default());
            }
            Err(error) => return Err(format!("cannot read configuration: {error}")),
        };
        let mut text = String::new();
        file.take(65537)
            .read_to_string(&mut text)
            .map_err(|error| format!("cannot read configuration: {error}"))?;
        if text.len() > 65536 {
            return Err("configuration exceeds 64 KiB".into());
        }
        Config::from_json(&text)
    };
    read().unwrap_or_else(|error| {
        eprintln!(
            "[Tectonic-Horizon] {}: {error}; using defaults",
            path.display()
        );
        Config::default()
    })
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| load_config(&config_path()))
}

// Translate 1.21 density operations into Horizon's 26.3 density schema.
// Sampling remains entirely in Horizon's Rust density compiler.
pub fn lower(mut value: Value, config: &Config) -> Result<Value, String> {
    if let Value::Array(values) = &mut value {
        for child in values {
            *child = lower(child.take(), config)?;
        }
    }
    let Value::Object(mut object) = value else {
        return Ok(value);
    };
    let kind = object
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if let Some(name) = object.remove("Name") {
        object.insert("id".into(), name);
        if let Some(properties) = object.remove("Properties") {
            object.insert("properties".into(), properties);
        }
    }
    if let Some(first) = object.remove("firstOctave") {
        let amplitudes = object.remove("amplitudes").ok_or("missing amplitudes")?;
        object.insert("base_octave".into(), first);
        object.insert(
            "octave_count".into(),
            json!(amplitudes.as_array().ok_or("amplitudes array")?.len()),
        );
        object.insert("amplitude_modifiers".into(), amplitudes);
        object.insert("normalize".into(), json!("legacy"));
    }
    if kind == "tectonic:config_constant" {
        return Ok(json!(
            config.value(object["key"].as_str().ok_or("config constant key")?)?
        ));
    }
    if kind == "tectonic:config_noise" {
        let scale = config.value(object["key"].as_str().ok_or("config noise key")?)?;
        object.remove("key");
        object.insert("type".into(), json!("minecraft:noise"));
        object.insert("xz_scale".into(), json!(scale));
        object.insert("y_scale".into(), json!(0));
        object.insert("shift_y".into(), json!(0));
    }
    if [
        "minecraft:flat_cache",
        "minecraft:cache_2d",
        "minecraft:cache_once",
        "minecraft:cache_all_in_cell",
    ]
    .contains(&kind.as_str())
    {
        object.insert("type".into(), json!("minecraft:cache"));
    }
    if kind == "minecraft:shifted_noise" {
        object.insert("type".into(), json!("minecraft:noise"));
    }
    if kind == "minecraft:y_clamped_gradient" {
        object.insert("type".into(), json!("minecraft:gradient"));
        object.insert("axis".into(), json!("y"));
        for (old, new) in [("from_y", "from_coordinate"), ("to_y", "to_coordinate")] {
            if let Some(value) = object.remove(old) {
                object.insert(new.into(), value);
            }
        }
    }
    if kind == "minecraft:interpolated" {
        object.insert("cell_size_xz".into(), json!(4));
        object.insert("cell_size_y".into(), json!(8));
    }
    for (old, new) in [
        ("argument", "input"),
        ("argument1", "left"),
        ("argument2", "right"),
    ] {
        if let Some(value) = object.remove(old) {
            object.insert(new.into(), value);
        }
    }
    for child in object.values_mut() {
        *child = lower(child.take(), config)?;
    }
    Ok(Value::Object(object))
}

fn entries(bundle: &str, config: &Config) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    let tables = [
        super::tectonic_terrain::BASE,
        super::tectonic_terrain::MOD,
        if config.global_terrain.increased_height {
            super::tectonic_terrain::TALL
        } else {
            &[]
        },
    ];
    for table in tables {
        for &(kind, id, text) in table {
            if kind == bundle {
                let value =
                    serde_json::from_str(text).expect("checked embedded Tectonic definition");
                out.insert(
                    id.into(),
                    lower(value, config).expect("checked Tectonic configuration operation"),
                );
            }
        }
    }
    out
}

pub fn populate(bundle: &str, base: &mut BTreeMap<String, String>, config: &Config) {
    if !config.general.mod_enabled {
        return;
    }
    if ![
        "noises",
        "densities",
        "settings",
        "rules",
        "biomes",
        "dimension_types",
    ]
    .contains(&bundle)
    {
        return;
    }
    let mut additions = entries(bundle, config);
    if bundle == "densities" {
        if !config.islands.enabled {
            let original = additions
                .remove("tectonic:noise/raw_continents")
                .expect("raw continents");
            additions.insert(
                "tectonic:noise/raw_continents".into(),
                json!({"type":"minecraft:max", "left": original, "right": -0.49}),
            );
        }
        let upper = if config.global_terrain.increased_height {
            640
        } else {
            320
        };
        additions.insert("tectonic:preliminary_surface_level".into(), json!({
            "type": "minecraft:find_top_surface", "lower_bound": -64, "upper_bound": upper,
            "cell_height": 8, "density": {"type":"minecraft:sub", "left":"minecraft:overworld/noise_router/initial_density_without_jaggedness", "right": 0.390625}
        }));
    }
    if bundle == "rules" {
        let mut source = entries("settings", config)
            .remove("minecraft:overworld")
            .expect("overworld settings");
        let surface = source["surface_rule"].take();
        // Keep the host's ore-vein rules, whose registry changed in 26.3.
        let mut rules = Vec::new();
        fn ores(
            value: &Value,
            definitions: &BTreeMap<String, String>,
            out: &mut Vec<Value>,
            depth: usize,
        ) {
            if depth > 64 {
                return;
            }
            if let Some(name) = value.as_str()
                && let Some(reference) = definitions
                    .get(name)
                    .and_then(|s| serde_json::from_str::<Value>(s).ok())
            {
                ores(&reference, definitions, out, depth + 1);
                return;
            }
            if value["type"] == "minecraft:ore_vein" {
                out.push(value.clone());
                return;
            }
            if let Some(values) = value.as_object() {
                for child in values.values() {
                    ores(child, definitions, out, depth + 1);
                }
            }
            if let Some(values) = value.as_array() {
                for child in values {
                    ores(child, definitions, out, depth + 1);
                }
            }
        }
        if let Some(original) = base
            .get("minecraft:overworld")
            .and_then(|s| serde_json::from_str::<Value>(s).ok())
        {
            ores(&original, base, &mut rules, 0);
        }
        rules.push(surface);
        additions.insert(
            "tectonic:overworld".into(),
            json!({"type":"minecraft:sequence", "sequence":rules}),
        );
    }
    if bundle == "settings"
        && let (Some(port), Some(host)) = (
            additions.get_mut("minecraft:overworld"),
            base.get("minecraft:overworld"),
        )
    {
        let mut settings: Value = serde_json::from_str(host).expect("host overworld settings");
        for key in [
            "temperature",
            "vegetation",
            "continents",
            "erosion",
            "depth",
            "ridges",
            "final_density",
        ] {
            settings["noise_router"][key] = port["noise_router"][key].clone();
        }
        settings["noise_router"]["chunk_surface_level"] =
            json!("tectonic:preliminary_surface_level");
        for key in [
            "barrier",
            "fluid_level_floodedness",
            "fluid_level_spread",
            "lava",
        ] {
            settings["aquifers"][key] = port["noise_router"][key].clone();
        }
        settings["aquifers"]["surface_level"] = json!("tectonic:preliminary_surface_level");
        // Spawn fitness and aquifer exclusion must sample the port's
        // climate functions rather than the vanilla island functions.
        let climate: BTreeMap<String, Value> = [
            "temperature",
            "vegetation",
            "continents",
            "erosion",
            "depth",
            "ridges",
        ]
        .into_iter()
        .map(|key| {
            (
                format!("minecraft:overworld/{key}"),
                port["noise_router"][key].clone(),
            )
        })
        .collect();
        fn remap(value: &mut Value, climate: &BTreeMap<String, Value>) {
            match value {
                Value::String(name) => {
                    if let Some(replacement) = climate.get(name) {
                        *value = replacement.clone();
                    }
                }
                Value::Array(values) => {
                    for child in values {
                        remap(child, climate);
                    }
                }
                Value::Object(values) => {
                    for child in values.values_mut() {
                        remap(child, climate);
                    }
                }
                _ => {}
            }
        }
        remap(&mut settings["aquifers"]["exclusion"], &climate);
        if let Some(points) = settings["spawn_target"].as_array_mut() {
            for point in points {
                if let Some(fields) = point.as_object_mut() {
                    for (old, new) in &climate {
                        if let Some(interval) = fields.remove(old) {
                            fields
                                .insert(new.as_str().expect("climate reference").into(), interval);
                        }
                    }
                }
            }
        }
        settings["noise"]["height"] = port["noise"]["height"].clone();
        settings["material_rule"] = json!("tectonic:overworld");
        *port = settings;
    }
    if bundle == "biomes"
        && config.oceans.remove_frozen_ocean_ice
        && let Some(text) = base.get_mut("minecraft:frozen_ocean")
    {
        let mut biome: Value = serde_json::from_str(text).expect("frozen ocean biome");
        biome["temperature"] = json!(0.5);
        *text = biome.to_string();
    }
    base.extend(
        additions
            .into_iter()
            .map(|(id, value)| (id, value.to_string())),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constants_keep_original_signs_and_defaults() {
        let config = Config::default();
        assert_eq!(config.value("underground_rivers").unwrap(), -1.0);
        assert_eq!(config.value("ocean_offset").unwrap(), -0.8);
        assert_eq!(config.value("vertical_scale").unwrap(), 1.125);
        assert!(config.value("typo").is_err());
    }
    #[test]
    fn partial_configs_keep_defaults_and_reject_typos_or_bad_scales() {
        let c = Config::from_json(r#"{"continents":{"underground_rivers":false}}"#).unwrap();
        assert_eq!(c.value("underground_rivers").unwrap(), 0.0);
        assert_eq!(c.value("ridge_scale").unwrap(), 0.25);
        assert!(Config::from_json(r#"{"biomes":{"temperature_scale":0}}"#).is_err());
        assert!(Config::from_json(r#"{"continents":{"ridge_scales":1}}"#).is_err());
    }
    #[test]
    fn every_embedded_density_lowers_without_custom_codecs() {
        let config = Config::default();
        for (id, value) in entries("densities", &config) {
            let text = value.to_string();
            for old in [
                "config_constant",
                "config_noise",
                "argument1",
                "flat_cache",
                "y_clamped_gradient",
            ] {
                assert!(!text.contains(old), "{id}: {old}");
            }
        }
    }
    #[test]
    fn config_noise_scales_positions_before_applying_shifts() {
        let value = lower(json!({"type":"tectonic:config_noise", "key":"continents_scale", "noise":"tectonic:parameter/continentalness", "shift_x":"minecraft:shift_x", "shift_z":0}), &Config::default()).unwrap();
        assert_eq!(value["type"], "minecraft:noise");
        assert_eq!(value["xz_scale"], 0.13);
        assert_eq!(value["y_scale"], 0);
        assert_eq!(value["shift_x"], "minecraft:shift_x");
    }
    #[test]
    fn disabled_mod_preserves_every_bundle() {
        let mut config = Config::default();
        config.general.mod_enabled = false;
        let original = BTreeMap::from([("minecraft:overworld".into(), "untouched".into())]);
        for bundle in ["settings", "densities", "rules", "noises", "biomes"] {
            let mut base = original.clone();
            populate(bundle, &mut base, &config);
            assert_eq!(base, original);
        }
    }
    #[test]
    fn tall_worlds_and_island_toggle_change_the_actual_graph() {
        let mut config = Config::default();
        config.global_terrain.increased_height = true;
        config.islands.enabled = false;
        let mut densities = BTreeMap::new();
        populate("densities", &mut densities, &config);
        let raw: Value = serde_json::from_str(&densities["tectonic:noise/raw_continents"]).unwrap();
        assert_eq!(raw["type"], "minecraft:max");
        assert_eq!(raw["right"], -0.49);
        let surface: Value =
            serde_json::from_str(&densities["tectonic:preliminary_surface_level"]).unwrap();
        assert_eq!(surface["upper_bound"], 640);
    }
}
