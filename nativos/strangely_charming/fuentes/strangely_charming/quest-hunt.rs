fn hook(game: &mut crate::game::Game<'_>, mob: &Mob) {
    if mob.kind.id=="minecraft:zombie" && (!killed_by_mob(mob) || mob.last_damage.is_some_and(|(d,_)|
        matches!(d.attacker,Some(Who::Mob(id)) if game.mobs.iter().any(|m|m.id==id && m.species()==Species::Wolf && m.extra.tame && m.extra.owner==Some(game.acting_uuid()))))) {
        game.sc_event(crate::game::sc_quests::Kind::Zombies,1);
    }
}
