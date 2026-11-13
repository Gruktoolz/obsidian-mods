fn hook(
    game: &mut crate::game::Game<'_>,
    p: &Projectile,
    hit: Hit,
) -> core::ops::ControlFlow<Result<()>> {
    if let Hit::Entity {
        who: Who::Mob(id), ..
    } = hit
        && p.owner == Some(Who::Player)
        && horizon_mods::evanders_config::enabled("protect_pets")
    {
        let owner = p.owner_uuid.unwrap_or_else(|| game.acting_player_uuid());
        if game
            .mobs
            .iter()
            .any(|m| m.id == id && m.extra.tame && m.extra.owner == Some(owner))
        {
            return core::ops::ControlFlow::Break(Ok(()));
        }
    }
    core::ops::ControlFlow::Continue(())
}
