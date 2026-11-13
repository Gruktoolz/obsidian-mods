fn create_on_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut crate::containers::ModBlockEntity,
) {
    crate::game::create_port::tick(game, pos, entity);
}
