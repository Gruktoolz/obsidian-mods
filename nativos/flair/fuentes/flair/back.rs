use super::umbx_flair_items::{Item, Moves, Paint, Slot, wave};
use super::umbx_flair_model::{Bone, Motion, bone, cube};
use super::umbx_flair_paint::{Canvas, Side, fbm, mix, noise, opaque, palette};

pub const WING_Z: f32 = 4.3;
pub const BACK: f32 = 2.1;
const FEATHERS: [([usize; 2], [usize; 3]); 3] = [([0, 0], [5, 6, 1]), ([12, 0], [4, 8, 1]), ([22, 0], [3, 7, 1])];

pub fn items() -> Vec<Item> {
    vec![
        Item::new("angel_wings", "Angel Wings", Slot::Back, pair(wing(-0.5)), Paint::Still(angel_paint()), Moves::Live(Box::new(|n, t| flap(n, t, 2.0, 0.22)))),
        Item::new("dragon_wings", "Dragon Wings", Slot::Back, pair(wing(-0.4)), Paint::Still(dragon_paint()), Moves::Live(Box::new(|n, t| flap(n, t, 1.5, 0.3)))),
        Item::new("phoenix_wings", "Phoenix Wings", Slot::Back, pair(wing(-0.45)), Paint::Live(Box::new(phoenix_paint)), Moves::Live(Box::new(|n, t| flap(n, t, 2.4, 0.26)))).glowing(),
        Item::new("katana", "Katana", Slot::Back, katana(), Paint::Still(katana_paint()), Moves::Still),
        Item::new("jetpack", "Jetpack", Slot::Back, jetpack(), Paint::Live(Box::new(jetpack_paint)), Moves::Live(Box::new(thrust))),
    ]
}

fn pair(left: Vec<Bone>) -> Vec<Bone> {
    let right: Vec<Bone> = left.iter().map(|b| b.mirrored(&b.name.replace("_l", "_r"))).collect();
    left.into_iter().chain(right).collect()
}

fn flap(name: &str, t: f32, speed: f32, size: f32) -> Motion {
    if !name.starts_with("wing") {
        return Motion::REST;
    }
    let side = if name.contains("_r") { -1.0 } else { 1.0 };
    Motion { rotation: [0.0, side * size * wave(t, speed, 0.0), 0.0], ..Motion::REST }
}

fn wing(spread: f32) -> Vec<Bone> {
    let root = bone("root_l", [0.0; 3], vec![cube([1.0, 1.0, BACK], [1.5, 3.0, WING_Z - BACK], [32.0, 0.0])]);
    let wing = bone(
        "wing_l",
        [1.5, 1.5, WING_Z],
        vec![cube([1.5, 0.0, WING_Z], [5.0, 6.0, 1.0], [0.0, 0.0]), cube([6.5, -3.0, WING_Z], [4.0, 8.0, 1.0], [12.0, 0.0]), cube([10.5, -5.0, WING_Z], [3.0, 7.0, 1.0], [22.0, 0.0])],
    )
    .turned([0.0, spread, 0.0]);
    vec![root, wing]
}

fn feathers(canvas: &mut Canvas, color: &dyn Fn(Side, usize, usize, usize) -> u32) {
    for (uv, size) in FEATHERS {
        canvas.cube(uv, size, |side, x, y, _, h| opaque(color(side, x, y, h)));
    }
    canvas.cube([32, 0], [2, 3, 2], |_, _, y, _, _| opaque(color(Side::Front, 0, y, 9)));
}

fn angel_paint() -> Canvas {
    let mut canvas = Canvas::default();
    feathers(&mut canvas, &|_, x, y, h| if y + 1 == h { 0xDD_EAFF } else if (y + x / 2) % 3 == 0 { 0xCC_D3E6 } else { 0xF4_F6FF });
    canvas
}

fn dragon_paint() -> Canvas {
    let mut canvas = Canvas::default();
    feathers(&mut canvas, &|_, x, y, h| {
        if x % 3 == 0 {
            0x1C_0A28
        } else if y + 1 == h {
            0x8C_3CFF
        } else {
            mix(0x3A_1450, 0x6A_2A90, noise(x as i64, y as i64) * 0.15 + y as f64 * 0.03)
        }
    });
    canvas
}

fn phoenix_paint(frame: u32) -> Canvas {
    let mut canvas = Canvas::default();
    let t = f64::from(frame) * 0.1;
    feathers(&mut canvas, &|_, x, y, h| {
        let heat = (1.0 - y as f64 / h.max(1) as f64) * 0.7 + 0.3 * fbm(x as f64 * 0.5, y as f64 * 0.5 + t * 1.5, 3, 3);
        let base = palette(0x9A_1206, 0xFF_6A10, 0xFF_E070, heat);
        if (y + x / 2) % 3 == 0 { mix(base, 0xFF_F4C0, 0.3) } else { base }
    });
    canvas
}

