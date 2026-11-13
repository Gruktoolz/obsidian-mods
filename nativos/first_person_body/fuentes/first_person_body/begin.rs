// `player_model_geometry` starts: in first person, the hooks below it act
// until it returns (`end.rs`), with the body moved back from the view.
fn first_person_body_begin(game: &crate::game::Game<'_>) {
    use horizon_client::mobs::first_person_body as m;
    let first_person = game.camera_type.first_person() && game.sleeping.is_none();
    if !first_person || !game.native_on(m::ID) {
        m::set_active(None);
        return;
    }
    let settings = m::current();
    let crouching = game.player_model.as_ref().is_some_and(|p| p.extra.crouching);
    m::set_active(Some(m::body_offset(&settings, game.yaw, game.pitch, crouching)));
}
