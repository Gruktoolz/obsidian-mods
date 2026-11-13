// kaptainwutax.seedcrackerX.config.StructureSave: the structures found so far, kept in a file named after the server (or the
// world), one `name;chunkX;chunkZ` line each, to be restored with `/seedcracker data restore` after rejoining.
use super::scx_feature::{Feature, FeatureData};
use std::sync::Arc;

/// `saveStructures`: the file's text for the structures among the data (decorators are not kept).
pub fn to_text(data: &[FeatureData]) -> String {
    let mut out = String::new();
    for d in data {
        if let Feature::Structure(s) = &*d.feature {
            out.push_str(&format!("{};{};{}\n", s.name(), d.chunk_x, d.chunk_z));
        }
    }
    out
}

/// `loadStructures`: the data of a saved file, for the structure types the version has; a line that is not three fields, or
/// names a structure the version lacks, is skipped.
pub fn from_text(text: &str, structure_types: &[Arc<Feature>]) -> Vec<FeatureData> {
    let mut result = Vec::new();
    for line in text.lines() {
        let info: Vec<&str> = line.split(';').collect();
        if info.len() != 3 {
            continue;
        }
        let (Ok(x), Ok(z)) = (info[1].parse::<i32>(), info[2].parse::<i32>()) else { continue };
        for feature in structure_types {
            if feature.name() == info[0] {
                result.push(FeatureData::structure(feature, x, z));
                break;
            }
        }
    }
    result
}

/// `getWorldName`: the file for a server address (or a world's folder), with `/` and `:` made `_`.
pub fn file_name(address: &str) -> String {
    format!("{}.txt", address.replace('/', "_").replace(':', "_"))
}
