// The mod's commands in every player's command tree.
fn wayfarer_stones_register_commands(commands: &mut Vec<horizon_client::mod_api::NativeCommand>) {
    crate::game::wayfarer_stones_glue::commands(commands);
}
