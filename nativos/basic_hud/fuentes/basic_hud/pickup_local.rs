// The confirmed local collection (including partial inventories), excluding mobs.
fn basic_hud_pickup_local(game: &crate::game::Game<'_>, taken: &crate::game::pickups::Taken, collector: crate::game::pickups::Collector) {
    if collector == crate::game::pickups::Collector::Player {
        if let crate::game::pickups::Taken::Item(item) = taken {
            if let Some(remote) = &game.remote {
                if let Some(count) = horizon_client::net::basic_hud_pickup_packets::take(&remote.session, item.id, remote.entity_id) {
                    crate::game::basic_hud_pickup_data::record(game, &item.stack, count.min(item.stack.count));
                }
            } else {
                crate::game::basic_hud_pickup_data::accepted_local(game, &item.stack);
            }
        }
    }
}