fn katana() -> Vec<Bone> {
    vec![
        bone(
            "katana",
            [1.0, 6.0, BACK + 0.6],
            vec![
                cube([-0.6, -4.0, BACK], [1.2, 10.0, 1.0], [0.0, 0.0]),
                cube([-0.7, 6.0, BACK], [1.4, 1.0, 1.2], [0.0, 12.0]),
                cube([-1.3, -4.5, BACK], [2.6, 0.5, 1.4], [8.0, 12.0]),
                cube([-0.5, -10.0, BACK + 0.05], [1.0, 5.5, 0.9], [0.0, 16.0]),
                cube([-0.6, -10.6, BACK], [1.2, 0.6, 1.0], [8.0, 16.0]),
            ],
        )
        .turned([0.0, 0.0, 0.6]),
    ]
}

fn katana_paint() -> Canvas {
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [1, 10, 1], |side, _, y, _, _| opaque(if y == 2 || y == 5 { 0xB0_1C20 } else if side == Side::Left { 0x2A_2A30 } else { 0x12_1216 }));
    canvas.cube([0, 12], [1, 1, 1], |_, _, _, _, _| opaque(0xD8_A83A));
    canvas.cube([8, 12], [3, 1, 1], |_, x, _, _, _| opaque(if x % 2 == 0 { 0xE8_BC4C } else { 0xA8_7A22 }));
    canvas.cube([0, 16], [1, 6, 1], |_, x, y, _, _| opaque(if (x + y) % 2 == 0 { 0xF0_ECE0 } else { 0x16_1616 }));
    canvas.cube([8, 16], [1, 1, 1], |_, _, _, _, _| opaque(0xD8_A83A));
    canvas
}

fn jetpack() -> Vec<Bone> {
    let flame = bone("flame_l", [-2.0, 9.5, BACK + 1.25], vec![cube([-2.5, 9.5, BACK + 0.75], [1.0, 2.0, 1.0], [0.0, 24.0])]);
    vec![
        bone(
            "pack",
            [0.0; 3],
            vec![
                cube([-3.5, 1.0, BACK], [3.0, 7.0, 2.5], [0.0, 0.0]),
                cube([0.5, 1.0, BACK], [3.0, 7.0, 2.5], [0.0, 0.0]),
                cube([-0.5, 2.0, BACK], [1.0, 4.0, 1.5], [22.0, 0.0]),
                cube([-2.75, 8.0, BACK + 0.5], [1.5, 1.5, 1.5], [0.0, 12.0]),
                cube([1.25, 8.0, BACK + 0.5], [1.5, 1.5, 1.5], [0.0, 12.0]),
            ],
        ),
        flame.mirrored("flame_r"),
        flame,
    ]
}

fn thrust(name: &str, t: f32) -> Motion {
    if !name.starts_with("flame") {
        return Motion::REST;
    }
    let flicker = 0.75 + 0.25 * wave(t, 23.0, if name.ends_with("_r") { 1.3 } else { 0.0 });
    Motion { scale: [1.0, flicker, 1.0], ..Motion::REST }
}

fn jetpack_paint(frame: u32) -> Canvas {
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [3, 7, 2], |side, x, y, w, _| {
        let rivet = (x == 0 || x + 1 == w) && y % 3 == 1;
        let stripe = y == 2 && side != Side::Top && side != Side::Bottom;
        opaque(if rivet { 0xD8_DCE4 } else if stripe { 0x9B_6BFF } else { mix(0x5A_6070, 0x8A_92A4, x as f64 / w.max(1) as f64) })
    });
    canvas.cube([22, 0], [1, 4, 1], |_, _, _, _, _| opaque(0x3A_3E48));
    canvas.cube([0, 12], [1, 1, 1], |_, _, _, _, _| opaque(0x22_2428));
    canvas.cube([0, 24], [1, 2, 1], |_, _, y, _, h| {
        let heat = y as f64 / h.max(1) as f64 + 0.25 * noise(y as i64, frame as i64);
        opaque(palette(0xFF_F0A0, 0xFF_9A20, 0xE0_3010, heat.min(1.0)))
    });
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::bounds;

    #[test]
    fn back_items_stay_behind_the_body_and_clear_the_head() {
        for item in items() {
            let (lo, hi) = bounds(&item.bones).unwrap();
            assert!(lo[2] >= BACK, "{} {lo:?}", item.key);
            assert!(hi[1] <= 12.0, "{} {hi:?}", item.key);
            if item.key.ends_with("wings") {
                for wing in item.bones.iter().filter(|b| b.name.starts_with("wing")) {
                    for c in &wing.cubes {
                        let inner = c.from[0].abs().min((c.from[0] + c.size[0]).abs());
                        assert!(c.from[2] >= WING_Z && (inner >= 6.0 || c.from[1] >= 0.0), "{} {:?}", item.key, c.from);
                    }
                }
            }
        }
    }

    #[test]
    fn wings_flap_in_opposite_directions_and_flames_flicker() {
        let (l, r) = (flap("wing_l", 0.5, 2.0, 0.2), flap("wing_r", 0.5, 2.0, 0.2));
        assert_eq!(l.rotation[1], -r.rotation[1]);
        assert_ne!(l.rotation[1], 0.0);
        assert_eq!(flap("root_l", 0.5, 2.0, 0.2), Motion::REST);
        assert_ne!(thrust("flame_l", 0.1), thrust("flame_l", 0.13));
    }
}
