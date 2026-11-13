fn hitbox_plus_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut HudBatch,
    corners: &mut Corners,
) {
    use horizon_client::official_hitbox_plus as m;
    if !m::show_sprint_display() {
        return;
    }
    let Some(player) = game.player.as_ref() else {
        return;
    };

    let text = if player.sprinting {
        if game.settings.toggle_sprint {
            Some("[Sprinting (Toggled)]")
        } else {
            Some("[Sprinting (Key)]")
        }
    } else if player.swimming {
        Some("[Swimming]")
    } else if player.local_crouching {
        if game.settings.toggle_crouch {
            Some("[Sneaking (Toggled)]")
        } else {
            Some("[Sneaking (Key)]")
        }
    } else {
        None
    };

    if let Some(msg) = text {
        let corner = match m::sprint_corner() {
            1 => Corner::TopRight,
            2 => Corner::BottomLeft,
            3 => Corner::BottomRight,
            _ => Corner::TopLeft,
        };
        let (x, y) = corners.place(corner, 115.0, 10.0);
        game.native_text(batch, msg, [x, y], 0xFFFFFF, true);
    }
}
