// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// `/chop` in every player's command tree.
fn chop_register_commands(commands: &mut Vec<crate::game::native_server::NativeCommand>) {
    crate::game::chop_server::register(commands);
}
