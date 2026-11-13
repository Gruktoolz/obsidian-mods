fn collide(
    motion: &mut crate::motion::MotionWorld<'_, '_, '_>,
    movement: Vec3,
) -> core::ops::ControlFlow<Result<Vec3>> {
    core::ops::ControlFlow::Break(crate::motion::fence_perch::collide(motion, movement))
}
