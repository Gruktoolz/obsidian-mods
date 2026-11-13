fn elytratrails_lines(
    game: &crate::game::Game<'_>,
    alpha: f64,
    lines: &mut Vec<horizon_client::gizmos::Line>,
) {
    game.elytratrails_draw(alpha as f32, lines);
}
