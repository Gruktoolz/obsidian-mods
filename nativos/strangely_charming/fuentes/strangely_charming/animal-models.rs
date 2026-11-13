// Classic 64 × 32 box UVs used by Charm's animal skins.
use super::{EntityModelDocument, MobTypes, Part, Polygon};
use std::path::Path;
use std::sync::Arc;

fn part(name: &str, at: [f32; 3]) -> Part {
    Part {
        name: name.into(),
        pose: [at[0], at[1], at[2], 0., 0., 0., 1., 1., 1.],
        polygons: Vec::new(),
        children: Vec::new(),
    }
}

fn cuboid(p: &mut Part, origin: [f32; 3], size: [f32; 3], uv: [f32; 2]) {
    let [x, y, z] = origin;
    let [w, h, d] = size;
    let [u, v] = uv;
    let (xx, yy, zz) = (x + w, y + h, z + d);
    let corners = [
        [x, y, z],
        [xx, y, z],
        [xx, yy, z],
        [x, yy, z],
        [x, y, zz],
        [xx, y, zz],
        [xx, yy, zz],
        [x, yy, zz],
    ];
    for (face, normal, [u0, v0, u1, v1]) in [
        ([5, 4, 0, 1], [0., -1., 0.], [u + d, v, u + d + w, v + d]),
        (
            [2, 3, 7, 6],
            [0., 1., 0.],
            [u + d + w, v + d, u + d + 2. * w, v],
        ),
        ([0, 4, 7, 3], [-1., 0., 0.], [u, v + d, u + d, v + d + h]),
        (
            [1, 0, 3, 2],
            [0., 0., -1.],
            [u + d, v + d, u + d + w, v + d + h],
        ),
        (
            [5, 1, 2, 6],
            [1., 0., 0.],
            [u + d + w, v + d, u + 2. * d + w, v + d + h],
        ),
        (
            [4, 5, 6, 7],
            [0., 0., 1.],
            [u + 2. * d + w, v + d, u + 2. * d + 2. * w, v + d + h],
        ),
    ] {
        let uv = [[u1, v0], [u0, v0], [u0, v1], [u1, v1]];
        let vertices = std::array::from_fn(|n| {
            let [x, y, z] = corners[face[n]];
            [x, y, z, uv[n][0] / 64., uv[n][1] / 32.]
        });
        p.polygons.push(Polygon {
            vertices,
            normal,
            cull_back: false,
        });
    }
}

fn squid() -> Part {
    let mut root = part("root", [0.; 3]);
    let mut body = part("body", [0., 16., 0.]);
    cuboid(&mut body, [-3., -4., -3.], [6., 8., 6.], [0., 0.]);
    root.children.push(body);
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::TAU / 8.;
        let mut t = part(&format!("tentacle{i}"), [a.cos() * 2.5, 20., a.sin() * 2.5]);
        t.pose[4] = std::f32::consts::FRAC_PI_2 - a;
        cuboid(&mut t, [-0.5, 0., -0.5], [1., 7., 1.], [48., 0.]);
        root.children.push(t);
    }
    root
}

pub(super) fn load(
    root: &Path,
    render: bool,
    document: Option<&EntityModelDocument>,
) -> crate::Result<MobTypes> {
    let mut types = MobTypes::load_with_models(root, render, document)?;
    if let Some(kind) = types.types.get_mut("strangely_charming:coral_squid") {
        kind.width = 0.45;
        kind.height = 0.6;
        kind.eye_height = 0.35;
        if render {
            let model = Arc::new(squid());
            kind.model = model.clone();
            kind.layers.insert("squid#main".into(), model);
        }
    }
    Ok(types)
}
