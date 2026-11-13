// kaptainwutax.seedcrackerX.cracker.storage.DataStorage: what the finders have found, the order it is kept in, the bits it
// adds up to, and the queue of what runs when more arrives. The game's part of `tick` (the config screen opening, the
// BlockUpdateQueue) is the game's; `tick` here is the part that hands the queue to the TimeMachine.
use super::scx_cracker_data::{BiomeEntry, DataAddedEvent, FeatureEntry, HashedSeedData, Host, PillarData, ScheduledSet};
use super::scx_cracker_time_machine::TimeMachine;
use super::scx_feature::{DecoratorClass, Feature, FeatureData, RegionShape, StructureClass};
use super::scx_version::MCVersion;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex, MutexGuard};

fn log2(x: f64) -> f64 {
    x.ln() / std::f64::consts::LN_2
}

/// `DataStorage.getBits(feature, decorators18)`.
pub fn get_bits(feature: &Feature, decorators18: bool) -> f64 {
    match feature {
        Feature::Structure(s) => match s.class.shape() {
            RegionShape::Uniform => log2((s.get_offset() * s.get_offset()) as f64),
            RegionShape::Triangular => log2((s.get_peak() * s.get_peak()) as f64),
            RegionShape::Treasure => log2(100.0),
        },
        Feature::Decorator(d) => {
            if !decorators18 && d.is_seedcracker_decorator() && d.version.is_newer_than(MCVersion::v1_17_1) {
                return 0.0;
            }
            match d.class {
                DecoratorClass::DesertWell => log2((1000 * 16 * 16) as f64),
                DecoratorClass::Dungeon => log2(256.0 * 16.0 * 16.0 * 0.125),
                DecoratorClass::DeepDungeon => log2(58.0 * 16.0 * 16.0 * 0.25),
                DecoratorClass::EmeraldOre => log2(28.0 * 16.0 * 16.0 * 0.5),
                DecoratorClass::EndGateway => log2((700 * 16 * 16 * 7) as f64),
            }
        }
    }
}

