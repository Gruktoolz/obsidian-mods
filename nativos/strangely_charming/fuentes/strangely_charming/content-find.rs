fn hook(dir: &Path, mut value: Vec<PathBuf>) -> Vec<PathBuf> {
 if dir.file_name().is_some_and(|n| n == "horizon-mods") {
  match crate::strangely_charming_content::install(dir) {
   Ok(path) => { if !value.contains(&path) { value.push(path); } },
   Err(e) => eprintln!("Strangely Charming: cannot install bundled content: {e}"),
  }
 }
 value
}
