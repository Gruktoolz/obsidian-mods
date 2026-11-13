fn hook(block: &str) -> core::ops::ControlFlow<Option<u8>> {
    match block {
        "strangely_charming:woodcutter"
            if horizon_mods::evanders_config::enabled("lumberjacks") =>
        {
            core::ops::ControlFlow::Break(Some(15))
        }
        "minecraft:beehive" if horizon_mods::evanders_config::enabled("beekeepers") => {
            core::ops::ControlFlow::Break(Some(16))
        }
        "strangely_charming:writing_desk"
            if horizon_mods::evanders_config::enabled("scrollkeepers") =>
        {
            core::ops::ControlFlow::Break(Some(17))
        }
        _ => core::ops::ControlFlow::Continue(()),
    }
}
