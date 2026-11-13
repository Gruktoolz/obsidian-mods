// The wall camera's whole orientation for the frame's angles, else the game's.
fn turned_view(yaw: f32, pitch: f32) -> std::ops::ControlFlow<glam::Mat4> {
    match crate::camera::spider_man_view::view(yaw, pitch) {
        Some(view) => std::ops::ControlFlow::Break(view),
        None => std::ops::ControlFlow::Continue(()),
    }
}
