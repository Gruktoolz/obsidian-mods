// A player died: respawn them at random once they are alive again.
fn killc_died(player: u128) {
    crate::game::killc::died(player);
}
