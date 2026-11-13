// Tectonic's NoisesMixin uses vanilla seed names for its climate parameter noises.
fn seeds(name: &str) -> std::ops::ControlFlow<[u64; 2]> {
    if let Some(path) = name.strip_prefix("tectonic:parameter/") {
        return std::ops::ControlFlow::Break(seed_from_hash(&format!("minecraft:{path}")));
    }
    std::ops::ControlFlow::Continue(())
}
