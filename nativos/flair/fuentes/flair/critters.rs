use super::umbx_flair_items::{Item, Moves, Paint, Slot, wave};
use super::umbx_flair_model::{Bone, Motion, bone, cube};
use super::umbx_flair_paint::{Canvas, Side, mix, opaque};
use super::umbx_flair_pets::perched;

pub const SEAT: [f32; 3] = [7.5, 0.0, 0.0];
const BLINK_FRAMES: u32 = 40;

pub fn items() -> Vec<Item> {
    vec![
        Item::new("fox", "Fox", Slot::Pet, perched(fox(), SEAT), Paint::Still(fox_paint()), Moves::Live(Box::new(fox_moves))),
        Item::new("owl", "Owl", Slot::Pet, perched(owl(), SEAT), Paint::Live(Box::new(owl_paint)), Moves::Live(Box::new(owl_moves))),
        Item::new("slime", "Slime", Slot::Pet, perched(slime(), SEAT), Paint::Still(slime_paint()), Moves::Live(Box::new(slime_moves))),
        Item::new("bee", "Bee", Slot::Pet, perched(bee(), SEAT), Paint::Still(bee_paint()), Moves::Live(Box::new(bee_moves))),
    ]
}

fn fox() -> Vec<Bone> {
    vec![
        bone("body", [0.0; 3], vec![cube([-1.5, -4.0, -1.0], [3.0, 4.0, 3.0], [0.0, 0.0])]),
        bone(
            "head",
            [0.0, -4.0, -0.5],
            vec![
                cube([-2.0, -7.0, -2.5], [4.0, 3.0, 3.0], [0.0, 8.0]),
                cube([-1.0, -5.5, -3.5], [2.0, 1.5, 1.0], [14.0, 8.0]),
                cube([-2.0, -8.5, -1.5], [1.0, 1.5, 1.0], [20.0, 8.0]),
                cube([1.0, -8.5, -1.5], [1.0, 1.5, 1.0], [20.0, 8.0]),
            ],
        ),
        bone("tail", [0.0, -1.0, 2.0], vec![cube([-1.0, -2.0, 2.0], [2.0, 2.0, 5.0], [0.0, 15.0])]).turned([-0.25, 0.0, 0.0]),
    ]
}

fn fox_moves(name: &str, t: f32) -> Motion {
    match name {
        "head" => Motion { rotation: [0.08 * wave(t, 0.8, 0.0), 0.45 * wave(t, 0.5, 1.0), 0.0], ..Motion::REST },
        "tail" => Motion { rotation: [0.1 * wave(t, 1.2, 0.0), 0.55 * wave(t, 1.8, 0.0), 0.0], ..Motion::REST },
        _ => Motion::REST,
    }
}

fn fox_paint() -> Canvas {
    let (fur, white, dark) = (0xE3_7A2C, 0xF6_F0E6, 0x2A_1A12);
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [3, 4, 3], |side, _, y, _, _| opaque(if side == Side::Front && y >= 1 { white } else { fur }));
    canvas.cube([0, 8], [4, 3, 3], |side, x, y, _, _| match (side, x, y) {
        (Side::Front, 0 | 3, 1) => opaque(dark),
        (Side::Front, _, 2) => opaque(white),
        _ => opaque(fur),
    });
    canvas.cube([14, 8], [2, 2, 1], |side, _, _, _, _| opaque(if side == Side::Front { dark } else { white }));
    canvas.cube([20, 8], [1, 2, 1], |_, _, y, _, _| opaque(if y == 0 { dark } else { fur }));
    canvas.cube([0, 15], [2, 2, 5], |side, x, _, w, _| opaque(if (side == Side::Back) || (matches!(side, Side::Left | Side::Right | Side::Top | Side::Bottom) && x + 2 >= w) { white } else { fur }));
    canvas
}

fn owl() -> Vec<Bone> {
    vec![
        bone("body", [0.0; 3], vec![cube([-1.5, -5.0, -1.5], [3.0, 5.0, 3.0], [0.0, 0.0])]),
        bone(
            "head",
            [0.0, -5.0, 0.0],
            vec![cube([-2.0, -8.0, -2.0], [4.0, 3.0, 4.0], [0.0, 8.0]), cube([-2.0, -8.75, -1.0], [1.0, 0.75, 1.0], [16.0, 8.0]), cube([1.0, -8.75, -1.0], [1.0, 0.75, 1.0], [16.0, 8.0])],
        ),
    ]
}

fn owl_moves(name: &str, t: f32) -> Motion {
    match name {
        "head" => Motion { rotation: [0.0, 1.1 * (wave(t, 0.35, 0.0) * 1.6).clamp(-1.0, 1.0), 0.06 * wave(t, 1.3, 0.0)], ..Motion::REST },
        "body" => Motion { scale: [1.0, 1.0 + 0.03 * wave(t, 1.6, 0.0), 1.0], ..Motion::REST },
        _ => Motion::REST,
    }
}

