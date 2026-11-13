// Register Fly Speed's root command and top-level actions for chat autocomplete.
fn fly_speed_commands(
    game: &mut crate::game::Game<'_>,
    commands: &mut Vec<&'static str>,
) {
    use crate::game::fly_speed as m;
    if game.native_on(m::ID) {
        commands.extend(["flyspeed", "flyspeed-help", "flyspeed-reset"]);
    }
}
