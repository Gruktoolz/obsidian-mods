// The items the mod declares (API 145): the wayfarer's scroll.
fn wayfarer_stones_items(items: &mut Vec<horizon_mods::content::Item>) {
    crate::game::wayfarer_stones_glue::items(items);
}
