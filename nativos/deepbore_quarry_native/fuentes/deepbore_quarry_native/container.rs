fn deepbore_quarry_container(
    block: &str,
    container: &mut Option<horizon_client::containers::ModContainer>,
) {
    crate::game::deepbore_quarry_glue::container(block, container);
}
