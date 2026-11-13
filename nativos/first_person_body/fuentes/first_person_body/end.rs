// `player_model_geometry` returns: the hooks below it stop acting.
fn first_person_body_end(value: ()) {
    horizon_client::mobs::first_person_body::set_active(None);
    value
}
