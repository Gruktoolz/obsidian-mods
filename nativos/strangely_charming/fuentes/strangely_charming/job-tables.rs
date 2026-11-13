fn hook(dir: &Path, out: &mut HashMap<String, Value>) {
    if dir
        .parent()
        .and_then(Path::file_name)
        .and_then(|s| s.to_str())
        != Some("minecraft")
    {
        return;
    }
    let Some(kind @ ("trade_set" | "villager_trade")) = dir.file_name().and_then(|s| s.to_str())
    else {
        return;
    };
    let mut files = Vec::new();
    horizon_mods::strangely_charming_content::merchant_files(&mut files);
    for (key, bytes) in files {
        if let Some((registry, id)) = key.split_once('/')
            && registry == kind
            && let Ok(value) = serde_json::from_slice(&bytes)
        {
            out.insert(id.to_owned(), value);
        }
    }
}
