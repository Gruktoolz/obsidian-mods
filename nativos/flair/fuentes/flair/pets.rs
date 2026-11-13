use super::umbx_flair_items::{Item, Moves, Paint, Slot, wave};
use super::umbx_flair_model::{Bone, Motion, bone, cube};
use super::umbx_flair_paint::{Canvas, Side, mix, noise, opaque};

pub const CAT_SEAT: [f32; 3] = [7.5, 0.0, 0.0];
pub const DRAGON_SEAT: [f32; 3] = [7.5, 0.0, 0.0];

struct Coat {
    fur: u32,
    stripe: u32,
    eyes: u32,
}

pub fn items() -> Vec<Item> {
    let orange = Coat { fur: 0xE8_913A, stripe: 0xB0_622A, eyes: 0x5B_D15B };
    let black = Coat { fur: 0x22_222A, stripe: 0x1A_1A20, eyes: 0xF5_D547 };
    vec![
        Item::new("cat", "Cat", Slot::Pet, perched(cat(), CAT_SEAT), Paint::Still(cat_paint(&orange)), Moves::Live(Box::new(cat_moves))),
        Item::new("black_cat", "Black Cat", Slot::Pet, perched(cat(), CAT_SEAT), Paint::Still(cat_paint(&black)), Moves::Live(Box::new(cat_moves))),
        Item::new("mini_dragon", "Mini Dragon", Slot::Pet, perched(dragon(), DRAGON_SEAT), Paint::Still(dragon_paint(0x3F_AE5A, 0x8B_E0A0, 0xF5_D547)), Moves::Live(Box::new(dragon_moves))),
        Item::new("void_dragon", "Void Dragon", Slot::Pet, perched(dragon(), DRAGON_SEAT), Paint::Still(dragon_paint(0x2A_1440, 0x8C_3CFF, 0xE8_80FF)), Moves::Live(Box::new(dragon_moves))).glowing(),
    ]
}

pub fn perched(bones: Vec<Bone>, seat: [f32; 3]) -> Vec<Bone> {
    vec![bone("pet", [0.0; 3], Vec::new()).with(bones).shifted(seat)]
}

fn cat() -> Vec<Bone> {
    vec![
        bone("body", [0.0; 3], vec![cube([-1.5, -4.0, -1.0], [3.0, 4.0, 3.0], [0.0, 0.0])]),
        bone(
            "head",
            [0.0, -4.0, -0.5],
            vec![
                cube([-2.0, -7.0, -2.5], [4.0, 3.0, 3.0], [0.0, 8.0]),
                cube([-2.0, -8.0, -1.5], [1.0, 1.0, 1.0], [14.0, 8.0]),
                cube([1.0, -8.0, -1.5], [1.0, 1.0, 1.0], [14.0, 8.0]),
                cube([-1.0, -5.5, -3.0], [2.0, 1.0, 0.5], [14.0, 11.0]),
            ],
        ),
        bone("tail", [0.0, -0.5, 2.0], vec![cube([-0.5, -1.0, 2.0], [1.0, 1.0, 4.0], [0.0, 15.0])]).turned([-0.3, 0.0, 0.0]),
    ]
}

fn cat_moves(name: &str, t: f32) -> Motion {
    match name {
        "head" => Motion { rotation: [0.1 * wave(t, 0.9, 0.0), 0.5 * wave(t, 0.6, 0.0), 0.0], ..Motion::REST },
        "tail" => Motion { rotation: [0.15 * wave(t, 1.3, 1.0), 0.7 * wave(t, 2.2, 0.0), 0.0], ..Motion::REST },
        _ => Motion::REST,
    }
}

fn cat_paint(coat: &Coat) -> Canvas {
    let mut canvas = Canvas::default();
    let fur = |y: usize| opaque(if y % 2 == 1 { coat.stripe } else { coat.fur });
    canvas.cube([0, 0], [3, 4, 3], |side, _, y, _, _| if side == Side::Front && y >= 2 { opaque(mix(coat.fur, 0xFF_FFFF, 0.7)) } else { fur(y) });
    canvas.cube([0, 8], [4, 3, 3], |side, x, y, _, _| match (side, x, y) {
        (Side::Front, 0 | 3, 1) => opaque(coat.eyes),
        (Side::Front, 1 | 2, 2) => opaque(0xF2_8FA8),
        _ => fur(y + 1),
    });
    canvas.cube([14, 8], [1, 1, 1], |_, _, _, _, _| opaque(mix(coat.fur, 0xF2_8FA8, 0.4)));
    canvas.cube([14, 11], [2, 1, 1], |_, _, _, _, _| opaque(mix(coat.fur, 0xFF_FFFF, 0.75)));
    canvas.cube([0, 15], [1, 1, 4], |_, x, _, _, _| fur(x));
    canvas
}

