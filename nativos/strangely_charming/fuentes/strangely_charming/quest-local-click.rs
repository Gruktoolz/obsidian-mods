fn hook(game: &mut crate::game::Game<'_>, click: &Option<Click>) {
    if game.remote.is_none()
        && let Some(Click::Custom { id, .. }) = click
        && let Some(action) = id.strip_prefix("strangely_charming:quest/")
    {
        game.sc_quest_action(action);
    }
}
