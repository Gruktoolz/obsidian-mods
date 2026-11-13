fn hook(game: &mut crate::game::Game<'_>, m: &Mob, by_player: bool) {
    if by_player
        && horizon_mods::evanders_config::enabled("raid_horns")
        && raider::state(m).is_some_and(|s| s.patrol_leader)
        && game.world.random.next_float() < 0.05
    {
        game.spawn_item(
            horizon_client::items::ItemStack::new("strangely_charming:raid_horn", 1),
            [m.position.x, m.position.y, m.position.z],
            super::dispenser::new_item_motion(),
            10,
        );
    }
}
