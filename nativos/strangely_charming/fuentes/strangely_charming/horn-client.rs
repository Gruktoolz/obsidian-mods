impl crate::game::Game<'_> {
    pub(in crate::game) fn evanders_play_horn(&mut self, stack: &horizon_client::items::ItemStack) {
        self.remote_use_instrument(stack);
    }
}
