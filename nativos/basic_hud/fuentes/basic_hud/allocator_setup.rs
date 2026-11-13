// The allocator is set up first thing in main: the thread callback goes in with it.
fn basic_hud_allocator_setup() {
    crate::basic_hud_thread_done::install();
}
