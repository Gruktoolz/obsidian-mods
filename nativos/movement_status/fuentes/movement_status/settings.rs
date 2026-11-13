fn movement_status_page( game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>, ) {
    use crate::game::mod_movement_status as m;
    if mod_id != m::ID { return; } buttons.push(m::background_label(m::background(|| { game.native_setting(m::ID, "background") })));
}
