// `own_body_in_view`: in first person, the body is drawn too, while the
// player stands or crouches (not swimming, crawling, gliding or spinning),
// isn't a spectator and, unless the settings say otherwise, isn't riding.
fn first_person_body_in_view(game: &crate::game::Game<'_>, value: bool) -> bool {
    use horizon_client::mobs::first_person_body as m;
    if value || !game.native_on(m::ID) {
        return value;
    }
    let settings = m::current();
    if settings.get(m::o::ENABLED) == 0 {
        return value;
    }
    let Some(model) = game.player_model.as_ref() else {
        return value;
    };
    let pose = model.extra.avatar_pose;
    let upright = matches!(
        pose,
        horizon_client::pose::Pose::Standing | horizon_client::pose::Pose::Crouching
    );
    let riding_ok = settings.get(m::o::RIDING) == 1 || !model.extra.passenger;
    upright && !model.extra.fall_flying && !model.extra.spectator && riding_ok
}
