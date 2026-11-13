// After the text displays: the elytra trails.
fn livingmotion_world(
    value: (),
    game: &mut crate::game::Game<'_>,
    camera: glam::DVec3,
    partial: f64,
    normal: &mut Vec<horizon_client::mesher::Vertex>,
) {
    let () = value;
    crate::game::livingmotion::world(&*game, camera, partial, normal);
}
