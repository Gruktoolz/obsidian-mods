use super::umbx_flair_items::{Item, Moves, Paint, Slot, wave};
use super::umbx_flair_model::{Bone, Cube, Motion, bone, only};
use super::umbx_flair_paint::{Canvas, Mask, fbm, mix, noise, opaque};
use std::sync::Arc;

pub const STRIP_W: usize = 512;
pub const STRIP_H: usize = 40;
pub const TILES: usize = 8;
const TILE: usize = 64;
const TOP: f32 = -8.25;
const TALL: f32 = 3.0;
const EDGE: f32 = 4.25;
const TRIM: usize = 3;
const SCROLL: usize = 2;
const SPARE: [usize; 4] = [0, 44, 40, 20];
const TAIL_RECT: [f32; 4] = [0.0, 44.0, 16.0, 64.0];
const KNOT_RECT: [f32; 4] = [16.0, 44.0, 40.0, 64.0];
const WHITE: u32 = 0xFF_FFFF;
const BLACK: u32 = 0;
const GLYPHS: [[&str; 5]; 4] = [
    ["..#..", "#.#.#", "..#..", ".#.#.", "#...#"],
    ["..#..", "#####", ".###.", "#.#.#", "..#.."],
    ["#####", "..#..", ".###.", "..#..", "#####"],
    ["..#..", "#####", "#.#.#", "#####", "..#.."],
];

pub type Writer<'a> = &'a dyn Fn(&str) -> Mask;

#[derive(Clone, Copy, PartialEq)]
enum Look {
    Shinobi,
    Crimson,
    Shadow,
    Dragon,
    Sakura,
    Galaxy,
    Neon,
}

const STYLES: [(&str, &str, &str, Look, bool); 7] = [
    ("shinobi_bandana", "Shinobi", "忍者 · 影 · 忍 · 侍 · ", Look::Shinobi, false),
    ("crimson_bandana", "Crimson", "侍 · 武士道 · 名誉 · ", Look::Crimson, false),
    ("shadow_bandana", "Shadow", "影 · 闇 · 夜 · 影 · ", Look::Shadow, true),
    ("dragon_bandana", "Dragon", "龍 · 炎 · 力 · 龍 · ", Look::Dragon, true),
    ("sakura_bandana", "Sakura", "桜 · 花 · 春 · 夢 · ", Look::Sakura, false),
    ("galaxy_bandana", "Galaxy", "星 · 宇宙 · 月 · ", Look::Galaxy, true),
    ("neon_bandana", "Neon", "未来 · 電脳 · 光 · ", Look::Neon, true),
];

pub fn items(write: Writer) -> Vec<Item> {
    STYLES
        .iter()
        .map(|(key, name, text, look, glow)| {
            let ink = Arc::new(write(text));
            let soft = Arc::new(ink.blurred(2));
            let look = *look;
            let paint = Paint::Layers { count: TILES, paint: Box::new(move |frame| layers(look, &ink, &soft, frame)) };
            let item = Item::new(key, name, Slot::Hat, model(), paint, Moves::Live(Box::new(tails)));
            if *glow { item.glowing() } else { item }
        })
        .collect()
}

pub fn fallback(_: &str) -> Mask {
    let mut mask = Mask::new(STRIP_W, STRIP_H);
    let scale = 5;
    let top = (STRIP_H - 5 * scale) / 2;
    for cell in 0..STRIP_W / 40 {
        let glyph = GLYPHS[cell % GLYPHS.len()];
        for (row, line) in glyph.iter().enumerate() {
            for (column, c) in line.bytes().enumerate() {
                if c == b'#' {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            mask.alpha[(top + row * scale + dy) * STRIP_W + cell * 40 + 8 + column * scale + dx] = 255;
                        }
                    }
                }
            }
        }
    }
    mask
}

