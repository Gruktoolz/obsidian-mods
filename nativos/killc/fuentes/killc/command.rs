// /suicide and /killc [player], the kill runs at the next server tick.
fn killc_command(
    game: &mut crate::game::Game<'_>,
    player: &crate::game::native_server::NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    use crate::game::killc as m;
    use horizon_client::text::{self, Text};
    if *ran {
        return;
    }
    let Some(asked) = m::parse_command(command) else {
        return;
    };
    *ran = true;
    match asked {
        Ok(answer) => {
            m::queue(format!("kill {}", player.name));
            game.native_feedback(Text::literal(answer));
        }
        // The server only checks the lowest usage's level, so check level 2 here.
        Err(_) if player.permission < 2 => {
            game.native_feedback(Text::literal("You don't have permission to kill other players.").with_color(text::RED));
        }
        Err(name) => {
            let target = game
                .native_server_players()
                .into_iter()
                .find(|p| p.name.eq_ignore_ascii_case(name));
            let Some(target) = target else {
                game.native_feedback(Text::literal("No player was found").with_color(text::RED));
                return;
            };
            m::queue(format!("kill {}", target.name));
            game.native_tell(target.uuid, Text::literal(format!("☠ You have been killed by {}!", player.name)));
            game.native_feedback(Text::literal(format!("🗡 You have killed {}!", target.name)));
        }
    }
}
