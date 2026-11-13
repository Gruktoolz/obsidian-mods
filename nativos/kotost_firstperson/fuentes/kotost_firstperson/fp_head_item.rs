fn kotost_fp_head_item() -> std::ops::ControlFlow<()> {
    if horizon_client::kotost_fp::mode() == horizon_client::kotost_fp::Mode::Body {
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
