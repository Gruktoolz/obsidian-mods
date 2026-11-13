fn hook(mob: &Mob) -> core::ops::ControlFlow<crate::text::Text> {
    if mob.species() == Species::Villager && mob.custom_name.is_none() {
        let name = match mob.extra.variant2 {
            15 => "Lumberjack",
            16 => "Beekeeper",
            17 => "Scrollkeeper",
            _ => return core::ops::ControlFlow::Continue(()),
        };
        return core::ops::ControlFlow::Break(crate::text::Text::literal(name));
    }
    core::ops::ControlFlow::Continue(())
}
