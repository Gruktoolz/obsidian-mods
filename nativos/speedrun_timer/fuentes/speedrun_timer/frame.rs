// Arms the timer in a world, starts it on the first movement after that, and
// stops it when the player leaves the End for the overworld.
fn speedrun_timer_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::speedrun_timer as m;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(m::ID) {
        return;
    }
    let now = std::time::Instant::now();
    let pos = game.native_player().map(|p| m::Pos {
        x: p.x,
        y: p.y,
        z: p.z,
        dimension: p.dimension,
    });
    match m::step_playing(pos, game.native_playing(), now) {
        m::Event::Nothing => {}
        m::Event::Started => game.native_chat("Speedrun timer started"),
        m::Event::Finished(nanos) => {
            // A new personal best is kept in the mod's folder with the others.
            let mut settings = {
                let game = &*game;
                m::settings(|key| game.native_setting(m::ID, key))
            };
            let millis = nanos / 1_000_000;
            let best = settings.best_ms == 0 || millis < settings.best_ms;
            if best {
                settings.best_ms = millis;
                m::set_settings(settings);
                for (key, value) in settings.stored() {
                    game.native_set_setting(m::ID, key, &value);
                }
            }
            let dir = game.native_data_dir(m::ID);
            let line = format!(
                "Run finished: {}{}",
                m::format(nanos),
                if best { " — personal best" } else { "" }
            );
            let said = match m::save_run(&dir, nanos) {
                Ok(path) => format!("{line} (saved to {})", path.display()),
                Err(e) => format!("{line} (not saved: {e})"),
            };
            game.native_chat(&said);
        }
    }
}
