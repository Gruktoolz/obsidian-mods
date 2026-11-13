fn right_click_block(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    canceled: &mut bool,
) {
    if *canceled || game.held().is_some() {
        return;
    }
    match crate::game::right_click_crops_gameplay::harvest(game, pos) {
        Ok(true) => {
            game.swing_arm();
            *canceled = true;
        }
        Ok(false) => {}
        Err(error) => eprintln!("[right_click_crops] Harvest failed: {error}"),
    }
}
