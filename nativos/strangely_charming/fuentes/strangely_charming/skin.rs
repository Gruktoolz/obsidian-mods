use crate::strangely_charming_format::{self as format, Bits, Features};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, OnceLock},
};
#[derive(Clone)]
pub struct Skin {
    pub features: Features,
    pub textures: HashMap<String, Vec<u8>>,
}
#[derive(Default)]
struct Cache {
    local: HashMap<String, Vec<u8>>,
    pixels: HashMap<u64, (Vec<u8>, Arc<Skin>)>,
    order: VecDeque<u64>,
    names: HashMap<String, Arc<Skin>>,
    name_order: VecDeque<String>,
}
fn cache() -> &'static Mutex<Cache> {
    static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
    CACHE.get_or_init(Default::default)
}
fn digest(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut h);
    h.finish()
}
pub fn get(name: &str) -> Option<Arc<Skin>> {
    cache().lock().ok()?.names.get(name).cloned()
}
pub fn bind(name: &str, pixels: &[u8]) -> Option<Arc<Skin>> {
    if name.contains("/strangely_charming/") {
        return None;
    }
    let mut c = cache().lock().ok()?;
    let entry = c
        .pixels
        .get(&digest(pixels))
        .filter(|(stored, _)| stored == pixels)
        .map(|(_, s)| s.clone());
    if let Some(s) = &entry {
        if !c.names.contains_key(name) {
            c.name_order.push_back(name.into());
        }
        c.names.insert(name.into(), s.clone());
        while c.name_order.len() > 512 {
            if let Some(old) = c.name_order.pop_front() {
                c.names.remove(&old);
            }
        }
    } else {
        c.names.remove(name);
    }
    entry
}
pub fn texture_name(skin: &str, kind: &str) -> String {
    format!("{skin}/strangely_charming/{kind}")
}
fn embedded(bytes: &[u8], kind: &str) -> Option<Vec<u8>> {
    // Alfalfa itself is skin-size bounded. Inspect PNG dimensions before decoding
    // to avoid allocating for arbitrary dimensions declared by a malformed skin.
    if bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let w = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    let valid = match kind {
        "wing" => (w, h) == (20, 16) || (w, h) == (12, 12),
        "cape" => (w, h) == (20, 16),
        _ => false,
    };
    if !valid {
        return None;
    }
    let (dw, dh, rgba) = crate::hud::decode_bytes(bytes).ok()?;
    if (dw, dh) != (w, h) || rgba.len() != (w * h * 4) as usize {
        return None;
    }
    let mut canvas = vec![0; 64 * 64 * 4];
    let yoff = usize::from(kind == "wing" && w == 12) * 2;
    for y in 0..h as usize {
        let start = (y + yoff) * 64 * 4;
        canvas[start..start + w as usize * 4]
            .copy_from_slice(&rgba[y * w as usize * 4..(y + 1) * w as usize * 4]);
    }
    Some(canvas)
}
pub fn process(
    width: u32,
    height: u32,
    original: &[u8],
    result: Option<Vec<u8>>,
) -> Option<Vec<u8>> {
    let mut rgba = result?;
    let Some(mut features) = format::decode(width, height, original) else {
        return Some(rgba);
    };
    // Vanilla's large opaque rectangles include otherwise unused pixels that
    // Ears uses for ears, tails and other features. Keep body-face opacity but
    // restore source alpha outside the precise base-skin UV islands (Ears'
    // SkinRegion FORCE_OPAQUE regions). Do this before erasure/texture splitting.
    const BODY: [(usize, usize, usize, usize); 12] = [
        (8, 0, 24, 8),
        (0, 8, 32, 16),
        (4, 16, 12, 20),
        (0, 20, 16, 32),
        (20, 16, 36, 20),
        (16, 20, 40, 32),
        (44, 16, 52, 20),
        (40, 20, 56, 32),
        (20, 48, 28, 52),
        (16, 52, 32, 64),
        (36, 48, 44, 52),
        (32, 52, 48, 64),
    ];
    for y in 0..64 {
        for x in 0..64 {
            if !BODY
                .iter()
                .any(|&(x0, y0, x1, y1)| x >= x0 && x < x1 && y >= y0 && y < y1)
            {
                let alpha = (y * 64 + x) * 4 + 3;
                rgba[alpha] = original[alpha];
            }
        }
    }
    let alfalfa = format::alfalfa(original).unwrap_or_default();
    let mut textures = HashMap::new();
    for kind in ["wing", "cape"] {
        if let Some(png) = alfalfa.get(kind).and_then(|b| embedded(b, kind)) {
            textures.insert(kind.into(), png);
        }
    }
    if !textures.contains_key("wing") {
        features.wings = 0;
    }
    if !textures.contains_key("cape") {
        features.cape = false;
    }
    if let Some(erase) = alfalfa.get("erase") {
        let mut b = Bits::new(erase);
        while let (Some(x), Some(y), Some(w), Some(h)) =
            (b.read(6), b.read(6), b.read(5), b.read(5))
        {
            for py in y..(y + h + 1).min(64) {
                for px in x..(x + w + 1).min(64) {
                    rgba[(py as usize * 64 + px as usize) * 4
                        ..(py as usize * 64 + px as usize) * 4 + 4]
                        .fill(0);
                }
            }
        }
    }
    if features.swap_jacket_tail {
        let before = rgba.clone();
        for y in 0..12 {
            for x in 0..8 {
                let tail = ((16 + y) * 64 + 56 + x) * 4;
                let jacket = ((36 + y) * 64 + 32 + x) * 4;
                let flipped = ((47 - y) * 64 + 32 + x) * 4;
                rgba[tail..tail + 4].copy_from_slice(&before[flipped..flipped + 4]);
                rgba[jacket..jacket + 4].copy_from_slice(&before[tail..tail + 4]);
            }
        }
    }
    if features.legs != 0 {
        let mut displaced = vec![0; 64 * 64 * 4];
        for (u, v) in [(0, 16), (0, 32), (0, 48), (16, 48)] {
            let mut regions = vec![(u, v + 10, 16, 6), (u + 8, v, 4, 4)];
            if features.legs == 2 {
                regions.extend([(u, v + 4, 16, 6), (u + 4, v, 4, 4)]);
            }
            for (x, y, w, h) in regions {
                for py in y..y + h {
                    for px in x..x + w {
                        let i = (py * 64 + px) * 4;
                        displaced[i..i + 4].copy_from_slice(&rgba[i..i + 4]);
                        rgba[i..i + 4].fill(0);
                    }
                }
            }
        }
        textures.insert("displaced".into(), displaced);
    }
    if features.emissive {
        let mut palette = Vec::new();
        for y in 32..36 {
            for x in 52..56 {
                let p = &original[(y * 64 + x) * 4..(y * 64 + x) * 4 + 4];
                if p[3] != 0 {
                    palette.push([p[0], p[1], p[2]]);
                }
            }
        }
        let split = |pixels: &mut [u8]| {
            let mut glow = pixels.to_vec();
            for (p, g) in pixels.chunks_exact_mut(4).zip(glow.chunks_exact_mut(4)) {
                if palette.contains(&[p[0], p[1], p[2]]) {
                    p.fill(0);
                } else {
                    g.fill(0);
                }
            }
            glow
        };
        if palette.is_empty() {
            features.emissive = false;
        } else {
            textures.insert("emissive_skin".into(), split(&mut rgba));
            for kind in ["wing", "displaced"] {
                if let Some(pixels) = textures.get_mut(kind) {
                    let glow = split(pixels);
                    textures.insert(format!("emissive_{kind}"), glow);
                }
            }
        }
    }
    let id = digest(&rgba);
    if let Ok(mut c) = cache().lock() {
        if !c.pixels.contains_key(&id) {
            c.order.push_back(id);
        }
        c.pixels
            .insert(id, (rgba.clone(), Arc::new(Skin { features, textures })));
        while c.order.len() > 512 {
            if let Some(old) = c.order.pop_front() {
                c.pixels.remove(&old);
            }
        }
    }
    Some(rgba)
}

