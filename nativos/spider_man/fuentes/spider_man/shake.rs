// The mod's camera shake, added to the other mods'.
fn plug_shake(game: &crate::game::Game<'_>, shake: &mut crate::game::native_hooks::NativeShake) {
    crate::game::spider_man::camera_shake(game, shake)
}
