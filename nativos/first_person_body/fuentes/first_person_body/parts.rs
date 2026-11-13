// `player_model_visibility`, overwritten: the game's own visibility (a
// spectator shows only its head, the skin layers the player hid are off),
// then, while the own first-person model is drawn, the head and hat hidden
// (the camera is inside them), and the arms too when the vanilla hand draws
// instead. Worn armour follows the parts' poses.
fn first_person_body_visibility(p: &mut Poses, hidden: u8, spectator: bool) {
    use horizon_client::mobs::first_person_body as m;
    for part in ["body", "right_arm", "left_arm", "right_leg", "left_leg"] {
        p.visible(part, !spectator);
    }
    for (bit, part) in [
        (1, "jacket"),
        (2, "left_sleeve"),
        (3, "right_sleeve"),
        (4, "left_pants"),
        (5, "right_pants"),
        (6, "hat"),
    ] {
        p.visible(part, hidden & (1 << bit) == 0);
    }
    if m::active().is_some() {
        for part in m::hidden_parts(&m::current()) {
            p.visible(part, false);
        }
    }
}
