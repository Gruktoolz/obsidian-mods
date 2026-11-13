use super::{Game, TOTEM};

pub(super) fn fallback_slot(game: &Game<'_>) -> Option<usize> {
    if !horizon_mods::evanders_config::enabled("inventory_totem")
        || game.held().is_some_and(|s| game.has_death_protection(s))
        || game
            .offhand
            .as_ref()
            .is_some_and(|s| game.has_death_protection(s))
    {
        return None;
    }
    game.inventory.iter().position(|cell| {
        cell.as_ref().is_some_and(|s| {
            s.item.as_ref() == TOTEM && s.count > 0 && game.has_death_protection(s)
        })
    })
}
