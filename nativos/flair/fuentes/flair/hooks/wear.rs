fn flair_wear(game: &mut crate::game::Game<'_>, camera: glam::DVec3, alpha: f64, out: &mut Vec<horizon_client::mesher::Vertex>) {
    crate::game::umbx_flair_glue::wear(game, camera, alpha, out);
}
