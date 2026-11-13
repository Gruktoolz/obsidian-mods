// Original MIT implementation of the documented shift-right-click behaviour.
fn toggle_frame(
    game: &mut crate::game::Game<'_>,
    entity: i32,
    kind: &str,
    claimed: &mut bool,
) {
    if *claimed
        || !matches!(kind, "minecraft:item_frame" | "minecraft:glow_item_frame")
        || !super::invisible_frames_sneak::is_sneaking(
            game.native_acting_player().map(|p| p.uuid),
        )
    {
        return;
    }
    let Some(frame) = game.item_frames.iter_mut().find(|f| f.id == entity as u32) else {
        return;
    };
    // Empty frames stay visible, unless this click makes a hidden one visible again.
    // Do not unlock a pre-existing fixed visible frame in survival.
    if (frame.item.is_none() && !frame.invisible)
        || (frame.fixed && !frame.invisible && !game.creative)
    {
        return;
    }
    frame.invisible = !frame.invisible;
    frame.fixed = frame.invisible;
    let [x, _, z] = frame.pos;
    game.world.modified.insert((x >> 4, z >> 4));
    *claimed = true;
}
