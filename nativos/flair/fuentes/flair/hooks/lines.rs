fn flair_lines(game: &crate::game::Game<'_>, alpha: f64, size: [u32; 2]) -> core::ops::ControlFlow<Vec<Line>> {
    crate::game::umbx_flair_lines::lines(game, alpha, size)
}
