// /speedrun says where the timer is; start, stop and reset drive it by hand.
fn speedrun_timer_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::speedrun_timer as m;
    let mut parts = command.split_whitespace();
    if parts.next() != Some("speedrun") || !game.native_on(m::ID) {
        return;
    }
    *ran = true;
    let now = std::time::Instant::now();
    match parts.next().unwrap_or("") {
        "" => {
            let (phase, nanos) = m::reading(now);
            let line = match phase {
                m::Phase::Idle => "Speedrun timer: waiting for a world".to_owned(),
                m::Phase::Armed => "Speedrun timer: armed, move to start".to_owned(),
                m::Phase::Running => format!("Speedrun timer: running, {}", m::format(nanos)),
                m::Phase::Done => format!("Speedrun timer: finished, {}", m::format(nanos)),
            };
            game.native_chat(&line);
        }
        "start" => {
            m::force_start(now);
            game.native_chat("Speedrun timer started");
        }
        "stop" => match m::force_stop(now) {
            Some(nanos) => {
                let dir = game.native_data_dir(m::ID);
                let _ = m::save_run(&dir, nanos);
                game.native_chat(&format!("Speedrun timer stopped: {}", m::format(nanos)));
            }
            None => game.native_chat("Speedrun timer is not running"),
        },
        "reset" => {
            m::reset();
            game.native_chat("Speedrun timer reset");
        }
        other => game.native_chat(&format!(
            "Unknown option {other}: use /speedrun, /speedrun start, /speedrun stop, /speedrun reset"
        )),
    }
}
