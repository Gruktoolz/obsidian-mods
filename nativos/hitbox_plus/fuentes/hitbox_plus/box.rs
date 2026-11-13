fn hitbox_plus_entity_box(
    gizmos: &mut crate::gizmos::Gizmos,
    min: [f64; 3],
    max: [f64; 3],
    position: [f64; 3],
    eye_height: f32,
    living: bool,
    view: (f32, f32),
) -> core::ops::ControlFlow<()> {
    use crate::official_hitbox_plus as m;
    use core::ops::ControlFlow::{Break, Continue};

    if !m::is_enabled() {
        return Continue(());
    }

    let stroke = m::current_thickness();

    // Green when crosshair aims directly at this entity's hitbox in weapon attack reach
    let color = if m::is_crosshair_on_target(min, max) {
        m::IN_REACH_COLOR
    } else {
        m::current_color()
    };

    // Draw the main bounding box
    gizmos.cuboid(min, max, color, stroke);

    // Red eye-level plane for living entities (players, mobs)
    if living {
        let eye = min[1] + f64::from(eye_height);
        let d = 0.009999999776482582;
        gizmos.cuboid(
            [min[0], eye - d, min[2]],
            [max[0], eye + d, max[2]],
            crate::gizmos::RED,
            stroke,
        );
    }

    // Blue look direction arrow
    if m::show_eye_ray() {
        let eye = [
            position[0],
            position[1] + f64::from(eye_height),
            position[2],
        ];
        let v = crate::gizmos::view_vector(view.0, view.1);
        let end = [
            eye[0] + v[0] * 2.0,
            eye[1] + v[1] * 2.0,
            eye[2] + v[2] * 2.0,
        ];
        gizmos.arrow(eye, end, crate::gizmos::BLUE, stroke);
    }

    Break(())
}
