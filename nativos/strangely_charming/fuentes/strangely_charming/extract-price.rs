fn hook(menu: &Menu, env: &Env, s: usize) -> core::ops::ControlFlow<bool> {
    use crate::stations::evanders_grinding as grinding;
    if menu.slots[s].role == Role::GrindResult
        && let Some((_, item, _)) = grinding::extraction(
            menu.containers[MAIN][0].as_ref(),
            menu.containers[MAIN][1].as_ref(),
        )
    {
        return core::ops::ControlFlow::Break(
            menu.creative || menu.levels >= grinding::cost(item, env.enchantments),
        );
    }
    core::ops::ControlFlow::Continue(())
}
