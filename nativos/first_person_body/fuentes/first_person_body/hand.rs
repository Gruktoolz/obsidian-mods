// With the model's arms, the vanilla first-person hand is not drawn (the
// model's arms and held items show instead), while the body shows.
fn first_person_body_hand(game: &crate::game::Game<'_>, canceled: &mut bool) {
    use horizon_client::mobs::first_person_body as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::current();
    if settings.get(m::o::ENABLED) == 1 && settings.get(m::o::ARMS) == 1 && game.own_body_in_view() {
        *canceled = true;
    }
}
