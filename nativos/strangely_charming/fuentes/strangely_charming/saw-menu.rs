use super::{Kind, Menu};
impl Menu {
    // The cutter owns slot zero for recipe selection. Slot one identifies the saw.
    pub fn evanders_set_table_saw(&mut self, saw: bool) {
        if self.kind == Kind::Stonecutter {
            self.data.truncate(1);
            if saw {
                self.data.push(1);
            }
        }
    }
    pub fn evanders_is_table_saw(&self) -> bool {
        self.kind == Kind::Stonecutter && self.data.get(1) == Some(&1)
    }
}
// The client reconstructs a transient Menu for each action. Capture the acting
// window during menu selection and consume its marker on the next constructor.
// Each guest's authoritative Menu receives its own marker at open time.
thread_local! { static SAW_SCREEN: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub fn prepare_screen(saw: bool) {
    SAW_SCREEN.set(saw);
}
pub(super) fn take_screen() -> bool {
    SAW_SCREEN.replace(false)
}