fn model() -> Vec<Bone> {
    let rect = [0.0, 0.0, 64.0, STRIP_H as f32];
    let wide = 2.0 * EDGE;
    let tile = |layer: usize, from: [f32; 3], size: [f32; 3], face: usize| Cube::faced(from, size, only(face, rect), layer);
    let band = vec![
        tile(0, [-EDGE, TOP, -EDGE], [EDGE, TALL, wide], 3),
        tile(1, [0.0, TOP, -EDGE], [EDGE, TALL, wide], 3),
        tile(2, [-EDGE, TOP, -EDGE], [wide, TALL, EDGE], 4),
        tile(3, [-EDGE, TOP, 0.0], [wide, TALL, EDGE], 4),
        tile(4, [0.0, TOP, -EDGE], [EDGE, TALL, wide], 5),
        tile(5, [-EDGE, TOP, -EDGE], [EDGE, TALL, wide], 5),
        tile(6, [-EDGE, TOP, 0.0], [wide, TALL, EDGE], 2),
        tile(7, [-EDGE, TOP, -EDGE], [wide, TALL, EDGE], 2),
    ];
    let all = |r: [f32; 4]| [r; 6];
    let low = bone("tail_low_l", [-0.5, -4.75, 4.6], vec![Cube::faced([-1.0, -4.75, 4.4], [1.0, 2.0, 0.4], all(TAIL_RECT), 0)]).turned([0.25, 0.0, 0.0]);
    let tail = bone("tail_l", [-0.5, -6.75, 4.6], vec![Cube::faced([-1.0, -6.75, 4.4], [1.0, 2.0, 0.4], all(TAIL_RECT), 0)]).turned([0.4, 0.0, 0.15]).with(vec![low]);
    vec![
        bone("band", [0.0; 3], band),
        bone("knot", [0.0, -7.0, 4.25], vec![Cube::faced([-0.75, -7.75, 4.25], [1.5, 1.5, 0.75], all(KNOT_RECT), 0)]),
        tail.mirrored("tail_r"),
        tail,
    ]
}

fn tails(name: &str, t: f32) -> Motion {
    if !name.starts_with("tail_") {
        return Motion::REST;
    }
    let right = name.starts_with("tail_r");
    let low = name.contains("low");
    let phase = if low { 0.9 } else { 0.0 } + if right { 0.5 } else { 0.0 };
    let size = if low { 0.3 } else { 0.15 };
    let side = if right { 1.0 } else { -1.0 };
    Motion { rotation: [size * wave(t, 3.2, phase), 0.0, side * 0.08 * wave(t, 2.1, phase)], ..Motion::REST }
}

fn cloth(look: Look, s: f64, y: f64, t: f64) -> u32 {
    let weave = if (s as i64 + y as i64) % 4 == 0 { 0.06 } else { 0.0 };
    let shade = fbm(s * 0.05, y * 0.08, 3, 3) * 0.12;
    let base = match look {
        Look::Shinobi => 0xF0_EEE6,
        Look::Crimson => mix(0x8E_0E22, 0xC0_1834, y / STRIP_H as f64),
        Look::Shadow | Look::Dragon => 0x10_1014,
        Look::Sakura => mix(0xF6_B6CC, 0xEC_94B4, y / STRIP_H as f64),
        Look::Galaxy => {
            let n = fbm(s * 0.03 + t * 0.05, y * 0.06, 9, 4);
            let sky = mix(0x0A_0820, mix(0x4A_1C8C, 0x12_7C8C, fbm(s * 0.02, y * 0.04, 4, 2)), (n - 0.35).max(0.0) * 1.6);
            let star = noise(s as i64, y as i64 + 5) > 0.985 && (t * 3.0 + noise(y as i64, s as i64) * 20.0).sin() > -0.2;
            if star { WHITE } else { sky }
        }
        Look::Neon => {
            if (y as i64) % 3 == 0 { 0x05_060A } else { 0x0A_0C14 }
        }
    };
    let sweep = (-((s - (t * 60.0).rem_euclid(STRIP_W as f64 + 200.0) + 100.0) / 30.0).powi(2)).exp() * 0.18;
    mix(mix(base, BLACK, weave + shade), WHITE, if look == Look::Neon { 0.0 } else { sweep })
}

fn ink(look: Look, s: f64, y: f64, t: f64) -> u32 {
    match look {
        Look::Shinobi => mix(0x0E_0E0E, 0x30_2A28, fbm(s * 0.2, y * 0.2, 6, 2)),
        Look::Crimson => 0x14_0406,
        Look::Shadow => mix(0xB0_1020, 0xFF_3448, 0.5 + 0.5 * (t * 2.4).sin()),
        Look::Dragon => mix(0xFF_B020, 0xFF_4A10, fbm(s * 0.04 - t * 0.6, y * 0.08, 2, 3)),
        Look::Sakura => WHITE,
        Look::Galaxy => mix(WHITE, 0xC8_E4FF, 0.5 + 0.5 * (s * 0.05 + t).sin()),
        Look::Neon => {
            let flicker = if noise((t * 8.0) as i64, 3) > 0.94 { 0.45 } else { 1.0 };
            mix(BLACK, 0x3C_E8FF, flicker)
        }
    }
}

