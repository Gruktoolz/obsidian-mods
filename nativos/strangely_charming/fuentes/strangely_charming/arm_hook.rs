fn hook(
    game: &crate::game::Game<'_>,
    left: bool,
    m: glam::Mat4,
    light: u32,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) {
    if let Some(player) = game.player_model.as_ref() {
        if let Some(arm) = game.res.player_arm(player.slim, left).as_ref() {
            let atlas = |name: &str| {
                game.gui
                    .atlas
                    .sprites
                    .get(name)
                    .map(|r| [r.u0, r.v0, r.u1, r.v1])
            };
            horizon_client::mobs::strangely_charming_geometry::arm(
                arm, player, left, m, &atlas, light, out,
            );
        }
    }
}
