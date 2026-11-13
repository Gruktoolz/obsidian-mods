use super::umbx_flair_capes::{chain, tiled};
use super::umbx_flair_items::{Item, Moves, Paint, Slot};
use super::umbx_flair_paint::{Canvas, fbm, mix, noise, opaque, palette};

pub const STYLES: [(&str, &str, u32); 4] = [
    ("celestial", "Celestial", 0x0B_0820),
    ("inferno", "Inferno", 0x1A_0604),
    ("thunderstorm", "Thunderstorm", 0x0C_1018),
    ("borealis", "Borealis", 0x06_0E18),
];
pub const TILES: usize = 8;
const TILE: usize = 64;
const WIDE: f64 = 128.0;
const TALL: f64 = 256.0;
const WHITE: u32 = 0xFF_FFFF;
const BLACK: u32 = 0;

pub fn items() -> Vec<Item> {
    STYLES
        .iter()
        .enumerate()
        .map(|(style, (key, name, _))| {
            let paint = Paint::Layers { count: TILES + 1, paint: Box::new(move |frame| layers(style, frame)) };
            Item::new(&format!("hd_{key}"), &format!("{name} HD"), Slot::Cape, chain(tiled), paint, Moves::Still)
        })
        .collect()
}

pub fn layers(style: usize, frame: u32) -> Vec<Canvas> {
    let t = f64::from(frame) * 0.1;
    let mut out: Vec<Canvas> = (0..TILES)
        .map(|tile| {
            let (column, row) = (tile % 2, tile / 2);
            let mut canvas = Canvas::default();
            for y in 0..TILE {
                for x in 0..TILE {
                    let u = (column * TILE + x) as f64 + 0.5;
                    let v = (row * TILE + y) as f64 + 0.5;
                    canvas.put(x, y, opaque(art(style, u / WIDE * 10.0, v / TALL * 16.0, t)));
                }
            }
            canvas
        })
        .collect();
    let lining = STYLES[style].2;
    let mut inside = Canvas::default();
    inside.rect([0, 0, 8, 8], |x, y| opaque(if x == 0 || y == 0 { mix(lining, WHITE, 0.15) } else { lining }));
    out.push(inside);
    out
}

pub fn art(style: usize, u: f64, v: f64, t: f64) -> u32 {
    let edge = (u.min(10.0 - u).min(v).min(16.0 - v) / 0.35).min(1.0);
    let color = match style {
        1 => inferno(u, v, t),
        2 => thunderstorm(u, v, t),
        3 => borealis(u, v, t),
        _ => celestial(u, v, t),
    };
    mix(BLACK, color, 0.55 + 0.45 * edge)
}

fn stars(u: f64, v: f64, t: f64, density: f64) -> f64 {
    let (cx, cy) = ((u * 9.0).floor(), (v * 9.0).floor());
    let seed = noise(cx as i64, cy as i64 + 911);
    if seed < 1.0 - density {
        return 0.0;
    }
    let (sx, sy) = (cx + 0.5 + (noise(cx as i64, cy as i64) - 0.5) * 0.6, cy + 0.5 + (noise(cy as i64, cx as i64) - 0.5) * 0.6);
    let d = ((u * 9.0 - sx).powi(2) + (v * 9.0 - sy).powi(2)).sqrt();
    let twinkle = 0.55 + 0.45 * (t * (1.5 + seed * 3.0) + seed * 40.0).sin();
    (1.0 - d * 3.2).max(0.0) * twinkle
}

fn celestial(u: f64, v: f64, t: f64) -> u32 {
    let sky = palette(0x05_0414, 0x15_0B3A, 0x2C_1260, v / 16.0);
    let cloud = fbm(u * 0.45 + t * 0.03, v * 0.35 - t * 0.02, 7, 5);
    let nebula = mix(sky, mix(0x6A_2CC8, 0x1F_B8C8, fbm(u * 0.3, v * 0.3 + 9.0, 3, 3)), ((cloud - 0.42) * 2.4).clamp(0.0, 0.8));
    let (dx, dy) = (u - 5.0, (v - 9.0) * 0.8);
    let r = (dx * dx + dy * dy).sqrt();
    let angle = dy.atan2(dx);
    let arms = (0.5 + 0.5 * (2.0 * angle - r * 1.6 + t * 0.25).cos()).powi(3) * (-r * 0.45).exp();
    let core = (-r * r * 1.2).exp();
    let galaxy = mix(nebula, 0xE8_D8FF, (arms * 1.4 + core).min(1.0));
    let (mx, my) = (u - 5.0, v - 3.0);
    let moon = mx * mx + my * my < 1.5 && (mx + 0.55).powi(2) + (my + 0.25).powi(2) > 1.25;
    let halo = (-(mx * mx + my * my).sqrt() * 1.6).exp() * 0.35;
    let lit = if moon { 0xFF_F2C8 } else { mix(galaxy, 0xFF_E8A8, halo) };
    mix(lit, WHITE, stars(u, v, t, 0.18))
}

