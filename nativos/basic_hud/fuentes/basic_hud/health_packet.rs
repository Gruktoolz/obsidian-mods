// Ingests SetEntityData packets to track remote player health without spoofing defaults.
fn basic_hud_health_packet(
    game: &mut crate::game::Game<'_>,
    id: i32,
    values: &[(u8, i32, horizon_protocol::entity_data::Value)],
) {
    crate::game::basic_hud_data::observe_health(game, id, values);
}
