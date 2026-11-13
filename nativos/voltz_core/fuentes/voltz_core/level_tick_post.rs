// After a level's tick: place the ores the new chunks are owed.
fn voltz_level_tick_post(game: &mut crate::game::Game<'_>, dimension: &str) {
    crate::game::voltz_glue::level_tick_post(game, dimension);
}
