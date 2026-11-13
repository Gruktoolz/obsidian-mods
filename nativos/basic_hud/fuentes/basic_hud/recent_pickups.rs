// Bounded, dependency-free pickup feed for recent item collections.
//
// Maintains collected items ordered newest first, aggregating repeated
// collections of matching items, saturating counts on overflow, and expiring
// stale entries after a configurable duration.
use std::time::Duration;

/// Strict maximum number of pickup entries retained in the feed.
pub const CAPACITY: usize = 64;

/// Inventory increase within one confirmed local pickup, bounded by its entity.
pub fn accepted_amount(before: u64, after: u64, available: u32) -> u32 {
    after.saturating_sub(before).min(u64::from(available)) as u32
}

/// A single item stack in the pickup feed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickupEntry<T> {
    pub item: T,
    pub count: u64,
    last_seen: Duration,
}

/// A generic, bounded feed of recently collected items.
///
/// Entries are maintained in newest-first order with a strict capacity limit of
/// 64 items. Repeated additions of equal items remove the older entry and
/// re-insert at the front with saturating summed count and refreshed expiry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickupFeed<T: PartialEq> {
    entries: Vec<PickupEntry<T>>,
}

impl<T: PartialEq> Default for PickupFeed<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PickupFeed<T> {
    /// Creates an empty pickup feed.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(CAPACITY),
        }
    }

    /// Adds `count` of `item` observed at `now` with time-to-live `ttl`.
    ///
    /// First expires stale entries (`age >= ttl`). If `count == 0`, no entry
    /// is added or modified. If an equal item already exists, its existing
    /// entry is removed and re-inserted at the front with saturating summed count
    /// and refreshed timestamp. Otherwise, a new entry is inserted at the front.
    /// The feed is then truncated to at most 64 entries.
    pub fn add(&mut self, item: T, count: u64, now: Duration, ttl: Duration) {
        self.expire(now, ttl);
        if count == 0 {
            return;
        }
        let count = if let Some(pos) = self.entries.iter().position(|e| e.item == item) {
            let existing = self.entries.remove(pos);
            existing.count.saturating_add(count)
        } else {
            count
        };
        self.entries.insert(
            0,
            PickupEntry {
                item,
                count,
                last_seen: now,
            },
        );
        self.entries.truncate(CAPACITY);
    }

    /// Removes all entries whose age (`now.saturating_sub(last_seen)`) is greater
    /// than or equal to `ttl`.
    pub fn expire(&mut self, now: Duration, ttl: Duration) {
        self.entries
            .retain(|entry| now.saturating_sub(entry.last_seen) < ttl);
    }

    /// Returns the current feed entries, ordered newest first.
    #[must_use]
    pub fn entries(&self) -> &[PickupEntry<T>] {
        &self.entries
    }

    /// Clears all entries from the feed.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
