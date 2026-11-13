fn hook(block: &str) -> core::ops::ControlFlow<Option<&'static str>> {
    if block == "strangely_charming:kiln" {
        return core::ops::ControlFlow::Break(Some("block.strangely_charming.kiln"));
    }
    core::ops::ControlFlow::Continue(())
}