fn trim(look: Look, s: f64, t: f64) -> u32 {
    match look {
        Look::Shinobi => 0x1A_1A1A,
        Look::Crimson | Look::Dragon => {
            let glint = (-((s - (t * 90.0).rem_euclid(STRIP_W as f64)) / 12.0).powi(2)).exp();
            mix(0xC8_942E, 0xFF_F0B8, glint)
        }
        Look::Shadow => 0x3A_0A12,
        Look::Sakura => 0xC8_5F8A,
        Look::Galaxy => 0x8C_6CFF,
        Look::Neon => 0x3C_E8FF,
    }
}

fn petal(s: f64, y: f64, t: f64) -> f64 {
    let (u, v) = (s + t * 14.0, y - t * 6.0);
    let (cell_u, cell_v) = ((u / 16.0).floor(), (v / 16.0).floor());
    if noise(cell_u as i64, cell_v as i64) < 0.75 {
        return 0.0;
    }
    let (du, dv) = ((u / 16.0).fract() - 0.5, (v / 16.0).fract() - 0.5);
    (1.0 - (du * du * 9.0 + dv * dv * 22.0)).max(0.0)
}

fn shade(look: Look, ink_mask: &Mask, soft: &Mask, s: usize, y: usize, frame: u32) -> u32 {
    let t = f64::from(frame) * 0.1;
    let (sf, yf) = (s as f64, y as f64);
    if y < TRIM || y + TRIM >= STRIP_H {
        return trim(look, sf, t);
    }
    if y == TRIM || y + TRIM + 1 == STRIP_H {
        return mix(cloth(look, sf, yf, t), BLACK, 0.35);
    }
    let at = (s + frame as usize * SCROLL) % STRIP_W;
    let mut color = cloth(look, sf, yf, t);
    if look == Look::Sakura {
        color = mix(color, 0xFF_E8F0, petal(sf, yf, t));
    }
    let glow = matches!(look, Look::Shadow | Look::Dragon | Look::Galaxy | Look::Neon);
    if glow {
        let halo = soft.at(at, y) * (0.6 + 0.25 * (t * 2.0).sin());
        color = mix(color, ink(look, sf, yf, t), halo * 0.8);
    }
    mix(color, ink(look, sf, yf, t), ink_mask.at(at, y))
}

fn layers(look: Look, ink_mask: &Mask, soft: &Mask, frame: u32) -> Vec<Canvas> {
    let t = f64::from(frame) * 0.1;
    (0..TILES)
        .map(|tile| {
            let mut canvas = Canvas::default();
            for y in 0..STRIP_H {
                for x in 0..TILE {
                    canvas.put(x, y, opaque(shade(look, ink_mask, soft, tile * TILE + x, y, frame)));
                }
            }
            if tile == 0 {
                let [sx, sy, sw, sh] = SPARE;
                canvas.rect([sx, sy, sw, sh], |x, y| {
                    let edge = x % 16 < 2 || y + 2 >= sh;
                    opaque(if edge { trim(look, x as f64 * 8.0, t) } else { cloth(look, x as f64 * 8.0, y as f64 * 2.0, t) })
                });
            }
            canvas
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::{bounds, layers as layer_count};

    #[test]
    fn bandanas_are_thin_bands_with_eight_text_tiles() {
        let items = items(&fallback);
        assert_eq!(items.len(), STYLES.len());
        for item in &items {
            let (lo, hi) = bounds(&item.bones).unwrap();
            assert!(lo[1] >= -8.3 && hi[1] <= 0.5 && hi[0] <= EDGE + 1e-4, "{}", item.key);
            assert_eq!(layer_count(&item.bones), TILES);
            assert_eq!(item.layer_count(), TILES);
        }
        let band = &items[0].bones[0];
        assert!(band.cubes.iter().all(|c| c.size[1] == TALL));
    }

    #[test]
    fn text_scrolls_and_tails_flow() {
        let mask = fallback("x");
        assert!(mask.alpha.iter().any(|a| *a == 255));
        let soft = mask.blurred(2);
        assert_ne!(layers(Look::Shinobi, &mask, &soft, 0), layers(Look::Shinobi, &mask, &soft, 3));
        assert_ne!(tails("tail_r.tail_low_l", 0.3), tails("tail_l", 0.3));
        assert_eq!(tails("band", 0.3), Motion::REST);
    }
}
