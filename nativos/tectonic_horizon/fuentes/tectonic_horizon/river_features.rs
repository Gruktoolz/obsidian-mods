fn river_features(
    data: &Arc<GeneratorData>,
    seed: i64,
    worker: &WorkerState,
    region: &mut [ProtoChunk; 9],
) {
    if worker.structures_only {
        return;
    }
    if let Err(error) =
        crate::worldgen::generator::tectonic_decoration::decorate(data, seed, region)
    {
        panic!("[Tectonic-Horizon] river decoration failed: {error}");
    }
}
