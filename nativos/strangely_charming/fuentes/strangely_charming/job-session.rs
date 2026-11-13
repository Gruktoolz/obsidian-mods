fn hook(
    game: &mut crate::game::Game<'_>,
    index: usize,
    item: &str,
) -> core::ops::ControlFlow<horizon_client::Result<bool>> {
    let profession = game.mobs[index].extra.variant2;
    if (profession == 17 && !horizon_mods::evanders_config::enabled("scrollkeepers"))
        || (profession == 15 && !horizon_mods::evanders_config::enabled("lumberjacks"))
        || (profession == 16 && !horizon_mods::evanders_config::enabled("beekeepers"))
    {
        return core::ops::ControlFlow::Break(Ok(false));
    }
    if villager::evanders_jobs::id(profession).is_none() {
        return core::ops::ControlFlow::Continue(());
    }
    fn profession_is_keeper(game: &crate::game::Game<'_>, index: usize) -> bool {
        game.mobs[index].extra.variant2 == 17
    }
    fn run(
        game: &mut crate::game::Game<'_>,
        index: usize,
        item: &str,
    ) -> horizon_client::Result<bool> {
        let m = &game.mobs[index];
        let species = m.species();
        if !matches!(species, Species::Villager | Species::WanderingTrader)
            || !m.alive()
            || item == "minecraft:villager_spawn_egg"
            || game.station.is_some()
            || m.extra.flag2
            || (species == Species::Villager && villager::sleeping(m))
        {
            return Ok(false);
        }
        if m.is_baby() {
            // A baby villager shakes its head; a baby wandering trader does
            // nothing of its own.
            if species == Species::WanderingTrader {
                return Ok(false);
            }
            let mut events = Vec::new();
            villager::set_unhappy(&mut game.mobs[index], &mut events);
            game.handle_mob_events(events)?;
            return Ok(true);
        }
        game.award_custom_stat("talked_to_villager", 1);
        if profession_is_keeper(game, index) && game.native_sneaking() {
            game.sc_quest_desk(index);
            return Ok(true);
        }
        game.ensure_offers(index);
        if profession_is_keeper(game, index) {
            game.sc_stamp_scrolls(index);
        }
        let m = &game.mobs[index];
        let trade = m.extra.trade.as_deref().cloned().unwrap_or_default();
        if trade.offers.is_empty() {
            if species == Species::Villager {
                let mut events = Vec::new();
                villager::set_unhappy(&mut game.mobs[index], &mut events);
                game.handle_mob_events(events)?;
            }
            return Ok(true);
        }
        let villager = species == Species::Villager;
        let level = if villager {
            i32::from(m.extra.variant3.max(1)).min(5)
        } else {
            1
        };
        let mut session = merchant::Session::new(trade.offers, villager);
        session.xp = trade.xp;
        session.level = level;
        session.profession = villager::evanders_jobs::id(m.extra.variant2)
            .unwrap()
            .into();
        session.variant = format!(
            "minecraft:{}",
            VILLAGER_TYPES[m.extra.variant as usize % VILLAGER_TYPES.len()]
        );
        session.random = m.random.clone();
        session.origin = [m.position.x, m.position.y, m.position.z];
        session.ambient_sound_time = m.ambient_timer;
        let (id, name) = (
            m.id,
            m.custom_name.clone().or_else(|| {
                Some(
                    if m.extra.variant2 == 15 {
                        "Lumberjack"
                    } else if m.extra.variant2 == 17 {
                        "Scrollkeeper"
                    } else {
                        "Beekeeper"
                    }
                    .into(),
                )
            }),
        );
        if villager {
            session.reputation = game.villager_reputation(id);
            session.hero = game.hero_amplifier();
            for seq in merchant::Session::sequences_needed(
                &game.stations().trades,
                &session.profession,
                level,
            ) {
                let r = game.rand_sequences.lend(&seq, game.level.seed);
                session.sequences.insert(seq, r);
            }
        }
        // startTrading: updateSpecialPrices, then the screen opens.
        session.update_special_prices();
        // setTradingPlayer: a wandering trader stands still while trading, a
        // villager's brain follows and looks at the player
        // (LookAndFollowTradingPlayerSink).
        game.mobs[index].extra.flag2 = true;
        game.open_merchant_state(MerchantState {
            session: Box::new(session),
            scroll: 0,
            dragging: false,
            focused: None,
            mob: Some(id),
            name,
        });
        Ok(true)
    }
    core::ops::ControlFlow::Break(run(game, index, item))
}
