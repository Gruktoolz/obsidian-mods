use super::umbx_flair_capes as capes;
use super::umbx_flair_geo as geo;
use super::umbx_flair_items::{Attach, Item, Moves, Paint, Slot};
use super::umbx_flair_paint::Canvas;
use super::umbx_flair_texture_file as texture_file;
use std::path::{Path, PathBuf};

pub(super) fn items(dir: &Path) -> Vec<Item> {
    let capes = listed(&dir.join("capes"), |p| has_suffix(p, ".png")).into_iter().map(|p| (cape(&p), p));
    let cosmetics = listed(&dir.join("cosmetics"), Path::is_dir).into_iter().map(|p| (cosmetic(&p), p));
    capes
        .chain(cosmetics)
        .filter_map(|(item, path)| item.map_err(|error| log::warn!("Flair: {} skipped: {error}", path.display())).ok())
        .collect()
}

fn listed(dir: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).filter(|p| keep(p)).collect();
    paths.sort();
    paths
}

fn has_suffix(path: &Path, suffix: &str) -> bool {
    path.file_name().is_some_and(|n| n.to_string_lossy().to_lowercase().ends_with(suffix))
}

fn file_name(path: &Path) -> String {
    path.file_name().unwrap_or_default().to_string_lossy().into_owned()
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", file_name(path)))
}

fn decode(path: &Path) -> Result<(u32, u32, Vec<u8>), String> {
    horizon_client::hud::decode(path).map_err(|e| format!("{}: {e}", file_name(path)))
}

fn frametime(path: &Path) -> u32 {
    let mut meta = path.as_os_str().to_owned();
    meta.push(".mcmeta");
    let Ok(text) = std::fs::read_to_string(&meta) else { return 1 };
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(json) => json["animation"]["frametime"].as_u64().map_or(1, |t| t.clamp(1, 1200) as u32),
        Err(error) => {
            log::warn!("Flair: {} ignored: {error}", Path::new(&meta).display());
            1
        }
    }
}

fn painted(mut frames: Vec<Canvas>, texture: &Path) -> Paint {
    if frames.len() == 1 { Paint::Still(frames.remove(0)) } else { Paint::Frames { frames, ticks: frametime(texture) } }
}

fn cape(path: &Path) -> Result<Item, String> {
    let (width, height, rgba) = decode(path)?;
    let frames = texture_file::cape_frames(width, height, &rgba)?;
    let name = path.file_stem().unwrap_or_default().to_string_lossy();
    Ok(Item::new(&format!("custom:{}", file_name(path)), &name, Slot::Cape, capes::chain(capes::classic), painted(frames, path), Moves::Still))
}

fn cosmetic(folder: &Path) -> Result<Item, String> {
    let meta: serde_json::Value = serde_json::from_str(&read(&folder.join("cosmetic.json"))?).map_err(|e| format!("cosmetic.json: {e}"))?;
    let slot = meta["slot"].as_str().and_then(Slot::parse).ok_or("cosmetic.json: \"slot\" must be cape, hat, back, pet or aura")?;
    let folder_name = file_name(folder);
    let name = meta["name"].as_str().unwrap_or(&folder_name);
    let files = listed(folder, Path::is_file);
    let find = |suffix: &str| files.iter().find(|p| has_suffix(p, suffix));
    let texture = find(".png").ok_or("no .png texture")?;
    let (width, height, rgba) = decode(texture)?;
    let (bones, frames) = if slot == Slot::Cape {
        (capes::chain(capes::classic), texture_file::cape_frames(width, height, &rgba)?)
    } else {
        let (bones, size) = geo::geometry(&read(find(".geo.json").ok_or("no .geo.json model")?)?)?;
        (bones, texture_file::sheet_frames(width, height, &rgba, size[1] as u32)?)
    };
    let moves = match find(".animation.json") {
        Some(path) => Moves::Keys(geo::animation(&read(path)?)?),
        None => Moves::Still,
    };
    let mut item = Item::new(&format!("custom:{folder_name}"), name, slot, bones, painted(frames, texture), moves);
    if let Some(attach) = meta["attach"].as_str() {
        item.attach = Attach::parse(attach).ok_or("cosmetic.json: \"attach\" must be head, body or root")?;
    }
    Ok(if meta["glow"].as_bool() == Some(true) { item.glowing() } else { item })
}
