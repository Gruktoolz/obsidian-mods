pub fn id(profession: u8) -> Option<&'static str> {
    match profession {
        15 => Some("strangely_charming:lumberjack"),
        16 => Some("strangely_charming:beekeeper"),
        17 => Some("strangely_charming:scrollkeeper"),
        _ => None,
    }
}
pub fn number(id: &str) -> Option<u8> {
    match id {
        "strangely_charming:lumberjack" => Some(15),
        "strangely_charming:beekeeper" => Some(16),
        "strangely_charming:scrollkeeper" => Some(17),
        _ => None,
    }
}
