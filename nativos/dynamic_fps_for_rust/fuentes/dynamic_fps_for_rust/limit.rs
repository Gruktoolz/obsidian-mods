fn limit(value: i32, game: &crate::game::Game<'_>) -> i32 {
    crate::game::dynamic_fps_adapter::limit(game, value)
}
