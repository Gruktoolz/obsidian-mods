fn hook(game: &mut crate::game::Game<'_>, entity: i32, canceled: &mut bool) {
    if !*canceled && horizon_mods::evanders_config::enabled("protect_pets") {
        let owner = game.acting_player_uuid();
        *canceled = game
            .mobs
            .iter()
            .any(|m| m.id == entity as u32 && m.extra.tame && m.extra.owner == Some(owner));
    }
}
