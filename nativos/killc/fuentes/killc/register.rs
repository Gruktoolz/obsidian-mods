// KillC's commands, none when config/killc.json disables it.
fn killc_register(commands: &mut Vec<crate::game::native_server::NativeCommand>) {
    use crate::game::{killc as m, native_server::NativeCommand};
    let dir = crate::game::Game::native_config_dir().ok();
    if !m::start(dir.as_deref()).enabled {
        return;
    }
    commands.push(NativeCommand::new("suicide"));
    commands.push(NativeCommand::new("killc"));
    commands.push(NativeCommand::op("killc <target:player>", 2));
}
