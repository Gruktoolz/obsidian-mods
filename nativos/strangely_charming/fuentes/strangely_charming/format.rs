// Independent Rust implementation of the Ears skin wire formats.
// Specification reference: exa/Ears-Fifth-Edition, recorded in compatibility.json.
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Features {
    pub ears: u8,
    pub anchor: u8,
    pub protrusions: u8,
    pub tail: u8,
    pub segments: u8,
    pub bends: [f32; 4],
    pub snout: [u8; 4], // width, height, depth, vertical offset
    pub chest: f32,
    pub wings: u8,
    pub wing_animation: u8,
    pub cape: bool,
    pub emissive: bool,
    pub legs: u8,
    pub animate_tail: bool,
    pub swap_jacket_tail: bool,
}
impl Default for Features {
    fn default() -> Self {
        Self {
            ears: 0,
            anchor: 0,
            protrusions: 0,
            tail: 0,
            segments: 0,
            bends: [0.; 4],
            snout: [0; 4],
            chest: 0.,
            wings: 0,
            wing_animation: 0,
            cape: false,
            emissive: false,
            legs: 0,
            animate_tail: true,
            swap_jacket_tail: false,
        }
    }
}
pub struct Bits<'a> {
    data: &'a [u8],
    at: usize,
}
impl<'a> Bits<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, at: 0 }
    }
    pub fn read(&mut self, count: usize) -> Option<u32> {
        if count > 32 || self.at.checked_add(count)? > self.data.len() * 8 {
            return None;
        }
        let mut value = 0;
        for _ in 0..count {
            value = (value << 1) | u32::from((self.data[self.at / 8] >> (7 - self.at % 8)) & 1);
            self.at += 1;
        }
        Some(value)
    }
    fn flag(&mut self) -> Option<bool> {
        Some(self.read(1)? != 0)
    }
    fn angle(&mut self) -> Option<f32> {
        let sign = if self.flag()? { -1. } else { 1. };
        Some(sign * self.read(6)? as f32 / 63. * 90.)
    }
}
fn ordinal(value: u32, max: u8) -> u8 {
    if value <= max as u32 { value as u8 } else { 0 }
}
/// Decode only complete modern RGBA skins; a legacy skin cannot contain Ears data.
pub fn decode(width: u32, height: u32, rgba: &[u8]) -> Option<Features> {
    if width != 64 || height != 64 || rgba.len() != 64 * 64 * 4 {
        return None;
    }
    let pixel = |i: usize| {
        let at = ((32 + i / 4) * 64 + i % 4) * 4;
        [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
    };
    let magic = pixel(0);
    if magic[..3] == [0x3f, 0x23, 0xd8] {
        return Some(decode_v0(pixel));
    }
    if magic[..3] != [0xea, 0x25, 0x01] {
        return None;
    }
    let bytes: Vec<u8> = (1..16).flat_map(|i| pixel(i)[..3].to_vec()).collect();
    decode_binary(&bytes)
}
pub fn decode_binary(bytes: &[u8]) -> Option<Features> {
    let mut b = Bits::new(bytes);
    let version = b.read(8)?;
    let mut f = Features::default();
    let ears = b.read(6)?;
    if ears != 0 {
        f.ears = ordinal((ears - 1) / 3 + 1, 9);
        f.anchor = ((ears - 1) % 3) as u8;
    }
    let mut protrusions = b.read(2)?;
    let tail = b.read(3)?;
    if tail != 0 {
        f.segments = b.read(2)? as u8 + 1;
        for i in 0..f.segments as usize {
            f.bends[i] = b.angle()?;
        }
    }
    f.snout[0] = b.read(3)? as u8;
    if f.snout[0] != 0 {
        f.snout[1] = b.read(2)? as u8 + 1;
        f.snout[2] = b.read(3)? as u8 + 1;
        f.snout[3] = (b.read(3)? as u8).min(8 - f.snout[1]);
    }
    f.chest = b.read(5)? as f32 / 31.;
    f.wings = ordinal(b.read(3)?, 6);
    let animated = if f.wings != 0 { b.flag()? } else { false };
    f.wing_animation = if animated { 0 } else { 1 };
    f.cape = b.flag()?;
    f.emissive = b.flag()?;
    f.tail = ordinal(
        if version >= 1 && tail == 7 {
            tail + b.read(3)?
        } else {
            tail
        },
        8,
    );
    if version >= 2 {
        f.legs = ordinal(b.read(3)?, 2);
        if f.wings != 0 {
            let a = b.read(3)?;
            if a <= 3 {
                f.wing_animation = a as u8;
            }
        }
        f.animate_tail = b.flag()?;
        f.swap_jacket_tail = b.flag()?;
    }
    if version >= 3 {
        protrusions |= b.read(2)? << 2;
    }
    f.protrusions = ordinal(protrusions, 7);
    Some(f)
}
fn decode_v0(pixel: impl Fn(usize) -> [u8; 4]) -> Features {
    const COLORS: [u32; 10] = [
        0x3f23d8, 0x23d848, 0xd82350, 0xb923d8, 0x23d8c6, 0xd87823, 0xd823b7, 0xd823ff, 0xfefdf2,
        0x5e605a,
    ];
    let color = |i| {
        let p = pixel(i);
        (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2])
    };
    let lookup = |i, values: [u8; 10], fallback| {
        COLORS
            .iter()
            .position(|c| *c == color(i))
            .map_or(fallback, |j| values[j])
    };
    let mut f = Features {
        ears: lookup(1, [1, 2, 0, 3, 4, 5, 6, 7, 8, 9], 0),
        anchor: lookup(2, [0, 1, 2, 0, 0, 0, 0, 0, 0, 0], 0),
        protrusions: lookup(3, [0, 2, 0, 1, 3, 0, 7, 6, 4, 5], 0),
        tail: lookup(4, [1, 2, 0, 3, 0, 4, 5, 6, 7, 8], 0),
        wings: lookup(8, [0, 2, 0, 5, 3, 4, 1, 6, 0, 0], 0),
        wing_animation: lookup(9, [0, 2, 1, 0, 0, 0, 3, 0, 0, 0], 0),
        emissive: color(10) == COLORS[5],
        legs: lookup(11, [0, 1, 0, 0, 0, 0, 2, 0, 0, 0], 0),
        ..Features::default()
    };
    if f.ears == 0 || f.ears == 3 {
        f.anchor = 0;
    }
    if color(5) != COLORS[0] {
        let p = pixel(5);
        f.bends = [255 - p[3], p[0], p[1], p[2]].map(|v| {
            if v == 0 {
                0.
            } else {
                let j = i32::from(v) - 128;
                (j + if j < 0 { -1 } else { 1 }) as f32 / 128. * 90.
            }
        });
        f.segments = 1;
        for i in 1..4 {
            if f.bends[i] == 0. {
                break;
            }
            f.segments += 1;
        }
    }
    let etc = pixel(7);
    if color(6) != COLORS[0] {
        let s = pixel(6);
        f.snout = [
            s[0].min(7),
            s[1].min(4),
            s[2].min(8),
            etc[1].min(8 - s[1].min(4)),
        ];
    }
    if color(7) != COLORS[0] {
        f.chest = (etc[0] as f32 / 128.).min(1.);
        f.cape = etc[2] & 16 != 0;
    }
    if color(12) != COLORS[0] {
        f.animate_tail = color(12) & 1 != 0;
        f.swap_jacket_tail = color(12) & 2 != 0;
    }
    f
}
pub const ALPHA_REGIONS: [[usize; 4]; 10] = [
    [8, 0, 24, 8],
    [0, 8, 8, 16],
    [16, 8, 32, 16],
    [4, 16, 12, 20],
    [20, 16, 36, 20],
    [44, 16, 52, 20],
    [0, 20, 56, 32],
    [20, 48, 28, 52],
    [36, 48, 44, 52],
    [16, 52, 48, 64],
];
/// Recover Alfalfa before vanilla forces alpha to 255. Fixed-size skin bounds all allocations.
pub fn alfalfa(rgba: &[u8]) -> Option<BTreeMap<String, Vec<u8>>> {
    if rgba.len() != 64 * 64 * 4 {
        return None;
    }
    let mut little = vec![0u8; 1600];
    let mut bit = 0;
    for [x1, y1, x2, y2] in ALPHA_REGIONS {
        for x in x1..x2 {
            for y in y1..y2 {
                let alpha = rgba[(y * 64 + x) * 4 + 3];
                if alpha == 0 {
                    continue;
                }
                let v = 127 - (alpha & 127);
                for j in 0..7 {
                    little[bit / 8] |= ((v >> j) & 1) << (bit % 8);
                    bit += 1;
                }
            }
        }
    }
    while little.last() == Some(&0) {
        little.pop();
    }
    little.reverse();
    parse_alfalfa(&little)
}
pub fn parse_alfalfa(bytes: &[u8]) -> Option<BTreeMap<String, Vec<u8>>> {
    if !bytes.starts_with(&[0xea, 0x1f, 0xa1, 0xfa, 1]) {
        return None;
    }
    let mut b = Bits::new(&bytes[5..]);
    let mut map = BTreeMap::new();
    loop {
        let first = b.read(8)? as u8;
        if first == 0 {
            return Some(map);
        }
        let key = if first < 64 {
            match first {
                1 => "wing".into(),
                2 => "erase".into(),
                3 => "cape".into(),
                4 => "locator".into(),
                _ => format!("!unk{first}"),
            }
        } else {
            let mut s = String::from(char::from(first));
            loop {
                let c = b.read(8)? as u8;
                s.push(char::from(c & 127));
                if c & 128 != 0 {
                    break;
                }
            }
            s
        };
        let mut data = Vec::new();
        loop {
            let count = b.read(8)? as usize;
            for _ in 0..count {
                data.push(b.read(8)? as u8);
            }
            if count != 255 {
                break;
            }
        }
        map.insert(key, data);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn skin() -> Vec<u8> {
        vec![255; 64 * 64 * 4]
    }
    fn put(img: &mut [u8], i: usize, p: [u8; 4]) {
        let n = ((32 + i / 4) * 64 + i % 4) * 4;
        img[n..n + 4].copy_from_slice(&p);
    }
    #[test]
    fn ordinary_and_legacy_skins_are_untouched() {
        assert!(decode(64, 64, &skin()).is_none());
        assert!(decode(64, 32, &vec![255; 64 * 32 * 4]).is_none());
        assert!(decode(64, 64, &[0; 16]).is_none());
    }
    #[test]
    fn v0_sides_front_and_combined_protrusions() {
        let mut img = skin();
        for i in 0..16 {
            put(&mut img, i, [0x3f, 0x23, 0xd8, 255]);
        }
        put(&mut img, 1, [0x23, 0xd8, 0x48, 255]);
        put(&mut img, 2, [0x23, 0xd8, 0x48, 255]);
        put(&mut img, 3, [0x23, 0xd8, 0xc6, 255]);
        let f = decode(64, 64, &img).unwrap();
        assert_eq!((f.ears, f.anchor, f.protrusions), (2, 1, 3));
        assert!(f.animate_tail);
    }
    #[test]
    fn binary_v3_extended_tail_and_halo() {
        let fields = [
            (3, 8),
            (27, 6),
            (2, 2),
            (7, 3),
            (1, 2),
            (0, 1),
            (63, 6),
            (1, 1),
            (21, 6),
            (3, 3),
            (1, 2),
            (3, 3),
            (7, 3),
            (31, 5),
            (1, 3),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 3),
            (2, 3),
            (2, 3),
            (0, 1),
            (1, 1),
            (1, 2),
        ];
        let mut bytes = vec![0; 45];
        let mut at = 0;
        for (v, n) in fields {
            for i in (0..n).rev() {
                bytes[at / 8] |= (((v >> i) & 1) as u8) << (7 - at % 8);
                at += 1;
            }
        }
        let f = decode_binary(&bytes).unwrap();
        assert_eq!((f.ears, f.anchor), (9, 2));
        assert_eq!(f.tail, 8);
        assert_eq!(f.bends, [90., -30., 0., 0.]);
        assert_eq!(f.snout, [3, 2, 4, 6]);
        assert_eq!(f.chest, 1.);
        assert_eq!((f.legs, f.protrusions, f.wing_animation), (2, 6, 2));
        assert!(!f.animate_tail);
        assert!(f.swap_jacket_tail && f.emissive && f.cape);
        assert!(decode_binary(&bytes[..3]).is_none());
    }
    #[test]
    fn alpha_roundtrip_and_corrupt_chunks() {
        let bytes = [0xea, 0x1f, 0xa1, 0xfa, 1, 1, 3, 4, 5, 6, 0];
        let mut img = skin();
        let mut bit = 0;
        for [x1, y1, x2, y2] in ALPHA_REGIONS {
            for x in x1..x2 {
                for y in y1..y2 {
                    let mut v = 0;
                    for j in 0..7 {
                        if bit / 8 < bytes.len() {
                            v |= ((bytes[bytes.len() - 1 - bit / 8] >> (bit % 8)) & 1) << j;
                        }
                        bit += 1;
                    }
                    img[(y * 64 + x) * 4 + 3] = 128 | (127 - v);
                }
            }
        }
        assert_eq!(alfalfa(&img).unwrap().get("wing"), Some(&vec![4, 5, 6]));
        assert!(parse_alfalfa(&bytes[..8]).is_none());
        assert!(alfalfa(&skin()).is_none());
    }
}
