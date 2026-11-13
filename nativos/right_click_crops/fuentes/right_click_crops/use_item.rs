fn use_item(
    game: &crate::game::Game<'_>,
    claimed: &mut bool,
) {
    if !*claimed && crate::game::right_click_crops_gameplay::claim_offhand_followup(game, None) {
        *claimed = true;
    }
}
