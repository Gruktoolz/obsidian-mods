fn hook(game: &mut crate::game::Game<'_>, packet: &ClientboundPlayPacket) {
    if let ClientboundPlayPacket::CustomPayload(p) = packet
        && p.channel == crate::game::sc_alerts::CHANNEL
    {
        game.sc_receive_alert(&p.data);
    }
    if let ClientboundPlayPacket::CustomPayload(p) = packet
        && p.channel == crate::game::sc_quests::BADGES
    {
        crate::game::sc_quests::read_badges(&p.data);
    }
}
