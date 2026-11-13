fn refuse_bone_meal(
    level: &crate::game::GameLevel<'_>,
    state: horizon_world::registry::BlockStateId,
    valid: &mut Option<bool>,
) {
    if valid.is_none() && level.facts.get(state).name.as_ref() == "inferium_lab:inferium_crop" {
        *valid = Some(false);
    }
}
