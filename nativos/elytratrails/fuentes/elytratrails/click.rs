fn elytratrails_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::official_elytratrails as m;
    if mod_id != m::ID {
        return;
    }
    game.elytratrails_page_click(button);
}