/// `SEED_DATA_COMPARATOR`: structures before decorators, the ones with most bits first, a tie after what it ties with.
pub fn seed_data_comparator(s1: &FeatureEntry, s2: &FeatureEntry) -> Ordering {
    let is_structure1 = s1.data.feature.is_structure();
    let is_structure2 = s2.data.feature.is_structure();
    // Structures always come before decorators.
    if is_structure1 != is_structure2 {
        return if is_structure2 { Ordering::Greater } else { Ordering::Less };
    }
    if feature_entry_equals(s1, s2) {
        return Ordering::Equal;
    }
    let diff = get_bits(&s2.data.feature, false) - get_bits(&s1.data.feature, false);
    if diff == 0.0 {
        Ordering::Greater
    } else if diff > 0.0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

/// `Entry.equals` for feature data: one feature object, one chunk.
pub fn feature_entry_equals(a: &FeatureEntry, b: &FeatureEntry) -> bool {
    a.data.same_place(&b.data)
}

fn biome_entry_equals(a: &BiomeEntry, b: &BiomeEntry) -> bool {
    a.data.biome == b.data.biome
}

pub struct DataStorage {
    pub host: Arc<dyn Host>,
    pub base_seed_data: Mutex<ScheduledSet<FeatureEntry>>,
    pub biome_seed_data: Mutex<ScheduledSet<BiomeEntry>>,
    pub hashed_seed_data: Mutex<Option<HashedSeedData>>,
    pub pillar_data: Mutex<Option<PillarData>>,
    scheduled_data: Mutex<Vec<DataAddedEvent>>,
    time_machine: Mutex<Arc<TimeMachine>>,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl DataStorage {
    pub fn new(host: Arc<dyn Host>) -> Arc<DataStorage> {
        Arc::new(DataStorage {
            host,
            base_seed_data: Mutex::new(ScheduledSet::new(Some(seed_data_comparator), feature_entry_equals)),
            biome_seed_data: Mutex::new(ScheduledSet::new(None, biome_entry_equals)),
            hashed_seed_data: Mutex::new(None),
            pillar_data: Mutex::new(None),
            scheduled_data: Mutex::new(Vec::new()),
            time_machine: Mutex::new(Arc::new(TimeMachine::new())),
        })
    }

    pub fn get_time_machine(&self) -> Arc<TimeMachine> {
        lock(&self.time_machine).clone()
    }

    /// `tick`, the part that is the storage's: when the TimeMachine is idle, take in what was added and run what is queued,
    /// on another thread. Returns that thread, for a caller that wants to wait for it.
    pub fn tick(self: &Arc<Self>) -> Option<std::thread::JoinHandle<()>> {
        let tm = self.get_time_machine();
        if tm.is_running() {
            return None;
        }
        lock(&self.base_seed_data).dump();
        lock(&self.biome_seed_data).dump();
        tm.set_running(true);
        let this = self.clone();
        Some(std::thread::spawn(move || {
            let events = std::mem::take(&mut *lock(&this.scheduled_data));
            for event in events {
                event(&this);
            }
            tm.set_running(false);
        }))
    }

    /// `addPillarData`: the first one only.
    pub fn add_pillar_data(&self, data: PillarData, event: DataAddedEvent) -> bool {
        let mut slot = lock(&self.pillar_data);
        let is_added = slot.is_none();
        if is_added {
            *slot = Some(data);
            drop(slot);
            self.schedule(event);
        }
        is_added
    }

    /// `addBaseData`.
    pub fn add_base_data(&self, data: FeatureData, event: DataAddedEvent) -> bool {
        let entry = FeatureEntry { data, event: event.clone() };
        {
            let mut set = lock(&self.base_seed_data);
            if set.contains(&entry) {
                return false;
            }
            set.schedule_add(entry);
        }
        self.schedule(event);
        true
    }

    /// `addBiomeData`.
    pub fn add_biome_data(&self, data: super::scx_cracker_data::BiomeData, event: DataAddedEvent) -> bool {
        let entry = BiomeEntry { data, event: event.clone() };
        {
            let mut set = lock(&self.biome_seed_data);
            if set.contains(&entry) {
                return false;
            }
            set.schedule_add(entry);
        }
        self.schedule(event);
        true
    }

    /// `addHashedSeedData`: a new hash replaces the old one and is searched at once.
    pub fn add_hashed_seed_data(&self, data: HashedSeedData, event: DataAddedEvent) -> bool {
        let mut slot = lock(&self.hashed_seed_data);
        if slot.is_none_or(|h| h.get_hashed_seed() != data.get_hashed_seed()) {
            *slot = Some(data);
            drop(slot);
            self.schedule(event);
            return true;
        }
        false
    }

    pub fn schedule(&self, event: DataAddedEvent) {
        lock(&self.scheduled_data).push(event);
    }

    pub fn get_base_bits(&self) -> f64 {
        let set = lock(&self.base_seed_data);
        let mut bits = 0.0;
        for e in set.iter() {
            if !matches!(&*e.data.feature, Feature::Structure(s) if s.class == StructureClass::PillagerOutpost) {
                bits += get_bits(&e.data.feature, false);
            }
        }
        bits
    }

    pub fn get_lifting_bits(&self) -> f64 {
        let set = lock(&self.base_seed_data);
        let mut bits = 0.0;
        for e in set.iter() {
            if let Feature::Structure(s) = &*e.data.feature {
                if s.class.is_old_structure() || s.class == StructureClass::Shipwreck {
                    bits += log2((s.get_offset() * s.get_offset()) as f64);
                }
            }
        }
        bits
    }

    pub fn get_decorator_bits(&self) -> f64 {
        let set = lock(&self.base_seed_data);
        let mut bits = 0.0;
        for e in set.iter() {
            if matches!(&*e.data.feature, Feature::Decorator(d) if d.is_seedcracker_decorator()) {
                bits += get_bits(&e.data.feature, true);
            }
        }
        bits
    }

    pub fn get_wanted_bits(&self) -> f64 {
        32.0
    }

    pub fn not_enough_biome_data(&self) -> bool {
        lock(&self.biome_seed_data).size() < 7
    }

    /// `clear`: forget everything, and stop the search that is running.
    pub fn clear(&self) {
        lock(&self.scheduled_data).clear();
        *lock(&self.pillar_data) = None;
        *lock(&self.base_seed_data) = ScheduledSet::new(Some(seed_data_comparator), feature_entry_equals);
        *lock(&self.biome_seed_data) = ScheduledSet::new(None, biome_entry_equals);
        let mut tm = lock(&self.time_machine);
        tm.terminate();
        *tm = Arc::new(TimeMachine::new());
    }

    /// The base data as the search reads it: the data themselves, in order.
    pub fn base_data(&self) -> Vec<FeatureData> {
        lock(&self.base_seed_data).iter().map(|e| e.data.clone()).collect()
    }
}
