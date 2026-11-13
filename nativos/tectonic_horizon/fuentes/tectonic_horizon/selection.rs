// Upstream uses the live menu switch; forks with per-world flags use their inherent method.
pub trait TectonicWorldSelection {
    fn mod_enabled(&self, id: &str) -> bool;
}
impl TectonicWorldSelection for super::WorldgenOverrides {
    fn mod_enabled(&self, _id: &str) -> bool {
        super::tectonic_horizon::active()
    }
}
