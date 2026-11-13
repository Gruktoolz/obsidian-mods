fn hitbox_plus_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    use horizon_client::official_hitbox_plus as m;
    if mod_id != m::ID {
        return;
    }
    let _ = game;
    match button {
        0 => { m::toggle(); }
        1 => { m::cycle_style(); }
        2 => { m::cycle_thickness(); }
        3 => { m::toggle_eye_ray(); }
        4 => { m::toggle_sprint_display(); }
        5 => { m::cycle_sprint_corner(); }
        _ => {}
    }
}
