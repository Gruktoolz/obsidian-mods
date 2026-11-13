fn hook(value: Box<Menu>, game: &crate::game::Game<'_>, station: &Station) -> Box<Menu> {
    let mut menu = value;
    menu.evanders_set_table_saw(
        game.block_name(station.pos).as_ref() == "strangely_charming:woodcutter",
    );
    menu
}
