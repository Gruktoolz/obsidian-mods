fn create_burner_predict_entity(
    game: &mut crate::game::Game<'_>,
    hand: crate::game::Hand,
) -> std::ops::ControlFlow<bool> {
    if game.create_burner_predict_entity(hand) {
        std::ops::ControlFlow::Break(true)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
