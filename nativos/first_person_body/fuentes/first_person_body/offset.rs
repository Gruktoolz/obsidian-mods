// `base_transform`: while the own first-person model is drawn, it stands a
// little behind the camera, so the view doesn't start inside the chest.
fn first_person_body_offset(value: glam::Mat4) -> glam::Mat4 {
    match horizon_client::mobs::first_person_body::active() {
        Some(offset) => glam::Mat4::from_translation(glam::Vec3::from_array(offset)) * value,
        None => value,
    }
}
