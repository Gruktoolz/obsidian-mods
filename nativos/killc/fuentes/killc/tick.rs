// End of each server tick: queued kills, then random respawns in the Overworld.
fn killc_tick(
    game: &mut crate::game::Game<'_>,
    phase: crate::game::native_lifecycle::ServerTickPhase,
) {
    use crate::game::{killc as m, native_lifecycle::ServerTickPhase};
    if !matches!(phase, ServerTickPhase::ServerTickEnd { .. }) {
        return;
    }
    for line in m::take_queue() {
        game.native_run_command(&line);
    }
    let config = m::config();
    if !config.enabled || !config.random_spawn {
        return;
    }
    for p in game.native_server_players() {
        // Beds are not told apart from the world spawn, as in the Fabric build.
        if p.health > 0.0 && m::respawned(p.uuid) && p.dimension == "minecraft:overworld" {
            // spreadplayers finds the surface and loads the chunk.
            game.native_run_command(&format!("spreadplayers 0 0 0 {} false {}", config.radius, p.name));
        }
    }
}
