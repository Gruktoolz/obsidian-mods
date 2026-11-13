// The settings page: one ON/OFF button per toggle.
fn armorhud_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::armorhud as m;
    if mod_id != m::ID {
        return;
    }
    for (key, label, default) in m::TOGGLES {
        let on = m::enabled(game.native_setting(m::ID, key).as_deref(), default);
        buttons.push(m::button_label(label, on));
    }
}
