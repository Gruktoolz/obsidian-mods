// The commands the chat box completes as the game's own.
fn scx_commands(commands: &mut Vec<&'static str>) {
    crate::game::scx_game::commands(commands);
}
