// The mod's settings page: its parts, then a pin for each effect.
fn basic_hud_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::{basic_hud as m, basic_hud_i18n as i18n};
    if mod_id != m::ID {
        return;
    }
    let language = m::config(|| game.native_setting(m::ID, "config"), |c| c.language(&game.settings.language).to_owned());
    buttons.push(i18n::tr(&language, "Edit HUD...").to_owned());
    let name = |id: &str| horizon_client::lang::get(&format!("effect.minecraft.{id}"));
    let labels = m::config(|| game.native_setting(m::ID, "config"), |c| m::labels(c, name));
    for (index, label) in labels.into_iter().enumerate() {
        // Only our own six labels are translated; effect names already come
        // from the game, and never pass through a substring replacement.
        if index < 6 {
            let keys = ["Effects", "Unpinned effects too", "Potions, apples, totems", "Durability", "Totems used", "Corner"];
            if let Some((_, value)) = label.split_once(": ") {
                let value = match value {
                    "top left" => "Top left", "top right" => "Top right",
                    "bottom left" => "Bottom left", "bottom right" => "Bottom right",
                    value => value,
                };
                buttons.push(format!("{}: {}", i18n::tr(&language, keys[index]), i18n::tr(&language, value)));
                continue;
            }
        }
        let label = label.strip_suffix(": pinned").map(|name| format!("{name}: {}", i18n::tr(&language, "pinned"))).unwrap_or(label);
        buttons.push(label);
    }
}
