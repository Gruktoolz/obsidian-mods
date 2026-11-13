use super::Game;
use super::umbx_flair_capes as capes;
use super::umbx_flair_catalog as catalog;
use super::umbx_flair_glue::{self as glue, Entry};
use super::umbx_flair_items::{Attach, Item, Slot, Stroke};
use super::umbx_flair_model::{self as model, Baked};
use glam::{DVec3, Mat4, Vec3};
use horizon_client::hud::HudBatch;
use horizon_client::mesher::Vertex;
use horizon_client::mobs::{Mob, Part, Polygon, render, render_model};

pub(super) const FULL_LIGHT: u32 = 240 | 240 << 8;
const TEXTURE: [f32; 2] = [64.0, 64.0];
const HELMET_GROW: f32 = 1.25;
const HELMET_LIFT: f32 = 1.0 / 16.0;
const ARMOR_LIFT: f32 = 1.5 / 16.0;
const ICON_TILT: f32 = 0.4;
const ICON_SPIN: f32 = 0.8;
const ICON_FILL: f32 = 0.8;
const AURA_CENTER: f32 = 1.1;
const AURA_EXTENT: f32 = 2.6;
const STROKE_SCALE: f32 = 0.006;

pub(super) type Quad = (f32, [[f32; 3]; 4], [[f32; 2]; 4], u32);

pub(super) struct Shape {
    parts: Vec<Part>,
    center: Vec3,
    extent: f32,
}

pub(super) fn shape(item: &Item) -> Shape {
    let layers = model::layers(&item.bones).max(1);
    let parts = (0..layers)
        .map(|layer| {
            part(&Baked {
                name: String::new(),
                pose: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                faces: Vec::new(),
                children: item.bones.iter().map(|b| model::bake(b, [0.0; 3], TEXTURE, layer)).collect(),
            })
        })
        .collect();
    let (lo, hi) = model::bounds(&item.bones).unwrap_or(([-4.0, -8.0, -4.0], [4.0, 0.0, 4.0]));
    let extent = (0..3).map(|i| hi[i] - lo[i]).fold(1.0, f32::max) / 16.0;
    Shape { parts, center: Vec3::from(lo).lerp(Vec3::from(hi), 0.5) / 16.0, extent }
}

fn pose(item: &Item, part: &Part, seconds: f32, swing: [f32; 2]) -> [f32; 9] {
    let mut out = item.motion(&part.name, seconds).apply(part.pose);
    if item.slot == Slot::Cape
        && let Some(i) = part.name.strip_prefix("seg").and_then(|i| i.parse().ok())
    {
        let bend = capes::bend(swing, seconds, i);
        for (k, angle) in bend.into_iter().enumerate() {
            out[3 + k] += angle;
        }
    }
    out
}

fn model_verts(game: &Game<'_>, entry: &Entry, base: Mat4, seconds: f32, swing: [f32; 2], light: u32, out: &mut Vec<Vertex>) {
    let Some(shape) = &entry.shape else { return };
    for (layer, part) in shape.parts.iter().enumerate() {
        if let Some(uv) = sprite(game, &glue::texture(&entry.item, layer)) {
            render_model(part, base, &|p| pose(&entry.item, p, seconds, swing), uv, light, 0, out);
        }
    }
}

fn part(baked: &Baked) -> Part {
    Part {
        name: baked.name.clone(),
        pose: baked.pose,
        polygons: baked.faces.iter().map(|f| Polygon { normal: f.normal, vertices: f.vertices, cull_back: false }).collect(),
        children: baked.children.iter().map(part).collect(),
    }
}

fn sprite(game: &Game<'_>, name: &str) -> Option<[f32; 4]> {
    game.gui.atlas.sprites.get(name).map(|r| [r.u0, r.v0, r.u1, r.v1])
}

pub(super) fn worn(game: &Game<'_>, mob: &Mob, camera: DVec3, alpha: f64, light: u32, lit: bool, out: &mut Vec<Vertex>) {
    let local = game.player_model.as_ref().is_some_and(|p| p.id == mob.id && p.kind.id == mob.kind.id);
    if local {
        draw(game, mob, camera, alpha, light, lit, &[Slot::Cape, Slot::Hat, Slot::Back, Slot::Pet, Slot::Aura], out);
    }
}

pub(super) fn first_person(game: &Game<'_>, camera: DVec3, alpha: f64, out: &mut Vec<Vertex>) {
    if game.player_model_shown() || !glue::state().first_person[Slot::Pet as usize] {
        return;
    }
    let Some(m) = game.player_model.as_ref() else { return };
    let light = game.world.light(m.position.x.floor() as i32, (m.position.y + 1.62).floor() as i32, m.position.z.floor() as i32);
    let packed = (u32::from(light >> 4) * 16) | ((u32::from(light & 15) * 16) << 8);
    draw(game, m, camera, alpha, packed, true, &[Slot::Pet], out);
}

fn draw(game: &Game<'_>, mob: &Mob, camera: DVec3, alpha: f64, light: u32, lit: bool, slots: &[Slot], out: &mut Vec<Vertex>) {
    if !game.native_on(catalog::ID) || game.own_invisible() {
        return;
    }
    let flair = glue::state();
    let seconds = flair.seconds();
    let swing = if lit { flair.swing } else { [0.0; 2] };
    for slot in slots {
        if *slot == Slot::Cape && !flair.cape_shown() {
            continue;
        }
        let Some(entry) = flair.worn(*slot).map(|i| &flair.entries[i]) else { continue };
        let Some(base) = attach(game, mob, camera, alpha as f32, entry.item.attach, *slot, flair.hide_armor) else { continue };
        let light = if entry.item.glow { FULL_LIGHT } else { light };
        model_verts(game, entry, base, seconds, swing, light, out);
    }
}

