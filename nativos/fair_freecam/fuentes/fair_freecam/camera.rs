// While the camera flies free, the world is drawn from it instead of the
// player's eyes (or the third-person camera).
fn freecam_camera(
    game: &mut crate::game::Game<'_>,
) -> core::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    use crate::game::fair_freecam as fc;
    use core::ops::ControlFlow;
    use winit::keyboard::KeyCode as K;
    if !fc::active() {
        return ControlFlow::Continue(());
    }
    // Back to the player when it dies, leaves the world, or the mod is turned off.
    let alive = game
        .player
        .as_ref()
        .is_some_and(|p| !p.health.dead_or_dying());
    if !alive || game.screen == crate::game::Screen::Dead || !game.native_on(fc::ID) {
        fc::stop();
        return ControlFlow::Continue(());
    }
    let playing = game.native_playing();
    let held = |k: K| playing && game.keys.contains(&k);
    let input = fc::Input {
        forward: held(K::KeyW),
        back: held(K::KeyS),
        left: held(K::KeyA),
        right: held(K::KeyD),
        up: held(K::Space),
        down: held(K::ShiftLeft),
        fast: held(K::ControlLeft),
    };
    let settings = {
        let game = &*game;
        fc::settings(|name| game.native_setting(fc::ID, name))
    };
    // Fair play: the camera is a small box that Horizon's own collision
    // (`DismountHelper.canDismountTo`: blocks' real shapes and the world
    // border) keeps out of blocks.
    let free = |at: [f64; 3]| {
        use horizon_core::geometry::{Aabb, Vec3};
        let r = fc::RADIUS;
        game.can_dismount_to(Aabb::new(
            Vec3::new(at[0] - r, at[1] - r, at[2] - r),
            Vec3::new(at[0] + r, at[1] + r, at[2] + r),
        ))
    };
    match fc::step(input, settings, free) {
        Some((at, yaw, pitch)) => {
            ControlFlow::Break(Some((glam::DVec3::from_array(at), yaw, pitch)))
        }
        None => ControlFlow::Continue(()),
    }
}
