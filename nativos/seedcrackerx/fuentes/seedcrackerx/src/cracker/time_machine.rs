// kaptainwutax.seedcrackerX.cracker.storage.TimeMachine: the search itself. From the pillars of the End, or from
// structures with enough bits, to structure seeds; from those, by decorators, by the hash the server sent, or by biomes,
// to the world seed. The phases and the order they run in are the Java's.
use super::scx_cracker_data::{Host, ProgressListener};
use super::scx_cracker_storage::DataStorage;
use super::scx_feature::{Detail, Feature, FeatureData, StructureClass};
use super::scx_lcg::LCG;
use super::scx_rand::ChunkRand;
use super::scx_seeds::{pillar_seed, structure_seed, world_seed};
use super::scx_version::MCVersion;
use super::scx_worldgen_random::WorldgenRandom;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, MutexGuard};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Biomes,
    StructureReduce,
    Structures,
    Lifting,
    Pillars,
}

impl Phase {
    pub fn next_phase(self) -> Option<Phase> {
        match self {
            Phase::Biomes => None,
            Phase::StructureReduce => Some(Phase::Biomes),
            Phase::Structures => Some(Phase::Biomes),
            Phase::Lifting => Some(Phase::StructureReduce),
            Phase::Pillars => Some(Phase::Structures),
        }
    }
}

#[derive(Default)]
struct State {
    pillar_seeds: Option<Vec<i32>>,
    structure_seeds: HashSet<i64>,
    world_seeds: HashSet<i64>,
}

