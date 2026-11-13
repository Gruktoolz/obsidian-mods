fn adapt_result(
    value: (bool, bool, bool),
    hub: &crate::lan::hub::Hub,
    index: usize,
    destination: [f64; 3],
    ctx: &crate::lan::hub::TickCtx<'_>,
) -> (bool, bool, bool) {
    crate::lan::hub::fence_perch_server::adapt(value, hub, index, destination, ctx)
}
