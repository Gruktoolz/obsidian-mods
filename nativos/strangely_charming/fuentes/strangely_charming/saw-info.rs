fn hook(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<serde_json::Value> {
    if !game.evanders_table_saw() {
        return core::ops::ControlFlow::Continue(());
    }
    let s = game.station.as_ref().expect("active Table Saw");
    let recipes = &game.evanders_cut_recipes().stonecutting;
    core::ops::ControlFlow::Break(serde_json::json!({
        "costs":s.offers.costs,"clues":[null,null,null],"bookcases":s.bookcases,
        "anvil_cost":s.anvil.cost, "result":s.result.as_ref().map(|r|format!("{} x{}",r.item,r.count)),
        "recipes":s.stone.recipes.iter().map(|&i|&recipes[i].id).collect::<Vec<_>>(),"selected":s.selected
    }))
}
