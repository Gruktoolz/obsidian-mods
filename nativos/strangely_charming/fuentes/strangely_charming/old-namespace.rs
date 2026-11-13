fn hook(bytes: &[u8]) -> core::ops::ControlFlow<Option<JavaString>> {
    if let Some(path) = bytes.strip_prefix(b"evanders_tweaks:")
        && matches!(
            path,
            b"poison_apple"
                | b"raid_horn"
                | b"woodcutter"
                | b"kiln"
                | b"ender_pearl_block"
                | b"coral_squid"
                | b"lumberjack"
                | b"beekeeper"
        )
    {
        let id = format!("strangely_charming:{}", std::str::from_utf8(path).unwrap());
        return core::ops::ControlFlow::Break(Some(id.as_str().into()));
    }
    core::ops::ControlFlow::Continue(())
}
