// Right-click harvest: a ripe crop under the crosshair and an empty hand pick
// it through the player's own break (drops, statistics, sound) and sow it back
// from the inventory. Break ends the click, as vanilla's own use would.
fn rightharvest_use_hand(
    game: &mut crate::game::Game<'_>,
    block_hit: Option<&Hit>,
) -> ::core::ops::ControlFlow<Result<bool>> {
    use crate::game::native_rightharvest as m;
    use core::ops::ControlFlow;

    let Some(hit) = block_hit else {
        return ControlFlow::Continue(());
    };
    // Only an empty hand — a held item keeps its own use — turned on in
    // Options > Mods, and not a spectator (who acts on nothing).
    if !game.native_on(m::ID) || game.held().is_some() || game.spectator() {
        return ControlFlow::Continue(());
    }
    let pos = hit.pos;
    let state = game.world.block(pos[0], pos[1], pos[2]);
    let name = game.res.facts().get(state).name.clone();
    let age = game
        .pack
        .assets
        .registry
        .state(state)
        .and_then(|s| s.properties.get("age"));
    let Some(seed) = m::ripe_seed(&name, age.map(String::as_str)) else {
        return ControlFlow::Continue(());
    };
    if let Err(e) = game.break_block_as_player(pos) {
        eprintln!("[rightharvest] harvesting {name} at {pos:?}: {e}");
        return ControlFlow::Continue(());
    }
    // A creative break drops nothing and creative sowing spends nothing.
    let mut sown = game.creative;
    if !sown {
        for slot in 0..game.inventory.len() {
            let left = match game.inventory[slot].as_mut() {
                Some(stack) if stack.count > 0 && &*stack.item == seed => {
                    stack.count -= 1;
                    stack.count
                }
                _ => continue,
            };
            if left == 0 {
                game.inventory[slot] = None;
            }
            sown = true;
            break;
        }
    }
    if sown
        && let Some(young) = game.with_props(state, &[("age", "0")])
        && let Err(e) = game.level_set(pos, young)
    {
        eprintln!("[rightharvest] sowing {name} at {pos:?}: {e}");
    }
    game.swing_arm();
    ControlFlow::Break(Ok(true))
}
