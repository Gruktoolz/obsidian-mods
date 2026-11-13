fn clear(
    motion: &mut crate::motion::MotionWorld<'_, '_, '_>,
    bounds: Aabb,
) -> core::ops::ControlFlow<Result<bool>> {
    core::ops::ControlFlow::Break(crate::motion::fence_perch::clear(motion, bounds))
}