fn attach(game: &Game<'_>, mob: &Mob, camera: DVec3, alpha: f32, attach: Attach, slot: Slot, hide_armor: bool) -> Option<Mat4> {
    let data = game.res.mob_data();
    match attach {
        Attach::Head => {
            let head = render::part_transform(mob, alpha, camera, data, "head")?;
            if hide_armor || game.armor[0].is_none() {
                return Some(head);
            }
            Some(head * Mat4::from_translation(Vec3::new(0.0, -HELMET_LIFT, 0.0)) * Mat4::from_scale(Vec3::new(HELMET_GROW, 1.0, HELMET_GROW)))
        }
        Attach::Body => {
            let body = render::part_transform(mob, alpha, camera, data, "body")?;
            let lift = match (!hide_armor && game.armor[1].is_some(), slot) {
                (true, Slot::Back | Slot::Cape) => Vec3::new(0.0, 0.0, ARMOR_LIFT),
                (true, Slot::Pet) => Vec3::new(0.0, -ARMOR_LIFT, 0.0),
                _ => Vec3::ZERO,
            };
            Some(body * Mat4::from_translation(lift))
        }
        Attach::Root => Some(render::base_transform(mob, alpha, camera)),
    }
}

pub(super) fn model_quads(verts: &[Vertex], gui: Mat4, cull: bool, out: &mut Vec<Quad>) {
    for q in verts.chunks_exact(4) {
        let p: [Vec3; 4] = std::array::from_fn(|i| gui.transform_point3(Vec3::from_array(q[i].pos)));
        let n = (p[1] - p[0]).cross(p[2] - p[0]);
        if cull && n.z >= 0.0 {
            continue;
        }
        let n = if n.z >= 0.0 { -n } else { n }.normalize_or_zero();
        let shade = (0.55 + 0.45 * -n.z - 0.15 * n.y).clamp(0.4, 1.0);
        let c = q[0].color;
        let channel = |s: u32| ((((c >> s) & 255) as f32 * shade) as u32) << s;
        let color = (c & 0xFF00_0000) | channel(0) | channel(8) | channel(16);
        out.push((p.iter().map(|v| v.z).sum::<f32>() / 4.0, p.map(|v| v.to_array()), std::array::from_fn(|i| q[i].uv), color));
    }
}

pub(super) fn stroke_quads(game: &Game<'_>, strokes: &[Stroke], gui: Mat4, width: f32, out: &mut Vec<Quad>) {
    let Some([u0, v0, u1, v1]) = sprite(game, "horizon/white") else { return };
    let uv = [(u0 + u1) / 2.0, (v0 + v1) / 2.0];
    for s in strokes {
        let (a, b) = (gui.transform_point3(DVec3::from(s.a).as_vec3()), gui.transform_point3(DVec3::from(s.b).as_vec3()));
        let along = (b - a).truncate().normalize_or_zero();
        let side = Vec3::new(-along.y, along.x, 0.0) * (s.width * width / 2.0).max(0.35);
        let [alpha, r, g, bl] = s.color.to_be_bytes();
        let color = horizon_client::hud::rgba(r, g, bl, alpha);
        out.push(((a.z + b.z) / 2.0, [(a + side).to_array(), (b + side).to_array(), (b - side).to_array(), (a - side).to_array()], [uv; 4], color));
    }
}

pub(super) fn flush(batch: &mut HudBatch, mut quads: Vec<Quad>, [x, y, w, h]: [f32; 4]) {
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    quads.sort_by(|a, b| a.0.total_cmp(&b.0));
    let start = batch.vertices.len();
    for (_, p, uv, color) in quads {
        batch.quad_3d(p, uv, color);
    }
    batch.depth_since(start, [x, y, x + w, y + h]);
}

pub(super) fn icon(game: &Game<'_>, batch: &mut HudBatch, entry: &Entry, seconds: f32, theme: usize, rect: [f32; 4], clip: [f32; 4]) {
    let [x, y, w, h] = rect;
    let spin = Mat4::from_rotation_x(ICON_TILT) * Mat4::from_rotation_y(seconds * ICON_SPIN);
    let mut quads = Vec::new();
    if entry.item.effect.is_some() {
        let size = w.min(h) / AURA_EXTENT;
        let gui = Mat4::from_translation(Vec3::new(x + w / 2.0, y + h / 2.0, 0.0)) * Mat4::from_scale(Vec3::new(size, size, -size)) * Mat4::from_rotation_z(std::f32::consts::PI) * spin * Mat4::from_translation(Vec3::new(0.0, -AURA_CENTER, 0.0));
        let mut strokes = Vec::new();
        entry.item.stroke(f64::from(seconds), 0.0, theme, &mut strokes);
        stroke_quads(game, &strokes, gui, size * STROKE_SCALE, &mut quads);
    } else {
        let Some(shape) = &entry.shape else { return };
        let mut verts = Vec::new();
        model_verts(game, entry, Mat4::IDENTITY, seconds, [0.0; 2], FULL_LIGHT, &mut verts);
        let size = w.min(h) / shape.extent * ICON_FILL;
        let gui = Mat4::from_translation(Vec3::new(x + w / 2.0, y + h / 2.0, 0.0)) * Mat4::from_scale(Vec3::new(size, size, -size)) * spin * Mat4::from_translation(-shape.center);
        model_quads(&verts, gui, false, &mut quads);
    }
    flush(batch, quads, clip);
}
