// Client-only adapter for accepted ground-item pickups. Inventory edits are not receipts.
use std::cell::RefCell;
use std::time::{Duration, Instant};
use super::{Game, basic_hud as m, basic_hud_data::{Icon, Row}, basic_hud_i18n as i18n};
use crate::game::basic_hud_recent_pickups::PickupFeed;
use horizon_client::items::ItemStack;

struct State {
    epoch: Instant,
    feed: PickupFeed<ItemStack>,
    pending: Option<(ItemStack, u64)>,
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State { epoch: Instant::now(), feed: PickupFeed::new(), pending: None });
}

pub(super) fn clear() {
    horizon_client::net::basic_hud_pickup_packets::clear();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.feed.clear();
        state.pending = None;
    });
}

fn carried_count(game: &Game<'_>, stack: &ItemStack) -> u64 {
    game.inventory.iter().take(36).flatten().chain(game.offhand.as_ref())
        .filter(|item| item.same_item_same_components(stack)).map(|item| u64::from(item.count)).sum()
}

// Bracket the engine's one pickup attempt, not arbitrary inventory changes.
// add_pickup below is called synchronously after give succeeds. A refused/full
// attempt produces no receipt; chests, crafting and commands never call it.
pub(super) fn before_local(game: &Game<'_>, stack: &ItemStack) {
    if !game.native_window() || !game.native_on(m::ID) || game.remote.is_some() { return; }
    if !m::config(|| game.native_setting(m::ID, "config"), |c| c.enabled(m::Widget::Pickups)) { return; }
    STATE.with(|state| state.borrow_mut().pending = Some((stack.clone(), carried_count(game, stack))));
}

pub(super) fn accepted_local(game: &Game<'_>, stack: &ItemStack) {
    if !game.native_window() || game.remote.is_some() { return; }
    let pending = STATE.with(|state| state.borrow_mut().pending.take());
    if let Some((item, before)) = pending {
        if item.same_item_same_components(stack) {
            let count = crate::game::basic_hud_recent_pickups::accepted_amount(before, carried_count(game, stack), stack.count);
            record(game, stack, count);
        }
    }
}

fn ttl(config: &m::Config) -> Duration {
    Duration::from_secs(u64::from(config.pickup_seconds.clamp(1, 60)))
}

pub(super) fn record(game: &Game<'_>, stack: &ItemStack, count: u32) {
    if count == 0 || !game.native_window() || !game.native_on(m::ID)
        || game.player.as_ref().is_none_or(|p| p.health.dead_or_dying()) { return; }
    let (enabled, seconds) = m::config(|| game.native_setting(m::ID, "config"), |c| (c.enabled(m::Widget::Pickups), ttl(c)));
    if !enabled { clear(); return; }
    // ItemStack equality includes components and damage. Count does not identify an item.
    let mut identity = stack.clone();
    identity.count = 1;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let now = state.epoch.elapsed();
        state.feed.add(identity, u64::from(count), now, seconds);
    });
}

pub(super) fn frame(game: &Game<'_>) {
    let (enabled, seconds) = m::config(|| game.native_setting(m::ID, "config"), |c| (c.enabled(m::Widget::Pickups), ttl(c)));
    if !enabled || game.player.as_ref().is_none_or(|p| p.health.dead_or_dying()) { clear(); return; }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.pending = None;
        let now = state.epoch.elapsed();
        state.feed.expire(now, seconds);
    });
}

pub(super) fn rows(game: &Game<'_>, config: &m::Config, preview: bool) -> Vec<Row> {
    let mut rows = STATE.with(|state| {
        let mut state = state.borrow_mut();
        let now = state.epoch.elapsed();
        state.feed.expire(now, ttl(config));
        state.feed.entries().iter().map(|entry| {
            let name = game.stack_display_name(&entry.item).plain();
            let value = format!("x{}", entry.count);
            Row { icon: Icon::Item(entry.item.clone()), text: format!("{} {value}", name.trim_matches(['[', ']'])),
                value, sample: false, color: m::GOOD }
        }).collect::<Vec<_>>()
    });
    if rows.is_empty() && preview {
        let stack = ItemStack::new("minecraft:torch", 1);
        let name = game.stack_display_name(&stack).plain();
        let sample = i18n::tr(config.language(&game.settings.language), "Sample");
        rows.push(Row { icon: Icon::Item(stack), text: format!("[{sample}] {name} x6"),
            value: "x6".into(), sample: true, color: m::GOOD });
    }
    rows
}
