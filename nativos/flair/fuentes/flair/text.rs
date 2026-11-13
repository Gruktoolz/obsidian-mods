use super::umbx_flair_bandanas::{STRIP_H, STRIP_W, fallback};
use super::umbx_flair_paint::Mask;
use ab_glyph::{Font, FontVec, PxScale, ScaleFont, point};
use std::path::PathBuf;

const FILL: f32 = 0.72;
const START: f32 = 6.0;
const WINDOWS_FONTS: [&str; 4] = ["YuGothB.ttc", "YuGothM.ttc", "meiryob.ttc", "msgothic.ttc"];
const OTHER_FONTS: [&str; 5] = [
    "/System/Library/Fonts/ヒラギノ角ゴシック W6.ttc",
    "/System/Library/Fonts/Hiragino Sans GB.ttc",
    "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
    "/usr/share/fonts/noto-cjk/NotoSansCJK-Bold.ttc",
    "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Bold.ttc",
];

fn candidates() -> Vec<PathBuf> {
    let windows = std::env::var_os("WINDIR").map_or_else(|| PathBuf::from("C:\\Windows"), PathBuf::from).join("Fonts");
    WINDOWS_FONTS.iter().map(|name| windows.join(name)).chain(OTHER_FONTS.iter().map(PathBuf::from)).collect()
}

pub(super) fn font() -> Option<FontVec> {
    let found = candidates().into_iter().find_map(|path| std::fs::read(&path).ok().and_then(|bytes| FontVec::try_from_vec_and_index(bytes, 0).ok()));
    if found.is_none() {
        log::warn!("Flair: no Japanese font found; bandanas use their built-in glyphs");
    }
    found
}

pub(super) fn write(font: Option<&FontVec>, text: &str) -> Mask {
    let Some(font) = font else { return fallback(text) };
    let mut mask = Mask::new(STRIP_W, STRIP_H);
    let scale = PxScale::from(STRIP_H as f32 * FILL);
    let scaled = font.as_scaled(scale);
    let baseline = ((STRIP_H as f32 + scaled.ascent() + scaled.descent()) / 2.0).round();
    let mut x = START;
    'fill: while !text.is_empty() {
        let before = x;
        for c in text.chars() {
            let id = font.glyph_id(c);
            let advance = scaled.h_advance(id);
            if x + advance > STRIP_W as f32 {
                break 'fill;
            }
            if let Some(outline) = font.outline_glyph(id.with_scale_and_position(scale, point(x, baseline))) {
                let bounds = outline.px_bounds();
                outline.draw(|gx, gy, coverage| {
                    let (px, py) = (bounds.min.x as i32 + gx as i32, bounds.min.y as i32 + gy as i32);
                    if (0..STRIP_W as i32).contains(&px) && (0..STRIP_H as i32).contains(&py) {
                        let i = py as usize * STRIP_W + px as usize;
                        mask.alpha[i] = mask.alpha[i].max((coverage.clamp(0.0, 1.0) * 255.0) as u8);
                    }
                });
            }
            x += advance;
        }
        if x <= before {
            break;
        }
    }
    mask
}
