fn hook(g: &Guest, pos: [i32; 3]) -> core::ops::ControlFlow<bool> {
    use crate::game::strangely_charming::{PORTABLE_TABLE, carries_table};
    if pos == PORTABLE_TABLE {
        return core::ops::ControlFlow::Break(
            horizon_mods::evanders_config::enabled("portable_crafting") && carries_table(g),
        );
    }
    core::ops::ControlFlow::Continue(())
}
