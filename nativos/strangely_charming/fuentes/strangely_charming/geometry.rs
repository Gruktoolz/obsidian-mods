use super::{Mob, MobData, Part, Polygon, anim, render};
use glam::{Mat4, Vec3};
use horizon_client_render::{strangely_charming_format::Features, strangely_charming_skin as skin};

struct Mesh {
    transform: Mat4,
    stack: Vec<Mat4>,
    polygons: Vec<Polygon>,
}
impl Mesh {
    fn new(origin: Vec3) -> Self {
        Self {
            transform: Mat4::from_translation(origin),
            stack: Vec::new(),
            polygons: Vec::new(),
        }
    }
    fn push(&mut self) {
        self.stack.push(self.transform);
    }
    fn pop(&mut self) {
        self.transform = self.stack.pop().expect("balanced Ears geometry stack");
    }
    fn t(&mut self, x: f32, y: f32, z: f32) {
        self.transform *= Mat4::from_translation(Vec3::new(x, y, z));
    }
    fn r(&mut self, degrees: f32, axis: usize) {
        self.transform *= match axis {
            0 => Mat4::from_rotation_x(degrees.to_radians()),
            1 => Mat4::from_rotation_y(degrees.to_radians()),
            _ => Mat4::from_rotation_z(degrees.to_radians()),
        };
    }
    // Pixel UVs on a 64-square atlas slot, with Ears' clockwise texture rotation.
    fn quad(&mut self, u: f32, v: f32, w: f32, h: f32, rotation: u8, flip: bool, back: bool) {
        let (uw, vh) = if rotation % 2 == 1 { (h, w) } else { (w, h) };
        let (mut u0, mut u1, mut v0, mut v1) = (u / 64., (u + uw) / 64., v / 64., (v + vh) / 64.);
        if flip {
            if rotation % 2 == 1 {
                std::mem::swap(&mut v0, &mut v1);
            } else {
                std::mem::swap(&mut u0, &mut u1);
            }
        }
        let mut uv = [[u0, v1], [u1, v1], [u1, v0], [u0, v0]];
        uv.rotate_right(rotation as usize % 4);
        let corners = [[0., h, 0.], [w, h, 0.], [w, 0., 0.], [0., 0., 0.]];
        let mut vertices = std::array::from_fn(|i| {
            let p = self
                .transform
                .transform_point3(Vec3::from_array(corners[i]));
            [p.x, p.y, p.z, uv[i][0], uv[i][1]]
        });
        if back {
            vertices.reverse();
        }
        let normal = self
            .transform
            .transform_vector3(Vec3::new(0., 0., if back { 1. } else { -1. }))
            .normalize_or_zero()
            .to_array();
        self.polygons.push(Polygon {
            normal,
            vertices,
            cull_back: true,
        });
    }
    fn front(&mut self, u: f32, v: f32, w: f32, h: f32, rot: u8) {
        self.quad(u, v, w, h, rot, false, false);
    }
    fn back(&mut self, u: f32, v: f32, w: f32, h: f32, rot: u8) {
        self.quad(u, v, w, h, rot, true, true);
    }
    fn both(&mut self, u: f32, v: f32, w: f32, h: f32, rot: u8, flip: bool) {
        self.quad(u, v, w, h, rot, flip, false);
        self.quad(u, v, w, h, rot, flip, true);
    }
    fn ear(&mut self, left: bool) {
        self.front(if left { 32. } else { 24. }, 0., 8., 8., 0);
        self.back(56., if left { 36. } else { 28. }, 8., 8., 1);
    }
}
fn ears(m: &mut Mesh, f: &Features, forward: f32) {
    let z = match f.anchor {
        1 => 0.,
        2 => 8.,
        _ => 4.,
    };
    match f.ears {
        1 | 4 => {
            m.t(0., 0., z);
            m.push();
            m.t(-4., -16., 0.);
            m.front(24., 0., 16., 8., 0);
            m.back(56., 28., 16., 8., 1);
            m.pop();
            if f.ears == 4 {
                m.t(-4., -8., 0.);
                m.front(36., 16., 4., 8., 1);
                m.back(12., 16., 4., 8., 1);
                m.t(12., 0., 0.);
                m.front(36., 32., 4., 8., 1);
                m.back(12., 32., 4., 8., 1);
            }
        }
        2 => {
            m.t(-8., -8., z);
            m.ear(false);
            m.t(16., 0., 0.);
            m.ear(true);
        }
        3 | 7 => {
            m.r(90., 1);
            let (x, y) = if f.ears == 3 {
                (-16., -8.)
            } else {
                match f.anchor {
                    1 => (0., -8.),
                    2 => (-16., -8.),
                    _ => (-8., -16.),
                }
            };
            m.t(x, y, 0.);
            m.ear(false);
            m.r(180., 1);
            m.t(-8., 0., -8.);
            m.ear(true);
        }
        5 => {
            for left in [false, true] {
                m.push();
                m.r(if left { -90. } else { 90. }, 1);
                m.t(
                    if left { 0. } else { -8. },
                    -7.,
                    if left { -8. } else { 0. },
                );
                m.r(-30., 0);
                m.ear(left);
                m.pop();
            }
        }
        6 | 9 => {
            m.t(4., if f.ears == 9 { -24. } else { -16. }, z);
            for left in [false, true] {
                m.push();
                m.r(if left { -45. } else { 45. }, 1);
                m.t(-4., 0., 0.);
                if f.ears == 9 {
                    m.front(24., 0., 8., 16., 1);
                    m.back(56., 28., 8., 16., 0);
                } else {
                    m.ear(left);
                }
                m.pop();
            }
        }
        8 => {
            m.t(0., -8., z);
            let angle = -6. - forward;
            for i in 0..4 {
                m.r(
                    angle
                        / match i {
                            0 => 3.,
                            2 => 2.,
                            _ => 1.,
                        },
                    0,
                );
                m.t(0., -4., 0.);
                m.front(24. + i as f32 * 4., 0., 8., 4., 1);
                m.back(56., 40. - i as f32 * 4., 8., 4., 0);
            }
        }
        _ => {}
    }
}
fn tail(m: &mut Mesh, f: &Features, age: f32, speed: f32, gliding: bool) {
    if f.tail == 0 {
        return;
    }
    let (mut angle, swing) = match f.tail {
        1 => (30., 40.),
        2 | 5..=8 => (if f.bends[0] != 0. { 90. } else { 80. }, 20.),
        3 => (130., -20.),
        _ => (0., 0.),
    };
    let mut bends = f.bends;
    if f.animate_tail && gliding {
        angle = 0.;
        bends[0] = -30.;
    }
    let animation = if f.animate_tail {
        speed * swing + (age / 12.).sin() * 4.
    } else {
        0.
    };
    m.t(0., -2., 4.);
    m.r(angle + animation, 0);
    if f.tail == 4 {
        m.t(4., 0., 0.);
        m.r(90., 2);
        if bends[0] < 0. {
            m.t(4., 0., 0.);
            m.r(bends[0], 1);
            m.t(-4., 0., 0.);
        }
        m.t(-4., 0., 0.);
        if bends[0] > 0. {
            m.r(bends[0], 1);
        }
        m.r(90., 0);
        bends[0] = 0.;
    }
    let segments = f.segments.clamp(1, 4) as usize;
    let h = (12 / segments) as f32;
    for (i, bend) in bends.iter().enumerate().take(segments) {
        m.r(*bend, 0);
        m.both(56., 16. + i as f32 * h, 8., h, 0, true);
        let ofs = if i > 0 && matches!(f.tail, 6 | 8) {
            4.
        } else {
            0.
        };
        let angles: &[f32] = match f.tail {
            5 | 6 => &[90.],
            7 | 8 => &[45., 90., 135.],
            _ => &[],
        };
        for a in angles {
            m.push();
            m.t(4., 0., 0.);
            m.r(*a, 1);
            m.t(-4., -ofs, 0.);
            m.both(56., 16. + i as f32 * h - ofs, 8., h + ofs, 0, true);
            m.pop();
        }
        m.t(0., h, 0.);
    }
}
fn snout(m: &mut Mesh, f: &Features) {
    let [w, h, d, offset] = f.snout.map(f32::from);
    if w == 0. || h == 0. || d == 0. {
        return;
    }
    m.t((8. - w) / 2., -(offset + h), -d);
    m.both(0., 2., w, h, 0, false);
    m.push();
    m.r(-90., 0);
    m.t(0., -1., 0.);
    m.both(0., 1., w, 1., 0, false);
    for _ in 1..d as u8 {
        m.t(0., -1., 0.);
        m.both(0., 0., w, 1., 0, false);
    }
    m.pop();
    m.push();
    m.t(0., h, 0.);
    m.r(90., 0);
    m.both(0., 2. + h, w, 1., 0, false);
    for _ in 1..d as u8 {
        m.t(0., 1., 0.);
        m.both(0., 3. + h, w, 1., 0, false);
    }
    m.pop();
    m.r(90., 1);
    for side in [0., w] {
        m.push();
        m.t(-1., 0., side);
        m.both(7., 0., 1., h, 0, false);
        for _ in 1..d as u8 {
            m.t(-1., 0., 0.);
            m.both(7., 4., 1., h, 0, false);
        }
        m.pop();
    }
}
fn wings(m: &mut Mesh, f: &Features, age: f32, speed: f32, gliding: bool) {
    let flight = matches!(f.wing_animation, 0 | 3) && gliding;
    let wiggle = if flight {
        -40.
    } else if matches!(f.wing_animation, 0 | 2) {
        ((age + 8.) / 12.).sin() * 2. + speed * 10.
    } else {
        0.
    };
    m.t(2., -14., 4.);
    match f.wings {
        1 | 3 | 4 | 5 => {
            if matches!(f.wings, 1 | 4 | 5) {
                m.push();
                m.r(-120. + wiggle, 1);
                m.both(0., 0., if f.wings == 5 { 10. } else { 20. }, 16., 0, false);
                m.pop();
            }
            if matches!(f.wings, 1 | 3 | 5) {
                m.t(4., 0., 0.);
                m.r(-60. - wiggle, 1);
                m.both(
                    if f.wings == 5 { 10. } else { 0. },
                    0.,
                    if f.wings == 5 { 10. } else { 20. },
                    16.,
                    0,
                    false,
                );
            }
        }
        2 => {
            m.t(2., 0., 0.);
            m.r(-90. + wiggle, 1);
            m.both(0., 0., 20., 16., 0, false);
        }
        6 => {
            m.t(-8., 0., 0.75);
            m.both(0., 0., 20., 16., 0, false);
        }
        _ => {}
    }
}
fn chest(m: &mut Mesh, f: &Features) {
    if f.chest <= 0. {
        return;
    }
    m.t(0., -10., 0.);
    m.r(-f.chest * 45., 0);
    m.both(20., 22., 8., 4., 0, false);
    m.push();
    m.t(0., 4., 0.);
    m.r(90., 0);
    m.both(56., 44., 8., 4., 0, false);
    m.pop();
    m.r(90., 1);
    m.t(-4., 0., 0.01);
    m.both(60., 48., 4., 4., 0, false);
    m.t(0., 0., 7.98);
    m.r(180., 1);
    m.t(-4., 0., 0.);
    m.both(60., 48., 4., 4., 0, true);
}
fn jacket(m: &mut Mesh, f: &Features) {
    if f.chest <= 0. {
        return;
    }
    m.t(0., -10., 0.);
    m.r(-f.chest * 45., 0);
    m.push();
    m.t(4., 2., 0.);
    m.transform *= Mat4::from_scale(Vec3::new(8.5 / 8., 4.5 / 4., 1.));
    m.t(-4., -2., -0.25);
    m.both(0., 48., 4., 4., 0, false);
    m.t(4., 0., 0.);
    m.both(12., 48., 4., 4., 0, false);
    m.pop();
    m.push();
    m.t(0., 4., 0.);
    m.r(90., 0);
    m.t(-0.25, -0.25, -0.25);
    m.transform *= Mat4::from_scale(Vec3::new(8.5 / 8., 4.5 / 4., 1.));
    m.both(28., 48., 8., 4., 0, false);
    m.pop();
    m.r(90., 1);
    m.t(-4., 0., 0.01);
    for side in 0..2 {
        m.push();
        m.t(-0.25, -0.25, -0.25);
        m.transform *= Mat4::from_scale(Vec3::new(4.5 / 4., 4.5 / 4., 1.));
        m.both(48., 48., 4., 4., 0, side == 1);
        m.pop();
        m.t(0., 0., 7.98);
        m.r(180., 1);
        m.t(-4., 0., 0.);
    }
}
fn cape(m: &mut Mesh, motion: [f32; 3]) {
    m.t(4., -12., 5.);
    m.r(6. + motion[1] / 2. + motion[0], 0);
    m.r(motion[2] / 2., 2);
    m.r(180. - motion[2] / 2., 1);
    m.t(-5., 0., 0.);
    m.both(0., 0., 10., 16., 0, false);
    m.push();
    m.t(10., 0., 1.);
    m.r(90., 1);
    m.both(9., 0., 1., 16., 0, true);
    m.r(90., 1);
    m.both(10., 0., 10., 16., 0, false);
    m.t(10., 0., 1.);
    m.r(90., 1);
    m.both(0., 0., 1., 16., 0, true);
    m.pop();
    m.r(90., 0);
    m.both(0., 0., 10., 1., 0, false);
    m.t(0., 0., -16.);
    m.both(0., 15., 10., 1., 0, false);
}
fn skew(m: &mut Mesh, u: f32, v: f32, w: f32, h: f32, x: f32, z: f32) {
    let start = m.polygons.len();
    m.front(u, v, w, h, 0);
    let inverse = m.transform.inverse();
    for (i, vert) in m.polygons[start].vertices.iter_mut().enumerate() {
        let p = inverse.transform_point3(Vec3::new(vert[0], vert[1], vert[2]));
        let sign = if i < 2 { 1. } else { -1. };
        let p = m
            .transform
            .transform_point3(p + Vec3::new(x * sign, 0., z * sign));
        vert[..3].copy_from_slice(&p.to_array());
    }
    let a = Vec3::from_slice(&m.polygons[start].vertices[0][..3]);
    let b = Vec3::from_slice(&m.polygons[start].vertices[1][..3]);
    let c = Vec3::from_slice(&m.polygons[start].vertices[2][..3]);
    m.polygons[start].normal = (b - a).cross(c - a).normalize_or_zero().to_array();
}
fn leg_half(m: &mut Mesh, u: f32, v: f32, g: f32, enabled: bool, bottom: bool, mend: bool) {
    m.push();
    m.t(2., 6., 2.);
    m.transform *= Mat4::from_scale(Vec3::new(
        (4. + g * 2.) / 4.,
        (12. + g * 2.) / 12.,
        (4. + g * 2.) / 4.,
    ));
    m.t(-2., -6., -2.);
    let bend = if enabled { 1. } else { 0. };
    let vo = if bottom { 10. } else { 4. };
    m.push();
    m.t(0., if bottom { -6. } else { -12. }, 0.);
    for (side, offset) in [4., 8., 12., 0.].iter().enumerate() {
        let (x, z) = match side {
            0 => (0., -bend),
            1 => (-bend, 0.),
            2 => (0., bend),
            _ => (bend, 0.),
        };
        m.push();
        if enabled && !mend && bottom {
            m.t(0., 2., 0.);
            skew(m, u + offset, v + vo + 2., 4., 4., x, z);
        } else {
            skew(m, u + offset, v + vo, 4., 6., x, z);
        }
        m.pop();
        if enabled && mend && !bottom {
            m.push();
            m.t(0., 6., 0.);
            skew(
                m,
                u + offset,
                v + vo + if side == 2 { 5. } else { 6. },
                4.,
                2.,
                -x,
                -z,
            );
            m.pop();
        }
        m.r(-90., 1);
        m.t(0., 0., -4.);
    }
    m.pop();
    m.r(90., 0);
    if bottom {
        m.push();
        m.t(0., -bend, 0.);
        m.quad(u + 8., v, 4., 4., 0, false, false);
        m.pop();
    } else {
        m.push();
        m.t(0., bend, 12.);
        m.back(u + 4., v, 4., 4., 0);
        m.pop();
    }
    if enabled && mend && bottom {
        m.t(0., 0., 6.);
        m.front(u + 4., v + vo, 4., 1., 0);
        m.t(0., 4., 0.);
        m.back(u + 12., v + vo, 4., 1., 0);
    }
    m.pop();
}
fn origin(part: &Part) -> Vec3 {
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for p in &part.polygons {
        for v in p.vertices {
            let pos = Vec3::new(v[0], v[1], v[2]);
            min = min.min(pos);
            max = max.max(pos);
        }
    }
    if min.is_finite() {
        Vec3::new(min.x, max.y, min.z)
    } else {
        Vec3::ZERO
    }
}
fn decorate(
    part: &mut Part,
    f: &Features,
    mob: &Mob,
    age: f32,
    speed: f32,
    alpha: f32,
    texture: &str,
) {
    let mut mesh = Mesh::new(origin(part));
    let a = f64::from(alpha);
    let at = [
        mob.previous.x + (mob.position.x - mob.previous.x) * a,
        mob.previous.y + (mob.position.y - mob.previous.y) * a,
        mob.previous.z + (mob.position.z - mob.previous.z) * a,
    ];
    let yaw =
        mob.previous_body_yaw + super::living::wrap(mob.body_yaw - mob.previous_body_yaw) * alpha;
    let dx = mob.extra.avatar.cloak_o[0]
        + (mob.extra.avatar.cloak[0] - mob.extra.avatar.cloak_o[0]) * a
        - at[0];
    let dz = mob.extra.avatar.cloak_o[2]
        + (mob.extra.avatar.cloak[2] - mob.extra.avatar.cloak_o[2]) * a
        - at[2];
    let forward = ((dx * f64::from(yaw.to_radians().sin())
        - dz * f64::from(yaw.to_radians().cos()))
        * 25.) as f32;

    let name = part.name.rsplit('/').next().unwrap_or(&part.name);
    if texture == "displaced" {
        let left = name == "left_leg";
        if left || name == "right_leg" {
            let full = f.legs == 2;
            let enabled = mob.extra.legs.is_none();
            for layer in 0..2 {
                if layer == 1 && mob.extra.hidden_model_parts & (1 << if left { 4 } else { 5 }) != 0
                {
                    continue;
                }
                mesh.push();
                if full && enabled {
                    mesh.t(0., 0., -0.5);
                }
                let (u, v) = if layer == 0 {
                    if left { (16., 48.) } else { (0., 16.) }
                } else {
                    if left { (0., 48.) } else { (0., 32.) }
                };
                if layer == 1 {
                    mesh.t(0., 0.5, 0.);
                }
                let grow = if layer == 1 { 0.25 } else { 0. };
                if full {
                    leg_half(&mut mesh, u, v, grow, enabled, false, true);
                }
                leg_half(&mut mesh, u, v, grow, enabled, true, !full);
                mesh.pop();
            }
        }
    } else if texture == "cape" {
        if name == "body" {
            cape(&mut mesh, mob.extra.avatar.cape(alpha, at, yaw, 0.));
        }
    } else if texture == "wing" {
        if name == "body" {
            wings(&mut mesh, f, age, speed, mob.extra.fall_flying);
        }
    } else if texture == "skin" {
        match name {
            "head" => {
                mesh.push();
                ears(&mut mesh, f, forward.clamp(-80., 80.));
                mesh.pop();
                mesh.push();
                snout(&mut mesh, f);
                mesh.pop();
                if matches!(f.protrusions, 1 | 3) {
                    mesh.push();
                    mesh.t(0., -8., 0.);
                    mesh.r(25., 0);
                    mesh.t(0., -8., 0.);
                    mesh.both(56., 0., 8., 8., 0, false);
                    mesh.pop();
                }
                if matches!(f.protrusions, 4 | 6) {
                    mesh.t(0., -12., 0.);
                    mesh.r(90., 0);
                    mesh.both(56., 0., 8., 8., 0, false);
                }
                if matches!(f.protrusions, 5 | 7) {
                    mesh.t(0., -10., 0.);
                    mesh.r(90., 0);
                    for _ in 0..2 {
                        mesh.t(4., 4., 0.);
                        mesh.r(45., 2);
                        mesh.t(-4., -4., 2.);
                        mesh.both(56., 0., 8., 8., 0, false);
                    }
                }
            }
            "body" => {
                mesh.push();
                tail(
                    &mut mesh,
                    f,
                    age,
                    speed,
                    mob.extra.fall_flying || mob.extra.synced_pose == 3,
                );
                mesh.pop();
                if mob.extra.chest.is_none() {
                    mesh.push();
                    chest(&mut mesh, f);
                    mesh.pop();
                    if mob.extra.hidden_model_parts & 2 == 0 {
                        jacket(&mut mesh, f);
                    }
                }
            }
            "left_leg" | "right_leg" | "left_arm" | "right_arm"
                if matches!(f.protrusions, 2 | 3 | 6 | 7) =>
            {
                let left = name.starts_with("left");
                if name.ends_with("leg") {
                    if mob.extra.feet.is_none() {
                        mesh.t(
                            0.,
                            0.,
                            -4. - match f.legs {
                                1 => 1.,
                                2 => 1.5,
                                _ => 0.,
                            },
                        );
                        mesh.r(90., 0);
                        mesh.both(
                            if left { 16. } else { 0. },
                            if left { 48. } else { 16. },
                            4.,
                            4.,
                            0,
                            true,
                        );
                    }
                } else {
                    mesh.r(90., 1);
                    mesh.t(
                        -4.,
                        0.,
                        if left {
                            if mob.slim { 3. } else { 4. }
                        } else {
                            0.
                        },
                    );
                    mesh.both(
                        if left { 44. } else { 52. },
                        if left { 48. } else { 16. },
                        4.,
                        4.,
                        2,
                        left,
                    );
                }
            }
            _ => {}
        }
    }
    part.polygons = mesh.polygons;
    for child in &mut part.children {
        decorate(child, f, mob, age, speed, alpha, texture);
    }
}
#[allow(clippy::too_many_arguments)]
pub fn draw(
    mob: &Mob,
    alpha: f32,
    base: Mat4,
    atlas: render::Atlas<'_>,
    light: u32,
    data: &MobData,
    visible: bool,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    if !visible
        || !matches!(
            mob.kind.id.as_str(),
            "minecraft:player" | "minecraft:mannequin"
        )
    {
        return;
    }
    let (key, texture) = render::body(mob);
    let Some(info) = skin::get(&texture) else {
        return;
    };
    let model = mob.kind.layer(&key).unwrap_or(&mob.kind.model);
    let state = anim::state(mob, alpha);
    for kind in ["skin", "wing", "cape", "displaced"] {
        if kind == "wing" && info.features.wings == 0 {
            continue;
        }
        if kind == "cape"
            && (!info.features.cape
                || mob.extra.fall_flying
                || mob.extra.hidden_model_parts & 1 != 0
                || mob.extra.chest.as_deref() == Some("minecraft:elytra"))
        {
            continue;
        }
        if kind == "displaced" && info.features.legs == 0 {
            continue;
        }
        let mut extra = (**model).clone();
        decorate(
            &mut extra,
            &info.features,
            mob,
            state.age,
            state.speed,
            alpha,
            kind,
        );
        let mut poses = anim::Poses::new(&extra);
        anim::setup(mob, &state, &mut poses, &data.animations);
        let name = if kind == "skin" {
            texture.to_string()
        } else {
            skin::texture_name(&texture, kind)
        };
        if let Some(uv) = atlas(&name) {
            render::draw(
                &extra,
                base,
                &poses,
                uv,
                u32::MAX,
                light,
                u32::from(mob.hurt_time > 0),
                out,
            );
        }
        if info.features.emissive && kind != "cape" {
            let name = skin::texture_name(&texture, &format!("emissive_{kind}"));
            if let Some(uv) = atlas(&name) {
                let begin = out.len();
                render::draw(&extra, base, &poses, uv, u32::MAX, 0xf0f0, 0, out);
                if kind == "skin" {
                    let mut body_poses = anim::Poses::new(model);
                    anim::setup(mob, &state, &mut body_poses, &data.animations);
                    render::draw(model, base, &body_poses, uv, u32::MAX, 0xf0f0, 0, out);
                }
                for v in &mut out[begin..] {
                    v.normal = [0; 4];
                }
            }
        }
    }
}

pub fn arm(
    arm: &Part,
    mob: &Mob,
    left: bool,
    base: Mat4,
    atlas: render::Atlas<'_>,
    light: u32,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    let Some(texture) = mob.skin.as_deref() else {
        return;
    };
    let Some(info) = skin::get(texture) else {
        return;
    };
    let pose = |p: &Part| {
        let mut v = p.pose;
        v[3] = 0.;
        v[4] = 0.;
        v[5] = if p.name == arm.name {
            if left { -0.1 } else { 0.1 }
        } else {
            0.
        };
        v
    };
    let mut extra = arm.clone();
    decorate(&mut extra, &info.features, mob, 0., 0., 0., "skin");
    if let Some(uv) = atlas(texture) {
        super::render_model(&extra, base, &pose, uv, light, 0, out);
    }
    if info.features.emissive {
        if let Some(uv) = atlas(&skin::texture_name(texture, "emissive_skin")) {
            let begin = out.len();
            super::render_model(&extra, base, &pose, uv, 0xf0f0, 0, out);
            super::render_model(arm, base, &pose, uv, 0xf0f0, 0, out);
            for v in &mut out[begin..] {
                v.normal = [0; 4];
            }
        }
    }
}
