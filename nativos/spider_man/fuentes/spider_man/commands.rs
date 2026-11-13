// The mod's commands, for the chat box's colours and completion.
fn plug_commands(game: &crate::game::Game<'_>, commands: &mut Vec<&'static str>) {
    crate::game::spider_man::commands(game, commands)
}
