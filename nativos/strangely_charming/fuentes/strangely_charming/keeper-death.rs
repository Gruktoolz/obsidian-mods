fn hook(game: &mut crate::game::Game<'_>, mob: &Mob) {
    if mob.species() == Species::Villager && mob.extra.variant2 == 17 {
        game.sc_keeper_died(mob.id);
    }
}
