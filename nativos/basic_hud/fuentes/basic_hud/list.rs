// The mod's row in Options > Mods.
fn basic_hud_listed(
    mods: &mut Vec<crate::game::native_hooks::NativeMod>,
    game: &mut crate::game::Game<'_>,
) {
    use crate::game::{basic_hud as m, basic_hud_i18n as i18n};
    let language = m::config(|| game.native_setting(m::ID, "config"), |c| c.language(&game.settings.language).to_owned());
    let key_title = i18n::tr(&language, "HUD Layout Editor").to_owned();
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: i18n::tr(&language, m::DESCRIPTION).into(),
        keys: vec![["layout_editor".into(), key_title, "key.keyboard.right.shift".into()]],
    });
}
