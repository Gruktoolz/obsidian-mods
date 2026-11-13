// /chatheads: tell the mod your name (so "Show your own head" can hide your
// own lines), and say the settings.
fn chat_heads_rust_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::chat::native_chat_heads_rust as m;
    let mut parts = command.split_whitespace();
    if parts.next() != Some("chatheads") || !game.native_on(m::ID) {
        return;
    }
    *ran = true;
    match parts.next().unwrap_or("") {
        "me" => {
            let Some(name) = parts.next() else {
                game.native_chat("Use /chatheads me <your name>");
                return;
            };
            if crate::game::chat::native_chat_heads_rust::sender(&format!("<{name}> x")).is_none() {
                game.native_chat("That is not a player name: 1-16 of A-Z a-z 0-9 _");
                return;
            }
            game.native_set_setting(m::ID, "own_name", name);
            game.native_chat(&format!("Your lines are {}'s", name));
        }
        "" => game.native_chat("Chat Heads Rust: heads beside player names; /chatheads me <name> hides your own"),
        other => game.native_chat(&format!("Unknown option {other}: /chatheads me <name>")),
    }
}