fn owl_paint(frame: u32) -> Canvas {
    let (feather, belly, beak) = (0x7A_5634, 0xD8_C2A0, 0xE8_A030);
    let blink = frame % BLINK_FRAMES < 2;
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [3, 5, 3], |side, x, y, _, _| {
        let spot = (x + y) % 2 == 0;
        opaque(if side == Side::Front { if spot { belly } else { mix(belly, feather, 0.3) } } else { mix(feather, 0, if spot { 0.1 } else { 0.0 }) })
    });
    canvas.cube([0, 8], [4, 3, 4], |side, x, y, _, _| match (side, x, y) {
        (Side::Front, 0 | 3, 1) => opaque(if blink { feather } else { 0xF5_D547 }),
        (Side::Front, 1 | 2, 2) => opaque(beak),
        (Side::Front, _, _) => opaque(belly),
        _ => opaque(feather),
    });
    canvas.cube([16, 8], [1, 1, 1], |_, _, _, _, _| opaque(mix(feather, 0, 0.25)));
    canvas
}

fn slime() -> Vec<Bone> {
    vec![bone("jelly", [0.0; 3], vec![cube([-2.0, -4.0, -2.0], [4.0, 4.0, 4.0], [0.0, 0.0]), cube([-1.0, -3.0, -1.0], [2.0, 2.0, 2.0], [0.0, 10.0])])]
}

fn slime_moves(name: &str, t: f32) -> Motion {
    if name != "jelly" {
        return Motion::REST;
    }
    let hop = (t * 2.2).sin().abs();
    let squash = 1.0 - 0.18 * (1.0 - hop);
    Motion { offset: [0.0, -1.5 * hop, 0.0], scale: [2.0 - squash, squash, 2.0 - squash], ..Motion::REST }
}

fn slime_paint() -> Canvas {
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [4, 4, 4], |side, x, y, _, _| match (side, x, y) {
        (Side::Front, 0 | 3, 1) => opaque(0x1E_4A1C),
        (Side::Front, 1 | 2, 3) => opaque(0x2E_6A2A),
        _ if x == 0 || y == 0 => opaque(0x9C_F08C),
        _ => opaque(0x6C_D45E),
    });
    canvas.cube([0, 10], [2, 2, 2], |_, _, _, _, _| opaque(0x4A_A83E));
    canvas
}

fn bee() -> Vec<Bone> {
    let wing = bone("wing_l", [0.5, -3.0, 0.0], vec![cube([0.5, -3.25, -0.5], [2.5, 0.25, 2.0], [0.0, 10.0])]);
    vec![
        bone("hover", [0.0; 3], Vec::new()).with(vec![
            bone("body", [0.0; 3], vec![cube([-1.0, -3.0, -1.5], [2.0, 2.0, 3.0], [0.0, 0.0]), cube([-0.25, -2.25, 1.5], [0.5, 0.5, 0.75], [12.0, 0.0])]),
            wing.mirrored("wing_r"),
            wing,
        ]),
    ]
}

fn bee_moves(name: &str, t: f32) -> Motion {
    match name {
        "hover" => Motion { offset: [0.6 * wave(t, 0.9, 0.0), -2.0 + 0.6 * wave(t, 2.7, 0.0), 0.6 * wave(t, 0.7, 1.0)], rotation: [0.0, 0.4 * wave(t, 0.5, 0.0), 0.0], ..Motion::REST },
        "wing_l" => Motion { rotation: [0.0, 0.0, -0.5 * wave(t, 40.0, 0.0)], ..Motion::REST },
        "wing_r" => Motion { rotation: [0.0, 0.0, 0.5 * wave(t, 40.0, 0.0)], ..Motion::REST },
        _ => Motion::REST,
    }
}

fn bee_paint() -> Canvas {
    let mut canvas = Canvas::default();
    canvas.cube([0, 0], [2, 2, 3], |side, x, y, _, _| match (side, x, y) {
        (Side::Front, _, 0) => opaque(0x1A_1410),
        (Side::Left | Side::Right | Side::Top | Side::Bottom, x, _) if x % 2 == 1 => opaque(0x1A_1410),
        _ => opaque(0xF2_C230),
    });
    canvas.cube([12, 0], [1, 1, 1], |_, _, _, _, _| opaque(0x1A_1410));
    canvas.cube([0, 10], [3, 1, 2], |_, _, _, _, _| opaque(0xE8_F4FF));
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::bounds;

    #[test]
    fn critters_sit_beside_the_head_and_move() {
        for item in items() {
            let (lo, hi) = bounds(&item.bones).unwrap();
            assert!(lo[0] >= 4.4 && hi[1] <= 1e-4, "{} {lo:?}", item.key);
            let names = ["head", "tail", "jelly", "hover"];
            assert!(names.iter().any(|n| item.motion(n, 0.2) != item.motion(n, 0.9)), "{}", item.key);
        }
        assert_ne!(owl_paint(0), owl_paint(5));
    }
}
