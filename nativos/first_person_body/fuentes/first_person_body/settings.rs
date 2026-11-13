// The mod's settings page: one button per option, then reset.
fn first_person_body_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client::mobs::first_person_body as m;
    if mod_id != m::ID {
        return;
    }
    let game = &*game;
    buttons.extend(m::settings(|key| game.native_setting(m::ID, key)).labels());
}
