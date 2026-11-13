fn deepbore_quarry_register(commands: &mut Vec<horizon_client::mod_api::NativeCommand>) {
    crate::game::deepbore_quarry_glue::commands(commands);
}
