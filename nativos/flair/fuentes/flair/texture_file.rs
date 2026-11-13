use super::umbx_flair_paint::{Canvas, SIZE};

pub const MAX_FRAMES: usize = 64;
const CAPE_ROWS: usize = 32;

fn checked(width: usize, height: usize, rgba: &[u8]) -> Result<(), String> {
    if rgba.len() != width * height * 4 || width == 0 || height == 0 {
        return Err(format!("{width}x{height} image with {} bytes", rgba.len()));
    }
    Ok(())
}

fn copy(rgba: &[u8], width: usize, top: usize, scale: usize, rows: usize, columns: usize) -> Canvas {
    let mut out = Canvas::default();
    for y in 0..rows {
        let row = (top + y * scale) * width;
        for x in 0..columns {
            let from = (row + x * scale) * 4;
            let to = (y * SIZE + x) * 4;
            out.0[to..to + 4].copy_from_slice(&rgba[from..from + 4]);
        }
    }
    out
}

fn count(height: usize, frame_height: usize) -> Result<usize, String> {
    if frame_height == 0 || height % frame_height != 0 {
        return Err(format!("height {height} is not a multiple of {frame_height}"));
    }
    let count = height / frame_height;
    if count > MAX_FRAMES {
        return Err(format!("{count} frames, at most {MAX_FRAMES}"));
    }
    Ok(count)
}

pub fn cape_frames(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<Canvas>, String> {
    let (width, height) = (width as usize, height as usize);
    checked(width, height, rgba)?;
    if width % SIZE != 0 {
        return Err(format!("width {width} is not a multiple of 64"));
    }
    let scale = width / SIZE;
    let frame_height = CAPE_ROWS * scale;
    let frames = count(height, frame_height)?;
    Ok((0..frames).map(|f| copy(rgba, width, f * frame_height, scale, CAPE_ROWS, SIZE)).collect())
}

pub fn sheet_frames(width: u32, height: u32, rgba: &[u8], frame_height: u32) -> Result<Vec<Canvas>, String> {
    let (width, height, frame_height) = (width as usize, height as usize, frame_height as usize);
    checked(width, height, rgba)?;
    if width > SIZE || frame_height > SIZE {
        return Err(format!("texture {width}x{frame_height} is larger than 64x64"));
    }
    let frames = count(height, frame_height)?;
    Ok((0..frames).map(|f| copy(rgba, width, f * frame_height, 1, frame_height, width)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(width: usize, height: usize) -> Vec<u8> {
        (0..width * height).flat_map(|i| [(i % 251) as u8, (i / width) as u8, (i % width) as u8, 0xFF]).collect()
    }

    #[test]
    fn a_plain_cape_fills_the_top_half() {
        let rgba = image(64, 32);
        let frames = cape_frames(64, 32, &rgba).unwrap();
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].0[..64 * 32 * 4], rgba[..]);
        assert!(frames[0].0[64 * 32 * 4..].iter().all(|b| *b == 0));
    }

    #[test]
    fn hd_cape_strips_scale_down_into_frames() {
        let rgba = image(128, 128);
        let frames = cape_frames(128, 128, &rgba).unwrap();
        assert_eq!(frames.len(), 2);
        let source = |x: usize, y: usize| u32::from_be_bytes([rgba[(y * 128 + x) * 4 + 3], rgba[(y * 128 + x) * 4], rgba[(y * 128 + x) * 4 + 1], rgba[(y * 128 + x) * 4 + 2]]);
        assert_eq!(frames[0].get(5, 7), source(10, 14));
        assert_eq!(frames[1].get(5, 7), source(10, 64 + 14));
    }

    #[test]
    fn sheets_split_into_frames_at_the_top_left() {
        let rgba = image(32, 64);
        let frames = sheet_frames(32, 64, &rgba, 32).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[1].0[..4], rgba[32 * 32 * 4..32 * 32 * 4 + 4]);
        assert_eq!(frames[0].get(40, 0), 0);
    }

    #[test]
    fn wrong_sizes_are_refused() {
        assert!(cape_frames(46, 22, &image(46, 22)).is_err());
        assert!(cape_frames(64, 40, &image(64, 40)).is_err());
        assert!(cape_frames(64, 32, &image(64, 31)).is_err());
        assert!(cape_frames(64, 32 * 65, &image(64, 32 * 65)).is_err());
        assert!(sheet_frames(128, 64, &image(128, 64), 64).is_err());
        assert!(sheet_frames(64, 48, &image(64, 48), 32).is_err());
    }
}
