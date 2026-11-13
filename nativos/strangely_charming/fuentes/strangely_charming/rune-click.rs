fn hook(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    *canceled |= game.sc_rune_at(pos) && !game.sc_has_hammer();
}
