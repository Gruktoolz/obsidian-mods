pub const SIZE: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas(pub Vec<u8>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
    Right,
    Front,
    Left,
    Back,
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas(vec![0; SIZE * SIZE * 4])
    }
}

impl Canvas {
    pub fn put(&mut self, x: usize, y: usize, argb: u32) {
        if x < SIZE && y < SIZE {
            let i = (y * SIZE + x) * 4;
            self.0[i..i + 4].copy_from_slice(&[(argb >> 16) as u8, (argb >> 8) as u8, argb as u8, (argb >> 24) as u8]);
        }
    }

    #[cfg(test)]
    pub fn get(&self, x: usize, y: usize) -> u32 {
        let i = (y * SIZE + x) * 4;
        u32::from_be_bytes([self.0[i + 3], self.0[i], self.0[i + 1], self.0[i + 2]])
    }

    pub fn rect(&mut self, [x, y, w, h]: [usize; 4], paint: impl Fn(usize, usize) -> u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put(x + dx, y + dy, paint(dx, dy));
            }
        }
    }

    pub fn cube(&mut self, [u, v]: [usize; 2], [w, h, d]: [usize; 3], paint: impl Fn(Side, usize, usize, usize, usize) -> u32) {
        let faces = [
            (Side::Top, [u + d, v, w, d]),
            (Side::Bottom, [u + d + w, v, w, d]),
            (Side::Right, [u, v + d, d, h]),
            (Side::Front, [u + d, v + d, w, h]),
            (Side::Left, [u + d + w, v + d, d, h]),
            (Side::Back, [u + 2 * d + w, v + d, w, h]),
        ];
        for (side, rect) in faces {
            self.rect(rect, |x, y| paint(side, x, y, rect[2], rect[3]));
        }
    }
}

pub const fn opaque(rgb: u32) -> u32 {
    rgb | 0xFF00_0000
}

pub fn mix(a: u32, b: u32, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let channel = |shift: u32| {
        let (x, y) = (f64::from((a >> shift) & 0xFF), f64::from((b >> shift) & 0xFF));
        ((x + (y - x) * t).round() as u32) << shift
    };
    channel(16) | channel(8) | channel(0)
}

pub fn palette(low: u32, mid: u32, high: u32, t: f64) -> u32 {
    if t < 0.5 { mix(low, mid, t * 2.0) } else { mix(mid, high, (t - 0.5) * 2.0) }
}

pub fn hue(turns: f64) -> u32 {
    let h = turns.rem_euclid(1.0) * 6.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    let byte = |v: f64| (v * 255.0).round() as u32;
    byte(r) << 16 | byte(g) << 8 | byte(b)
}

pub fn noise(a: i64, b: i64) -> f64 {
    let mut h = (a as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (b as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    (h >> 11) as f64 / (1u64 << 53) as f64
}

pub fn smooth(x: f64, y: f64, seed: i64) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (fx, fy) = (x - x0, y - y0);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let (i, j) = (x0 as i64, y0 as i64);
    let corner = |a: i64, b: i64| noise(a.wrapping_mul(7919).wrapping_add(seed), b);
    let top = corner(i, j) + (corner(i + 1, j) - corner(i, j)) * sx;
    let bottom = corner(i, j + 1) + (corner(i + 1, j + 1) - corner(i, j + 1)) * sx;
    top + (bottom - top) * sy
}

pub fn fbm(x: f64, y: f64, seed: i64, octaves: u32) -> f64 {
    let (mut sum, mut amp, mut freq, mut norm) = (0.0, 0.5, 1.0, 0.0);
    for o in 0..octaves {
        sum += amp * smooth(x * freq, y * freq, seed + i64::from(o) * 101);
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mask {
    pub width: usize,
    pub height: usize,
    pub alpha: Vec<u8>,
}

impl Mask {
    pub fn new(width: usize, height: usize) -> Self {
        Mask { width, height, alpha: vec![0; width * height] }
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        f64::from(self.alpha[(y % self.height) * self.width + x % self.width]) / 255.0
    }

    pub fn blurred(&self, radius: usize) -> Mask {
        let mut out = Mask::new(self.width, self.height);
        let r = radius as isize;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0u32;
                let mut count = 0u32;
                for dy in -r..=r {
                    for dx in -r..=r {
                        let (px, py) = (x as isize + dx, y as isize + dy);
                        if py >= 0 && (py as usize) < self.height {
                            sum += u32::from(self.alpha[py as usize * self.width + px.rem_euclid(self.width as isize) as usize]);
                            count += 1;
                        }
                    }
                }
                out.alpha[y * self.width + x] = (sum / count.max(1)) as u8;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get_round_trip_argb() {
        let mut canvas = Canvas::default();
        canvas.put(3, 5, 0x8012_3456);
        assert_eq!(canvas.get(3, 5), 0x8012_3456);
        canvas.put(SIZE, 0, 0xFFFF_FFFF);
        assert_eq!(canvas.get(0, 0), 0);
    }

    #[test]
    fn cube_paints_the_box_uv_layout() {
        let mut canvas = Canvas::default();
        canvas.cube([0, 0], [8, 8, 8], |side, _, _, _, _| opaque(side as u32));
        assert_eq!(canvas.get(9, 1), opaque(Side::Top as u32));
        assert_eq!(canvas.get(17, 1), opaque(Side::Bottom as u32));
        assert_eq!(canvas.get(1, 9), opaque(Side::Right as u32));
        assert_eq!(canvas.get(9, 9), opaque(Side::Front as u32));
        assert_eq!(canvas.get(17, 9), opaque(Side::Left as u32));
        assert_eq!(canvas.get(25, 9), opaque(Side::Back as u32));
        assert_eq!(canvas.get(1, 1), 0);
    }

    #[test]
    fn smooth_noise_is_continuous_and_masks_blur() {
        assert!((smooth(3.0, 4.0, 1) - smooth(3.0001, 4.0, 1)).abs() < 0.01);
        assert!((0..200).all(|i| (0.0..=1.0).contains(&fbm(i as f64 * 0.37, i as f64 * 0.11, 3, 4))));
        let mut mask = Mask::new(8, 4);
        mask.alpha[9] = 255;
        assert_eq!(mask.at(1, 1), 1.0);
        assert_eq!(mask.at(9, 5), 1.0);
        let soft = mask.blurred(1);
        assert!(soft.alpha[10] > 0 && soft.alpha[10] < 255);
    }

    #[test]
    fn colors_blend_and_noise_stays_in_range() {
        assert_eq!(mix(0x00_0000, 0xFF_FFFF, 0.5), 0x80_8080);
        assert_eq!(hue(0.0), 0xFF_0000);
        assert_eq!(hue(1.25), hue(0.25));
        assert!((0..1000).all(|i| (0.0..1.0).contains(&noise(i, i * 7 - 3))));
    }
}
