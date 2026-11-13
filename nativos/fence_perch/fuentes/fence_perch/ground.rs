fn ground(
    motion: &mut crate::motion::MotionWorld<'_, '_, '_>,
    below: bool,
    horizontal: bool,
    movement: Vec3,
) -> core::ops::ControlFlow<Result<()>> {
    core::ops::ControlFlow::Break(crate::motion::fence_perch::ground(
        motion, below, horizontal, movement,
    ))
}
