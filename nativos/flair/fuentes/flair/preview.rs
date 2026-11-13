use super::Game;
use super::umbx_flair_items::Stroke;
use super::umbx_flair_render::{self as render, FULL_LIGHT};
use glam::{DVec3, Mat4, Vec3};

const HEIGHT: f32 = 1.8;
const CROUCH_HEIGHT: f32 = 1.5;
const CROUCH_RAISE: f64 = 0.125;
const VIEW_HEIGHT: f32 = 2.7;
const VIEW_WIDTH: f32 = 1.9;
const VIEW_CENTER: f32 = 1.05;
const STROKE_SCALE: f32 = 0.004;

pub(super) struct Look {
    pub(super) yaw: f32,
    pub(super) strokes: Vec<Stroke>,
    pub(super) covered: bool,
}

pub(super) fn player(game: &Game<'_>, batch: &mut horizon_client::hud::HudBatch, rect: [f32; 4], look: &Look) {
    let Some(model) = game.player_model.as_ref() else { return };
    if game.player.as_ref().is_some_and(|p| p.health.dead_or_dying()) {
        return;
    }
    let mut m = model.clone();
    m.extra.model_scale = 0.0;
    m.body_yaw = look.yaw;
    m.previous_body_yaw = look.yaw;
    m.head_yaw = look.yaw;
    m.previous_head_yaw = look.yaw;
    m.head_pitch = 0.0;
    m.previous_head_pitch = 0.0;
    if look.covered {
        m.cape = None;
    }
    m.position.y += CROUCH_RAISE * f64::from(u8::from(m.extra.crouching));
    m.previous = m.position;
    let camera = DVec3::new(m.position.x, m.position.y, m.position.z);
    let atlas = &game.gui.atlas;
    let lookup = |name: &str| atlas.sprites.get(name).map(|r| [r.u0, r.v0, r.u1, r.v1]);
    let draw = if game.own_body_visible() { horizon_client::mobs::render::render_with } else { horizon_client::mobs::render::render_layers };
    let mut verts = Vec::new();
    draw(&m, 1.0, camera, &lookup, FULL_LIGHT, game.res.mob_data(), &game.player_armor_layers(), &mut verts);
    let chest = game.armor[1].as_ref().map(|s| &*s.item);
    if !game.own_invisible() {
        game.player_cape_geometry(&m, chest, camera, 1.0, FULL_LIGHT, &mut verts);
    }
    game.player_elytra_geometry(&m, chest, camera, 1.0, FULL_LIGHT, &mut verts);
    game.held_item_geometry(&m, camera, 1.0, FULL_LIGHT, false, None, &mut verts);
    game.head_item_geometry(&m, camera, 1.0, FULL_LIGHT, false, &mut verts);
    let [x, y, w, h] = rect;
    let size = (h / VIEW_HEIGHT).min(w / VIEW_WIDTH);
    let crouch = if m.extra.crouching { HEIGHT - CROUCH_HEIGHT } else { 0.0 };
    let gui = Mat4::from_translation(Vec3::new(x + w / 2.0, y + h / 2.0, 0.0))
        * Mat4::from_scale(Vec3::new(size, size, -size))
        * Mat4::from_translation(Vec3::new(0.0, VIEW_CENTER - crouch / 2.0, 0.0))
        * Mat4::from_rotation_z(std::f32::consts::PI);
    let mut quads = Vec::new();
    render::model_quads(&verts, gui, true, &mut quads);
    render::stroke_quads(game, &look.strokes, gui, size * STROKE_SCALE, &mut quads);
    render::flush(batch, quads, rect);
}