pub fn local(name: &str) -> Option<Vec<u8>> {
    cache().lock().ok()?.local.get(name).cloned()
}
pub fn load_local(path: &str) -> Result<String, String> {
    use std::{fs::File, io::Read};
    let file = File::open(path).map_err(|e| format!("Cannot open skin: {e}"))?;
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 || bytes.len() < 24 || &bytes[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("Expected a PNG smaller than 1 MiB.".into());
    }
    let w = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let h = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if (w, h) != (64, 64) {
        return Err("Ears test skins must be 64 by 64 pixels.".into());
    }
    let (w, h, raw) = crate::hud::decode_bytes(&bytes).map_err(|e| e.to_string())?;
    let pixels = crate::skins::process_legacy_skin(w, h, &raw).ok_or("Invalid skin")?;
    use sha2::{Digest, Sha256};
    let hash = format!("{:x}", Sha256::digest(&bytes));
    let name = crate::skins::skin_location(&hash);
    let mut c = cache().lock().map_err(|_| "Skin cache unavailable")?;
    if c.local.len() >= 16 && !c.local.contains_key(&name) {
        return Err(
            "16 local test skins are loaded; restart the client before loading more.".into(),
        );
    }
    c.local.insert(name, pixels);
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn skin() -> Vec<u8> {
        let mut p = vec![255; 16384];
        for i in 0..16 {
            let at = ((32 + i / 4) * 64 + i % 4) * 4;
            p[at..at + 4].copy_from_slice(&[63, 35, 216, 255]);
        }
        p
    }
    fn pixel(p: &mut [u8], i: usize, rgb: [u8; 3]) {
        let at = ((32 + i / 4) * 64 + i % 4) * 4;
        p[at..at + 3].copy_from_slice(&rgb);
    }
    #[test]
    fn vanilla_opacity_does_not_fill_ears_or_tail() {
        let mut p = skin();
        // Right-ear front, back above/below vanilla's y=32 boundary, tail,
        // and untouched left-ear front. Include partial alpha and hidden RGB.
        let samples = [
            (24, 0, 0),
            (56, 28, 0),
            (56, 32, 0),
            (56, 16, 0),
            (32, 0, 0),
            (63, 27, 96),
            (28, 48, 0),
        ];
        for &(x, y, a) in &samples {
            let i = (y * 64 + x) * 4;
            p[i..i + 4].copy_from_slice(&[13, 27, 39, a]);
        }
        // Base head/torso/arm/leg faces must retain vanilla's forced opacity.
        let body = [
            (8, 0),
            (0, 8),
            (4, 16),
            (0, 20),
            (20, 16),
            (16, 20),
            (44, 16),
            (40, 20),
            (20, 48),
            (16, 52),
            (36, 48),
            (32, 52),
        ];
        for &(x, y) in &body {
            p[(y * 64 + x) * 4 + 3] = 0;
        }
        let vanilla = crate::skins::process_legacy_skin(64, 64, &p).unwrap();
        assert_eq!(vanilla[(28 * 64 + 56) * 4 + 3], 255);
        assert_eq!(vanilla[(32 * 64 + 56) * 4 + 3], 0);
        let out = process(64, 64, &p, Some(vanilla)).unwrap();
        for &(x, y, _) in &samples {
            let i = (y * 64 + x) * 4;
            assert_eq!(&out[i..i + 4], &p[i..i + 4], "feature pixel {x},{y}");
        }
        for &(x, y) in &body {
            assert_eq!(out[(y * 64 + x) * 4 + 3], 255);
        }
    }
    #[test]
    fn ordinary_and_legacy_skins_keep_vanilla_preprocessing() {
        for height in [32, 64] {
            let p = vec![0; 64 * height as usize * 4];
            let vanilla = crate::skins::process_legacy_skin(64, height, &p);
            assert_eq!(process(64, height, &p, vanilla.clone()), vanilla);
        }
    }
    #[test]
    fn ordinary_skin_and_invalid_embedded_png_remain_safe() {
        let p = vec![255; 16384];
        assert_eq!(process(64, 64, &p, Some(p.clone())), Some(p));
        assert!(embedded(b"not a png", "wing").is_none());
        let mut png = vec![0; 24];
        png[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
        png[16..20].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(embedded(&png, "wing").is_none());
    }
    #[test]
    fn emissive_pixels_are_separated_and_bound_to_the_exact_skin() {
        let mut p = skin();
        pixel(&mut p, 10, [216, 120, 35]);
        for y in 32..36 {
            for x in 52..56 {
                let i = (y * 64 + x) * 4;
                p[i..i + 4].fill(0);
            }
        }
        let palette = (32 * 64 + 52) * 4;
        p[palette..palette + 4].copy_from_slice(&[10, 20, 30, 255]);
        p[..4].copy_from_slice(&[10, 20, 30, 255]);
        let out = process(64, 64, &p, Some(p.clone())).unwrap();
        assert_eq!(&out[..4], &[0; 4]);
        let info = bind("test/emissive", &out).unwrap();
        assert_eq!(&info.textures["emissive_skin"][..4], &[10, 20, 30, 255]);
        assert!(info.features.emissive);
        assert!(bind("test/emissive", &vec![255; 16384]).is_none());
        assert!(get("test/emissive").is_none());
    }
    #[test]
    fn digitigrade_displaces_only_selected_leg_regions() {
        let mut p = skin();
        pixel(&mut p, 11, [35, 216, 72]);
        let out = process(64, 64, &p, Some(p.clone())).unwrap();
        let info = bind("test/legs", &out).unwrap();
        assert_eq!(&out[(26 * 64) * 4..(26 * 64) * 4 + 4], &[0; 4]);
        assert_eq!(&out[(20 * 64) * 4..(20 * 64) * 4 + 4], &[255; 4]);
        assert_eq!(
            &info.textures["displaced"][(26 * 64) * 4..(26 * 64) * 4 + 4],
            &[255; 4]
        );
    }
}
