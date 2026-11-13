// The players this window knows of. The game's own list is private to its name
// tags, and these are the same two sources: a LAN host's guests, a server's players.
use super::Game;

pub(super) struct Player {
    pub id: u32,
    pub name: String,
    pub at: [f64; 3],
    pub alive: bool,
}

pub(super) fn others(game: &Game<'_>) -> Vec<Player> {
    let mut out = Vec::new();
    if let Some(host) = &game.lan {
        for (conn, m) in &host.models {
            if let Some(g) = host.hub.guest(*conn) {
                out.push(Player {
                    id: m.id,
                    name: g.name.clone(),
                    at: [m.position.x, m.position.y, m.position.z],
                    alive: m.alive(),
                });
            }
        }
    }
    for (id, name) in game.remote_player_names() {
        if let Some(m) = game.mobs.iter().find(|m| m.id == id) {
            out.push(Player {
                id,
                name,
                at: [m.position.x, m.position.y, m.position.z],
                alive: m.alive(),
            });
        }
    }
    out
}

/// The other player standing at `at`. A server's event is sent from the
/// player's model, so it matches exactly; a guest's own body can run ahead of
/// their model on the host. Anything else there is a mob holding a totem.
pub(super) fn at(game: &Game<'_>, at: [f64; 3]) -> Option<String> {
    let reach = if game.lan.is_some() { 9.0 } else { 0.01 };
    others(game)
        .into_iter()
        .map(|p| ((0..3).map(|a| (p.at[a] - at[a]).powi(2)).sum::<f64>(), p.name))
        .filter(|(d, _)| *d < reach)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, name)| name)
}
