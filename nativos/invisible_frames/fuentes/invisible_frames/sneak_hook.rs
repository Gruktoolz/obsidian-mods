fn capture(host: &mut crate::game::lan::LanHost, conn: u32) {
    if let Some(guest) = host.hub.guest(conn) {
        super::invisible_frames_sneak::record(guest.uuid, guest.sneaking);
    }
}
