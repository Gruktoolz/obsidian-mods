fn freecam_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::freecam_yannis as m;

    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    let free = game.native_mod_key(m::ID, "free", "key.keyboard.f4");
    let follow = game.native_mod_key(m::ID, "follow", "key.keyboard.f6");
    let Some(player) = game.native_player() else {
        return;
    };
    let initial = m::Pose {
        position: [player.x, player.y + 1.62, player.z],
        yaw: player.yaw,
        pitch: player.pitch,
    };
    m::STATE.with(|cell| {
        let mut s = cell.borrow_mut();
        s.world(Some(player.dimension));
        if s.settings.is_none() {
            s.settings = Some(m::Settings::load(|k| game.native_setting(m::ID, k)));
        }
        if key == free {
            if pressed {
                s.free_key = game.native_playing();
                s.chord = false;
            } else {
                if s.free_key && !s.chord && game.native_playing() {
                    s.toggle_free(initial);
                }
                s.free_key = false;
            }
        } else if pressed && game.native_playing() {
            if key == follow {
                s.toggle_follow(initial);
            } else if s.free_key {
                if let Some(slot) = key
                    .strip_prefix("key.keyboard.")
                    .and_then(|n| n.parse::<usize>().ok())
                    .filter(|n| (1..=9).contains(n))
                {
                    s.tripod(slot - 1, initial);
                }
            }
        }
    });
}
