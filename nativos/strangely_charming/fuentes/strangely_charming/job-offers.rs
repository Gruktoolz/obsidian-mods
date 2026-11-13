fn hook(game: &mut crate::game::Game<'_>, index: usize) -> core::ops::ControlFlow<()> {
    let m = &game.mobs[index];
    let Some(job) = villager::evanders_jobs::id(m.extra.variant2) else {
        return core::ops::ControlFlow::Continue(());
    };
    if m.extra
        .trade
        .as_ref()
        .is_none_or(|t| t.unset || t.offers.is_empty())
    {
        let v = format!(
            "minecraft:{}",
            VILLAGER_TYPES[m.extra.variant as usize % VILLAGER_TYPES.len()]
        );
        let at = [m.position.x, m.position.y, m.position.z];
        let level = m.extra.variant3.max(1);
        let offers = if m.extra.variant2 == 17 {
            let mut offers = m
                .extra
                .trade
                .as_ref()
                .map(|t| t.offers.clone())
                .unwrap_or_default();
            for tier in 1..=level {
                for offer in game.offers_from_set(&format!("{job}/level_{tier}"), &v, at) {
                    if !offers
                        .iter()
                        .any(|old| old.result.item == offer.result.item)
                    {
                        offers.push(offer);
                    }
                }
            }
            offers
        } else {
            game.offers_from_set(&format!("{job}/level_{level}"), &v, at)
        };
        let trade = game.mobs[index].extra.trade.get_or_insert_default();
        trade.offers = offers;
        trade.unset = false;
    }
    core::ops::ControlFlow::Break(())
}
