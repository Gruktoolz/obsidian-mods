fn hook(game: &mut crate::game::Game<'_>, batch: &mut HudBatch, ox: f32, oy: f32) {
    if game.container.is_none()
        && game.craft_size == 2
        && !game.mount_active()
        && horizon_mods::evanders_config::enabled("scrollkeepers")
    {
        game.sc_inventory_badges(batch, ox, oy);
    }
}
