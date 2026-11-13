fn elytratrails_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::official_elytratrails as m;
    if mod_id != m::ID {
        return;
    }
    buttons.extend(game.elytratrails_page_buttons());
}