fn inferno(u: f64, v: f64, t: f64) -> u32 {
    let rise = v / 16.0;
    let turbulence = fbm(u * 0.55, v * 0.32 + t * 0.9, 11, 5);
    let heat = (rise * 1.25 + (turbulence - 0.5) * 0.9 - 0.15).clamp(0.0, 1.0);
    let fire = if heat < 0.35 { mix(0x0A_0302, 0x7A_0E04, heat / 0.35) } else { palette(0xC0_2A06, 0xFF_8A1C, 0xFF_F0A0, (heat - 0.35) / 0.65) };
    let smoke = mix(fire, 0x1A_1416, ((0.3 - rise) * 2.5).clamp(0.0, 0.7) * fbm(u * 0.4, v * 0.4 - t * 0.2, 5, 3));
    let ember_cell = ((u * 3.0).floor(), ((v + t * 2.2) * 2.0).floor());
    let spark = noise(ember_cell.0 as i64, ember_cell.1 as i64 + 77);
    let ember = spark > 0.93 && ((u * 3.0).fract() - 0.5).abs() < 0.12 && (((v + t * 2.2) * 2.0).fract() - 0.5).abs() < 0.12;
    if ember { mix(smoke, 0xFF_D870, 0.9) } else { smoke }
}

fn bolt_x(v: f64, seed: i64) -> f64 {
    let mut x = 3.0 + noise(seed, 1) * 4.0;
    let mut y = 0.0;
    while y < v {
        y += 0.8;
        x += (noise(seed, (y * 10.0) as i64) - 0.5) * 1.6;
    }
    x.clamp(0.5, 9.5)
}

fn thunderstorm(u: f64, v: f64, t: f64) -> u32 {
    let cloud = fbm(u * 0.35 + t * 0.08, v * 0.3, 21, 5);
    let sky = palette(0x07_0A12, 0x26_2E44, 0x5C_6A88, (cloud * 1.2 - 0.1).clamp(0.0, 1.0) * (1.0 - v / 32.0));
    let cycle = (t / 3.2).floor() as i64;
    let phase = t / 3.2 - cycle as f64;
    let flash = if phase < 0.08 { 1.0 - phase / 0.08 } else { 0.0 };
    let path = bolt_x(v, cycle);
    let reach = 4.0 + noise(cycle, 9) * 12.0;
    let d = (u - path).abs();
    let glow = if v < reach { (-d * 5.0).exp() * flash } else { 0.0 };
    let core = d < 0.12 && v < reach && flash > 0.0;
    let lit = mix(sky, 0x9F_B8FF, flash * 0.35 * cloud);
    if core { 0xF4_F8FF } else { mix(lit, 0xB8_D0FF, glow) }
}

fn borealis(u: f64, v: f64, t: f64) -> u32 {
    let sky = mix(0x02_0610, 0x0A_1A30, v / 16.0);
    let wave = 4.5 + 1.6 * (u * 0.6 + t * 0.2).sin() + 0.8 * fbm(u * 0.5, t * 0.15, 4, 3);
    let band = (-(v - wave).powi(2) * 0.35).exp();
    let streaks = 0.55 + 0.45 * fbm(u * 2.2 + t * 0.3, 0.0, 8, 3);
    let curtain = palette(0x2C_FF9A, 0x1F_D6E0, 0x9B_5CFF, ((u / 10.0) + 0.15 * (t * 0.1).sin()).rem_euclid(1.0));
    let lit = mix(mix(sky, WHITE, stars(u, v, t, 0.2) * (1.0 - band)), curtain, (band * streaks * 1.3).min(1.0));
    let ridge = 12.6 + 1.4 * (u * 0.9).sin() + 0.8 * (u * 2.3 + 1.0).sin();
    if v > ridge {
        let snow = v - ridge < 0.35 && (u * 0.9).sin() > 0.2;
        if snow { 0xD8_E8F4 } else { mix(0x0A_1220, 0x04_0810, ((v - ridge) / 3.0).min(1.0)) }
    } else {
        lit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_hd_cape_paints_eight_tiles_and_a_lining_that_move() {
        for style in 0..STYLES.len() {
            let a = layers(style, 3);
            assert_eq!(a.len(), TILES + 1);
            assert!(a[..TILES].iter().all(|c| c.get(10, 10) >> 24 == 0xFF));
            let frames: Vec<Vec<Canvas>> = (0..40).step_by(7).map(|f| layers(style, f)).collect();
            assert!(frames.windows(2).any(|w| w[0] != w[1]), "{}", STYLES[style].0);
        }
        assert_eq!(items().len(), STYLES.len());
    }
}
