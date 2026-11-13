fn use_item_on(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    claimed: &mut bool,
) {
    if *claimed {
        return;
    }
    if crate::game::right_click_crops_gameplay::claim_offhand_followup(game, Some(pos)) {
        *claimed = true;
        return;
    }
    match crate::game::right_click_crops_gameplay::harvest(game, pos) {
        Ok(harvested) => *claimed = harvested,
        Err(error) => eprintln!("[right_click_crops] Harvest failed: {error}"),
    }
}
