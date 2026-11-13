use super::umbx_flair_items::{Item, Moves, Paint, Slot, wave};
use super::umbx_flair_model::{Bone, Motion, bone, cube};
use super::umbx_flair_paint::{Canvas, Side, mix, opaque};

const WHITE: u32 = 0xFF_FFFF;
const BLACK: u32 = 0;

pub fn items() -> Vec<Item> {
    vec![
        Item::new("top_hat", "Top Hat", Slot::Hat, top_hat(), Paint::Still(top_hat_paint()), Moves::Still),
        Item::new("halo", "Halo", Slot::Hat, halo(), Paint::Live(Box::new(halo_paint)), Moves::Live(Box::new(float))).glowing(),
        Item::new("crown", "Crown", Slot::Hat, crown(), Paint::Live(Box::new(crown_paint)), Moves::Still),
        Item::new("devil_horns", "Devil Horns", Slot::Hat, horns(), Paint::Still(horns_paint()), Moves::Still),
    ]
}

fn top_hat() -> Vec<Bone> {
    vec![bone(
        "hat",
        [0.0; 3],
        vec![
            cube([-6.0, -9.0, -6.0], [12.0, 1.0, 12.0], [0.0, 0.0]),
            cube([-4.0, -17.0, -4.0], [8.0, 8.0, 8.0], [0.0, 13.0]),
            cube([-4.0, -10.5, -4.0], [8.0, 1.5, 8.0], [32.0, 13.0]).inflated(0.15),
        ],
    )]
}

fn top_hat_paint() -> Canvas {
    let mut canvas = Canvas::default();
    let felt = |side: Side, x: usize, w: usize| {
        let shade = if side == Side::Top { 0.12 } else { 0.05 * (x as f64 / w.max(1) as f64 * std::f64::consts::PI).sin() };
        opaque(mix(0x16_161C, WHITE, shade))
    };
    canvas.cube([0, 0], [12, 1, 12], |side, x, _, w, _| felt(side, x, w));
    canvas.cube([0, 13], [8, 8, 8], |side, x, _, w, _| felt(side, x, w));
    canvas.cube([32, 13], [8, 1, 8], |side, x, _, w, _| {
        let buckle = side == Side::Front && (3..5).contains(&x);
        opaque(if buckle { 0xF2_C14E } else { mix(0x5B_2DB8, BLACK, 0.1 * (x as f64 / w.max(1) as f64)) })
    });
    canvas
}

fn halo() -> Vec<Bone> {
    vec![bone(
        "ring",
        [0.0, -11.0, 0.0],
        vec![
            cube([-5.0, -11.5, -5.0], [10.0, 1.0, 1.0], [0.0, 0.0]),
            cube([-5.0, -11.5, 4.0], [10.0, 1.0, 1.0], [0.0, 0.0]),
            cube([-5.0, -11.5, -4.0], [1.0, 1.0, 8.0], [0.0, 4.0]),
            cube([4.0, -11.5, -4.0], [1.0, 1.0, 8.0], [0.0, 4.0]),
        ],
    )]
}

fn float(name: &str, t: f32) -> Motion {
    if name == "ring" { Motion { offset: [0.0, 0.6 * wave(t, 2.0, 0.0), 0.0], rotation: [0.0, t * 0.9, 0.0], ..Motion::REST } } else { Motion::REST }
}

fn halo_paint(frame: u32) -> Canvas {
    let mut canvas = Canvas::default();
    let shine = |x: usize| {
        let glint = ((x as f64 - frame as f64 * 0.7).rem_euclid(14.0) - 2.0).abs() < 1.5;
        opaque(if glint { 0xFF_F8D8 } else { 0xFF_D86B })
    };
    canvas.cube([0, 0], [10, 1, 1], |_, x, _, _, _| shine(x));
    canvas.cube([0, 4], [1, 1, 8], |_, x, _, _, _| shine(x + 5));
    canvas
}

fn crown() -> Vec<Bone> {
    let point = |x: f32, z: f32, tall: f32| cube([x, -10.0 - tall, z], [1.0, tall, 1.0], [0.0, 12.0]);
    vec![bone(
        "crown",
        [0.0; 3],
        vec![
            cube([-4.5, -10.0, -4.5], [9.0, 2.0, 9.0], [0.0, 0.0]),
            point(-4.5, -4.5, 2.0),
            point(3.5, -4.5, 2.0),
            point(-4.5, 3.5, 2.0),
            point(3.5, 3.5, 2.0),
            point(-0.5, -4.5, 2.5),
            point(-0.5, 3.5, 2.5),
        ],
    )]
}

fn crown_paint(frame: u32) -> Canvas {
    let mut canvas = Canvas::default();
    let glint = |x: usize| ((x as f64 - frame as f64 * 0.5).rem_euclid(20.0) - 1.0).abs() < 1.0;
    canvas.cube([0, 0], [9, 2, 9], |side, x, y, _, _| {
        let gem = side != Side::Top && side != Side::Bottom && y == 0 && x % 4 == 2;
        let gem_color = if x == 6 { 0xD8_1B3C } else { 0x2E_6BFF };
        opaque(if gem { gem_color } else if glint(x) { 0xFF_F2B0 } else { mix(0xF2_B632, 0x9C_6A10, y as f64 * 0.4) })
    });
    canvas.cube([0, 12], [1, 3, 1], |_, _, y, _, _| opaque(if y == 0 { 0xFF_F2B0 } else { 0xF2_B632 }));
    canvas
}

fn horns() -> Vec<Bone> {
    let horn = bone(
        "horn_l",
        [-2.5, -8.0, -1.0],
        vec![
            cube([-3.5, -10.0, -2.0], [2.0, 2.0, 2.0], [0.0, 0.0]),
            cube([-3.25, -12.0, -1.75], [1.5, 2.0, 1.5], [0.0, 5.0]),
            cube([-3.0, -13.5, -1.5], [1.0, 1.5, 1.0], [8.0, 0.0]),
        ],
    )
    .turned([-0.2, 0.0, -0.35]);
    vec![horn.mirrored("horn_r"), horn]
}

fn horns_paint() -> Canvas {
    let mut canvas = Canvas::default();
    canvas.rect([0, 0, 16, 10], |_, y| opaque(mix(0xE0_301E, 0x5A_0606, y as f64 / 9.0)));
    canvas.rect([0, 0, 8, 4], |_, y| opaque(mix(0x9A_1010, 0x5A_0606, y as f64 / 4.0)));
    canvas.rect([8, 0, 4, 3], |_, _| opaque(0xFF_6A3A));
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::bounds;

    #[test]
    fn hats_sit_on_the_head() {
        for item in items() {
            let (lo, hi) = bounds(&item.bones).unwrap();
            assert!(lo[1] >= -18.0 && hi[1] <= 1.0, "{}", item.key);
            assert!(lo[0] >= -7.0 && hi[0] <= 7.0, "{}", item.key);
            assert_eq!(item.canvases(0)[0].0.len(), 64 * 64 * 4);
        }
    }

    #[test]
    fn the_halo_floats_and_spins() {
        assert_ne!(float("ring", 0.0).rotation, float("ring", 1.0).rotation);
        assert_eq!(float("other", 1.0), Motion::REST);
    }
}
