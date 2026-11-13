fn basic_hud_pickup_poll(events: Vec<Event>, session: &crate::net::Session) -> Vec<Event> {
    crate::net::basic_hud_pickup_packets::observe(session, &events);
    events
}
