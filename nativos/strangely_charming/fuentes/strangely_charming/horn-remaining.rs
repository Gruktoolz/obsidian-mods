impl super::Cooldowns {
    pub fn evanders_horn_minutes(&self) -> Option<u32> {
        let (_, end) = self.entries.get("strangely_charming:raid_horn")?;
        let ticks = end.wrapping_sub(self.tick_count);
        (ticks > 0).then(|| (ticks as u32 + 600) / 1200)
    }
}
