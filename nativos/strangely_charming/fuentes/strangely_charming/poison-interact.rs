fn hook(game: &mut crate::game::Game<'_>, entity: i32, stack: Option<&ItemStack>, claimed: &mut bool) { crate::game::strangely_charming::interact(game, entity, stack, claimed); }
