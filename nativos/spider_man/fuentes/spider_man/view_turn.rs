// The wall camera's turn of the whole view, kept where Horizon's view matrix is
// built (the render crate): set by the game's camera each frame for the yaw and
// pitch it renders from, read by `view_matrix` (`view.rs`).

use std::sync::Mutex;

/// The yaw and pitch the frame renders from, and the turned orientation
/// (x, y, z, w: camera axes to world) to show for them.
static TURN: Mutex<Option<(f32, f32, [f32; 4])>> = Mutex::new(None);

pub fn set(turn: Option<(f32, f32, [f32; 4])>) {
    *TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = turn;
}

/// The turned view for exactly the frame's angles; any other view the game
/// builds (previews, other cameras) stays its own.
pub fn view(yaw: f32, pitch: f32) -> Option<glam::Mat4> {
    let turn = *TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let (y, p, q) = turn?;
    (y.to_bits() == yaw.to_bits() && p.to_bits() == pitch.to_bits())
        .then(|| glam::Mat4::from_quat(glam::Quat::from_array(q).conjugate()))
}
