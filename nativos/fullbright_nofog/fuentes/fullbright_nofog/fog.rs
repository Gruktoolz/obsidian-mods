// The final fog: pushed past the view while NoFog is on. Its colour and the
// sky and cloud fade stay as they are.
fn fullbright_nofog_fog(
    game: &crate::game::Game<'_>,
    fog_type: horizon_client::fog::FogType,
    fog: &mut horizon_client::fog::FogData,
) {
    use crate::game::fullbright_nofog as m;
    if !game.native_on(m::ID) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if !settings.clears(fog_type == horizon_client::fog::FogType::Atmospheric) {
        return;
    }
    let [es, ee, rs, re] = m::cleared_distances();
    fog.environmental_start = es;
    fog.environmental_end = ee;
    fog.render_distance_start = rs;
    fog.render_distance_end = re;
}
