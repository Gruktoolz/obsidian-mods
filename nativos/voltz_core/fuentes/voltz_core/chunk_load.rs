// A chunk came into the world: new overworld chunks get Voltz ores.
fn voltz_chunk_load(dimension: &str, chunk: [i32; 2], new: bool) {
    crate::game::voltz_glue::chunk_load(dimension, chunk, new);
}