pub struct TimeMachine {
    is_running: AtomicBool,
    should_terminate: AtomicBool,
    state: Mutex<State>,
    /// How many threads the brute force may use.
    threads: AtomicI32,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl TimeMachine {
    pub fn new() -> TimeMachine {
        let cores = std::thread::available_parallelism().map_or(4, |n| n.get() as i32);
        TimeMachine { is_running: AtomicBool::new(false), should_terminate: AtomicBool::new(false), state: Mutex::new(State::default()), threads: AtomicI32::new(cores.max(1)) }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn set_running(&self, running: bool) {
        self.is_running.store(running, Ordering::SeqCst);
    }

    pub fn should_terminate(&self) -> bool {
        self.should_terminate.load(Ordering::Relaxed)
    }

    pub fn terminate(&self) {
        self.should_terminate.store(true, Ordering::SeqCst);
    }

    /// For a test that wants the search to use a given number of threads.
    pub fn set_threads(&self, threads: i32) {
        self.threads.store(threads.max(1), Ordering::Relaxed);
    }

    fn threads(&self) -> usize {
        self.threads.load(Ordering::Relaxed).max(1) as usize
    }

    pub fn pillar_seeds(&self) -> Option<Vec<i32>> {
        lock(&self.state).pillar_seeds.clone()
    }

    pub fn structure_seeds(&self) -> HashSet<i64> {
        lock(&self.state).structure_seeds.clone()
    }

    pub fn set_structure_seeds(&self, seeds: HashSet<i64>) {
        lock(&self.state).structure_seeds = seeds;
    }

    /// `structureSeeds.add(seed)`: whether it was new.
    pub fn add_structure_seed(&self, seed: i64) -> bool {
        lock(&self.state).structure_seeds.insert(seed)
    }

    pub fn world_seeds(&self) -> HashSet<i64> {
        lock(&self.state).world_seeds.clone()
    }

    fn world_seed_count(&self) -> usize {
        lock(&self.state).world_seeds.len()
    }

    /// `timeMachine(partialWorldSeed, pillarSeed)`: the structure seed with those 16 bits from the pillar and the rest chosen.
    pub fn time_machine(inverse_lcg: &LCG, partial_world_seed: i64, pillar_seed: i32) -> i64 {
        let mut current: i64 = 0;
        current |= (partial_world_seed & 0xFFFF_0000) << 16;
        current |= (pillar_seed as i64) << 16;
        current |= partial_world_seed & 0xFFFF;
        current = inverse_lcg.next_seed(current);
        current ^ LCG::JAVA.multiplier
    }

    /// `poke(phase)`.
    pub fn poke(&self, data: &DataStorage, mut phase: Option<Phase>) {
        if self.world_seed_count() == 1 {
            return;
        }
        self.set_running(true);
        while let Some(p) = phase {
            if self.should_terminate() {
                break;
            }
            if p != Phase::Biomes && self.poke_structure_reduce(data) {
                phase = Some(Phase::Biomes);
                continue;
            } else if p == Phase::Structures {
                if !self.poke_structures(data) {
                    break;
                }
            } else if p == Phase::Lifting {
                if !self.poke_structures(data) && !self.poke_lifting(data) {
                    break;
                }
            } else if p == Phase::Pillars {
                if !self.poke_pillars(data) {
                    break;
                }
            } else if p == Phase::Biomes && !self.poke_biomes(data) {
                break;
            }
            phase = p.next_phase();
        }
        if !self.should_terminate() {
            let found = {
                let s = lock(&self.state);
                if s.world_seeds.len() == 1 { s.world_seeds.iter().next().copied() } else { None }
            };
            if let Some(seed) = found {
                data.host.world_seed_found(seed);
            }
        }
    }

    fn poke_pillars(&self, data: &DataStorage) -> bool {
        let host = &data.host;
        let pillar = match lock(&data.pillar_data).clone() {
            Some(p) if lock(&self.state).pillar_seeds.is_none() => p,
            _ => return false,
        };
        lock(&self.state).pillar_seeds = Some(Vec::new());
        host.debug("====================================");
        host.warn("tmachine.lookingForPillarSeed", &[]);
        let mut found = Vec::new();
        let mut pillar_seed = 0i32;
        while pillar_seed < 1 << 16 && !self.should_terminate() {
            if pillar.test(pillar_seed as i64) {
                host.print_seed("tmachine.foundPillarSeed", pillar_seed as i64);
                found.push(pillar_seed);
                lock(&self.state).pillar_seeds = Some(found.clone());
            }
            pillar_seed += 1;
        }
        if !found.is_empty() {
            host.warn("tmachine.pillarSeedSearchFinished", &[]);
        } else {
            host.error("finishedSearchNoResult");
        }
        true
    }

    /// The data the structure search tests: the structures, and the decorators of before 1.18 (the outpost, whose village
    /// check the library has wrong, is left out).
    fn search_cache(data: &DataStorage) -> Vec<FeatureData> {
        let mut cache = Vec::new();
        for e in data.base_data() {
            let keep = match &*e.feature {
                Feature::Decorator(d) => !d.is_seedcracker_decorator() || d.version.is_older_than(MCVersion::v1_18),
                Feature::Structure(_) => true,
            };
            if keep && !matches!(&*e.feature, Feature::Structure(s) if s.class == StructureClass::PillagerOutpost) {
                cache.push(e);
            }
        }
        cache
    }

    fn poke_lifting(&self, data: &DataStorage) -> bool {
        let host = &data.host;
        if !lock(&self.state).structure_seeds.is_empty() || data.get_lifting_bits() < 40.0 {
            return false;
        }
        let version = host.version();
        let mut data_list: Vec<FeatureData> = Vec::new();
        for e in data.base_data() {
            if let Feature::Structure(s) = &*e.feature {
                if s.class.is_old_structure() || s.class == StructureClass::Shipwreck {
                    data_list.push(e);
                }
            }
        }
        let cache = Self::search_cache(data);
        host.warn("tmachine.startLifting", &[data_list.len().to_string()]);

        // You could first lift on 1L<<18 with %2 since that would be a smaller range
        // Then lift on 1<<19 with those 1<<18 fixed with % 4 and for nextInt(24)
        // You can even do %8 on 1<<20 (however we included shipwreck so only nextInt(20) so 1<<19 is the max here
        let mut lower_bits_found: Vec<i64> = Vec::new();
        let mut rand = ChunkRand::new();
        'lower: for lower_bits in 0..(1i64 << 19) {
            for d in &data_list {
                let Feature::Structure(s) = &*d.feature else { continue };
                let Detail::Region(r) = &d.detail else { continue };
                rand.set_region_seed(lower_bits, r.region_x, r.region_z, s.salt(), version);
                if rand.next_int_bound(s.get_offset()) % 4 != r.offset_x % 4 || rand.next_int_bound(s.get_offset()) % 4 != r.offset_z % 4 {
                    continue 'lower;
                }
            }
            lower_bits_found.push(lower_bits);
        }

        let found: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
        let threads = self.threads();
        for lower_bits in lower_bits_found {
            if self.should_terminate() {
                break;
            }
            let total = 1i64 << (48 - 19);
            let step = (total + threads as i64 - 1) / threads as i64;
            std::thread::scope(|scope| {
                for t in 0..threads as i64 {
                    let (from, to) = (t * step, ((t + 1) * step).min(total));
                    let (cache, found) = (&cache, &found);
                    scope.spawn(move || {
                        let mut rand = ChunkRand::new();
                        for upper_bits in from..to {
                            let seed = (upper_bits << 19) | lower_bits;
                            if cache.iter().all(|d| d.test_start(seed, &mut rand)) {
                                lock(found).insert(seed);
                            }
                        }
                    });
                }
            });
        }
        let seeds = found.into_inner().unwrap_or_else(|e| e.into_inner());
        let non_empty = !seeds.is_empty();
        lock(&self.state).structure_seeds = seeds;
        if non_empty {
            host.warn("tmachine.structureSeedSearchFinished", &[]);
        } else {
            host.error("finishedSearchNoResult");
        }
        non_empty
    }

    fn poke_structures(&self, data: &DataStorage) -> bool {
        let host = &data.host;
        let pillar_seeds = {
            let s = lock(&self.state);
            match &s.pillar_seeds {
                Some(p) if s.structure_seeds.is_empty() => p.clone(),
                _ => return false,
            }
        };
        if data.get_base_bits() < data.get_wanted_bits() {
            return false;
        }
        let cache = Self::search_cache(data);
        let inverse_lcg = LCG::JAVA.combine_steps(-2);
        let threads = self.threads();

        for pillar_seed in pillar_seeds {
            host.debug("====================================");
            host.warn("tmachine.lookingForStructureSeeds", &[pillar_seed.to_string()]);

            let progress = Mutex::new(ProgressListener::new());
            let total = 1i64 << 32;
            let step = (total + threads as i64 - 1) / threads as i64;
            std::thread::scope(|scope| {
                for t in 0..threads as i64 {
                    let (lower, upper) = (t * step, ((t + 1) * step).min(total));
                    let (cache, progress, inverse_lcg) = (&cache, &progress, &inverse_lcg);
                    scope.spawn(move || {
                        let mut rand = ChunkRand::new();
                        // The percent of the whole one block of 2^27 is, at the 4 threads of the Java.
                        let percent = 100.0f32 / (total as f32 / (1u64 << 27) as f32);
                        let mut partial = lower;
                        while partial < upper && !self.should_terminate() {
                            if (partial & ((1 << 27) - 1)) == 0 {
                                lock(progress).add_percent(percent, true, host.as_ref());
                            }
                            let seed = Self::time_machine(inverse_lcg, partial, pillar_seed);
                            if cache.iter().all(|d| d.test_start(seed, &mut rand)) {
                                self.add_structure_seed(seed);
                                host.print_seed("foundStructureSeed", seed);
                            }
                            partial += 1;
                        }
                    });
                }
            });
            if self.should_terminate() {
                return false;
            }
            lock(&progress).add_percent(0.0, true, host.as_ref());
        }

        if !lock(&self.state).structure_seeds.is_empty() {
            host.warn("tmachine.structureSeedSearchFinished", &[]);
        } else {
            host.error("finishedSearchNoResult");
        }
        true
    }

    fn poke_biomes(&self, data: &DataStorage) -> bool {
        let host = &data.host;
        {
            let s = lock(&self.state);
            if s.structure_seeds.is_empty() || s.world_seeds.len() == 1 || s.structure_seeds.len() > 1000 {
                return false;
            }
        }
        host.debug("====================================");

        lock(&self.state).world_seeds.clear();
        lock(&data.base_seed_data).dump();
        let version = host.version();
        let structure_seeds: Vec<i64> = self.structure_seeds().into_iter().collect();

        if version.is_newer_or_equal_to(MCVersion::v1_18) && data.get_decorator_bits() > 32.0 {
            host.warn("tmachine.decoratorWorldSeedSearch", &[]);
            let mut rand = WorldgenRandom::new(0);
            let decorators: Vec<FeatureData> = lock(&data.base_seed_data)
                .get_base_set()
                .iter()
                .filter(|e| matches!(&*e.data.feature, Feature::Decorator(d) if d.is_seedcracker_decorator()))
                .map(|e| e.data.clone())
                .collect();
            for &structure_seed in &structure_seeds {
                let mut upper_bits = 0i64;
                while upper_bits < 1 << 16 && !self.should_terminate() {
                    let world_seed = (upper_bits << 48) | structure_seed;
                    if decorators.iter().all(|d| d.test_start_world(world_seed, &mut rand)) {
                        self.add_world_seed_with_spam_protection(host.as_ref(), world_seed);
                    }
                    upper_bits += 1;
                }
                if self.should_terminate() {
                    return false;
                }
            }
            if self.world_seed_count() > 0 {
                host.warn("tmachine.worldSeedSearchFinished", &[]);
                return true;
            } else {
                host.warn("finishedSearchNoResult", &[]);
            }
        }

        let hashed = *lock(&data.hashed_seed_data);
        if let Some(h) = hashed.filter(|h| h.get_hashed_seed() != 0) {
            host.warn("tmachine.hashedSeedWorldSeedSearch", &[]);
            for &structure_seed in &structure_seeds {
                for world_seed in world_seed::from_hash(structure_seed, h.get_hashed_seed()) {
                    lock(&self.state).world_seeds.insert(world_seed);
                    host.print_seed("tmachine.foundWorldSeed", world_seed);
                }
                if self.should_terminate() {
                    return false;
                }
            }
            if self.world_seed_count() > 0 {
                host.warn("tmachine.worldSeedSearchFinished", &[]);
                return true;
            } else {
                *lock(&data.hashed_seed_data) = None;
                host.error("tmachine.noResultsRevertingToBiomes");
            }
        }

        lock(&data.biome_seed_data).dump();
        if data.not_enough_biome_data() {
            host.error("tmachine.moreBiomesNeeded");
            return false;
        }
        let biomes: Vec<_> = lock(&data.biome_seed_data).iter().map(|e| e.data).collect();
        let Some(factory) = host.biome_source_factory() else {
            host.error("tmachine.moreBiomesNeeded");
            return false;
        };

        host.warn("tmachine.biomeWorldSeedSearch", &[biomes.len().to_string()]);
        host.warn("tmachine.fuzzyBiomeSearch", &[]);
        for &structure_seed in &structure_seeds {
            for world_seed in structure_seed::to_random_world_seeds(structure_seed) {
                let Some(source) = factory(version, world_seed) else { continue };
                if biomes.iter().all(|b| b.test(version, source.as_ref())) {
                    lock(&self.state).world_seeds.insert(world_seed);
                    host.print_seed("tmachine.foundWorldSeed", world_seed);
                }
                if self.should_terminate() {
                    return false;
                }
            }
        }

        if self.world_seed_count() > 0 {
            return true;
        }
        if structure_seeds.len() > 10 {
            return false;
        }
        host.warn("tmachine.deepBiomeSearch", &[]);
        for &structure_seed in &structure_seeds {
            let mut upper_bits = 0i64;
            while upper_bits < 1 << 16 && !self.should_terminate() {
                let world_seed = (upper_bits << 48) | structure_seed;
                if let Some(source) = factory(version, world_seed) {
                    if biomes.iter().all(|b| b.test(version, source.as_ref())) {
                        self.add_world_seed_with_spam_protection(host.as_ref(), world_seed);
                    }
                }
                upper_bits += 1;
            }
            if self.should_terminate() {
                return false;
            }
        }

        if self.world_seed_count() > 0 {
            host.warn("tmachine.worldSeedSearchFinished", &[]);
        } else {
            host.error("finishedSearchNoResult");
        }
        if self.world_seed_count() > 0 {
            return true;
        }

        host.error("tmachine.deleteBiomeInformation");
        lock(&data.biome_seed_data).get_base_set().clear();

        host.warn("tmachine.randomSeedSearch", &[]);
        for &structure_seed in &structure_seeds {
            for s in structure_seed::to_random_world_seeds(structure_seed) {
                host.print_seed("tmachine.foundWorldSeed", s);
            }
        }
        true
    }

    /// A found world seed: the first nine are printed, then the rest only go to the log.
    fn add_world_seed_with_spam_protection(&self, host: &dyn Host, world_seed: i64) {
        let mut s = lock(&self.state);
        s.world_seeds.insert(world_seed);
        let n = s.world_seeds.len();
        drop(s);
        if n < 10 {
            host.print_seed("tmachine.foundWorldSeed", world_seed);
            if n == 9 {
                host.warn("tmachine.printSeedsInConsole", &[]);
            }
        } else {
            host.debug(&format!("Found world seed {world_seed}"));
        }
    }

    fn poke_structure_reduce(&self, data: &DataStorage) -> bool {
        let host = &data.host;
        if self.should_terminate() {
            return false;
        }
        let version = host.version();
        {
            let s = lock(&self.state);
            if !s.world_seeds.is_empty() || s.structure_seeds.len() < 2 {
                return false;
            }
        }
        if version.is_older_than(MCVersion::v1_13) {
            return false;
        }
        let structure_seeds: Vec<i64> = self.structure_seeds().into_iter().collect();
        let mut result: HashSet<i64> = HashSet::new();
        host.debug("====================================");
        host.warn("tmachine.reduceSeeds", &[structure_seeds.len().to_string()]);

        if let Some(pillars) = self.pillar_seeds() {
            for &seed in &structure_seeds {
                if pillars.contains(&(pillar_seed::from_structure_seed(seed) as i32)) {
                    result.insert(seed);
                }
            }
        }

        if result.len() != 1 {
            {
                let mut base = lock(&data.base_seed_data);
                base.dump();
                base.get_base_set().retain(|e| e.data.feature.version() == version);
            }
            let cache = Self::search_cache(data);
            let mut rand = ChunkRand::new();
            for &seed in &structure_seeds {
                if cache.iter().all(|d| d.test_start(seed, &mut rand)) {
                    result.insert(seed);
                }
            }
        }

        if !result.is_empty() && structure_seeds.len() > result.len() {
            if result.len() < 10 {
                for &seed in &result {
                    host.print_seed("foundStructureSeed", seed);
                }
            } else {
                host.warn("tmachine.succeedReducing", &[result.len().to_string()]);
            }
            lock(&self.state).structure_seeds = result;
            return true;
        }
        host.warn("tmachine.failedReducing", &[]);
        false
    }
}

impl Default for TimeMachine {
    fn default() -> Self {
        Self::new()
    }
}
