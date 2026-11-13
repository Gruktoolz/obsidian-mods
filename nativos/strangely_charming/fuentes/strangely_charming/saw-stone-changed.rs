fn hook(menu: &mut Menu, env: &Env) -> core::ops::ControlFlow<()> {
    if !menu.evanders_is_table_saw() {
        return core::ops::ControlFlow::Continue(());
    }
    let recipes = crate::stations::evanders_saw::recipes();
    if std::ptr::eq(env.station_recipes, recipes) {
        return core::ops::ControlFlow::Continue(());
    }
    let saw = Env {
        station_recipes: recipes,
        ..*env
    };
    core::ops::ControlFlow::Break(menu.stone_changed(&saw))
}
