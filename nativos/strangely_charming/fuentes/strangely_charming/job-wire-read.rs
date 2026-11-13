fn hook(_ret: (), m: &mut Mob, values: &[(u8, i32, Value)]) {
    if matches!(m.species(), Species::Villager | Species::ZombieVillager) {
        if let Some((_, _, Value::Int(v))) = values.iter().find(|(k, _, _)| *k == 120) {
            let job = (v & 255) as u8;
            if job <= 17 {
                m.extra.variant2 = job;
            }
        }
    }
}