fn dragon() -> Vec<Bone> {
    let wing = bone("wing_l", [1.5, -4.0, 0.0], vec![cube([1.5, -7.0, -0.5], [0.5, 3.0, 5.0], [0.0, 9.0])]).turned([0.0, 0.0, 0.3]);
    vec![
        bone("body", [0.0; 3], vec![cube([-1.5, -4.0, -2.0], [3.0, 3.0, 5.0], [0.0, 0.0])]),
        bone(
            "head",
            [0.0, -4.5, -2.0],
            vec![
                cube([-1.5, -7.0, -5.0], [3.0, 3.0, 3.0], [16.0, 0.0]),
                cube([-1.0, -6.0, -7.0], [2.0, 1.5, 2.0], [28.0, 0.0]),
                cube([-1.5, -8.0, -3.0], [1.0, 1.0, 1.0], [36.0, 0.0]),
                cube([0.5, -8.0, -3.0], [1.0, 1.0, 1.0], [36.0, 0.0]),
            ],
        ),
        bone("tail", [0.0, -2.5, 3.0], vec![cube([-0.5, -3.0, 3.0], [1.0, 1.0, 5.0], [0.0, 18.0])]),
        wing.mirrored("wing_r"),
        wing,
    ]
}

fn dragon_moves(name: &str, t: f32) -> Motion {
    match name {
        "pet" => Motion { offset: [0.0, -1.5 + 0.5 * wave(t, 3.0, 0.0), 0.0], ..Motion::REST },
        "wing_l" => Motion { rotation: [0.0, 0.0, 0.3 * wave(t, 6.0, 0.0)], ..Motion::REST },
        "wing_r" => Motion { rotation: [0.0, 0.0, -0.3 * wave(t, 6.0, 0.0)], ..Motion::REST },
        "tail" => Motion { rotation: [0.2, 0.4 * wave(t, 2.0, 0.0), 0.0], ..Motion::REST },
        "head" => Motion { rotation: [0.15 * wave(t, 1.1, 0.0), 0.3 * wave(t, 0.5, 0.0), 0.0], ..Motion::REST },
        _ => Motion::REST,
    }
}

fn dragon_paint(scales: u32, membrane: u32, eyes: u32) -> Canvas {
    let mut canvas = Canvas::default();
    let skin = |side: Side, x: usize, y: usize| {
        let base = if side == Side::Bottom { mix(scales, 0xF0_E6A0, 0.5) } else { scales };
        opaque(if noise(x as i64, y as i64) > 0.7 { mix(base, 0, 0.25) } else { base })
    };
    canvas.cube([0, 0], [3, 3, 5], |side, x, y, _, _| skin(side, x, y));
    canvas.cube([16, 0], [3, 3, 3], |side, x, y, _, _| if side == Side::Front && y == 1 && x != 1 { opaque(eyes) } else { skin(side, x, y) });
    canvas.cube([28, 0], [2, 2, 2], |side, x, y, _, _| skin(side, x, y + 3));
    canvas.cube([36, 0], [1, 1, 1], |_, _, _, _, _| opaque(0xF0_E6C8));
    canvas.cube([0, 9], [1, 3, 5], |_, x, _, _, _| opaque(if x % 2 == 0 { mix(membrane, 0, 0.35) } else { membrane }));
    canvas.cube([0, 18], [1, 1, 5], |side, x, y, _, _| skin(side, x, y));
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::bounds;

    #[test]
    fn pets_perch_on_the_left_shoulder() {
        for item in items() {
            let (lo, hi) = bounds(&item.bones).unwrap();
            assert!(lo[0] >= 5.0 && hi[1] <= 1e-4, "{}", item.key);
            assert!([CAT_SEAT, DRAGON_SEAT].contains(&item.bones[0].pivot), "{}", item.key);
        }
    }

    #[test]
    fn pets_move() {
        assert_ne!(cat_moves("tail", 0.2), cat_moves("tail", 0.7));
        assert_ne!(dragon_moves("wing_l", 0.1).rotation[2], dragon_moves("wing_r", 0.1).rotation[2]);
        assert_eq!(cat_moves("body", 0.4), Motion::REST);
    }
}
