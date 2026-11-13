fn hook(m: &Mob, age: f32) -> core::ops::ControlFlow<Vec<Layer>> {
    if let Some(id) = super::villager::evanders_jobs::id(m.extra.variant2) {
        if matches!(m.species(), Species::Villager | Species::ZombieVillager) {
            let mut model = m.clone();
            model.extra.variant2 = 7;
            let mut result = layers(&model, age);
            let job = id.split_once(':').unwrap().1;
            for layer in &mut result {
                if layer.texture.ends_with("/profession/fletcher") {
                    layer.texture = format!(
                        "strangely_charming:{}/profession/{job}",
                        if m.species() == Species::ZombieVillager {
                            "entity/zombie_villager"
                        } else {
                            "entity/villager"
                        }
                    )
                    .into();
                }
            }
            return core::ops::ControlFlow::Break(result);
        }
    }
    core::ops::ControlFlow::Continue(())
}
