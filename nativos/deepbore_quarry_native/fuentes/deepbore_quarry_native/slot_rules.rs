// Fuel's slot takes anything but an upgrade: fuel is the game's data, out of reach
// here, and what isn't fuel just doesn't burn. An upgrade slot takes one upgrade.
fn deepbore_quarry_slot_rules(
    menu: &str,
    slot: usize,
    stack: &horizon_client::items::ItemStack,
    may_place: &mut bool,
    max_stack: &mut u32,
) {
    if menu != "deepbore_quarry:quarry" {
        return;
    }
    let upgrade = matches!(
        &*stack.item,
        "deepbore_quarry:area_upgrade"
            | "deepbore_quarry:speed_upgrade"
            | "deepbore_quarry:fluids_upgrade"
            | "deepbore_quarry:chunk_upgrade"
    );
    if slot == 0 {
        *may_place = !upgrade;
    } else if (1..5).contains(&slot) {
        *may_place = upgrade;
        *max_stack = 1;
    } else {
        *may_place = false;
    }
}
