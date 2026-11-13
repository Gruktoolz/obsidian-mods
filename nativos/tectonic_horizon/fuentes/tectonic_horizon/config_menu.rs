// Mods-menu editor for the same JSON file used by the terrain runtime.
use super::tectonic_horizon::{Config, config_path};
use serde_json::Value;
use std::{cell::{Cell, RefCell}, io::Write};
const SECTIONS: [(&str, &str); 6] = [
    ("general", "General"), ("global_terrain", "Global terrain"),
    ("continents", "Continents"), ("islands", "Islands"),
    ("oceans", "Oceans"), ("biomes", "Biomes"),
];
const STEPS: [f64; 6] = [0.001, 0.01, 0.1, 1.0, 10.0, 100.0];
thread_local! {
    static SECTION: Cell<Option<usize>> = const { Cell::new(None) };
    static STEP: Cell<usize> = const { Cell::new(1) };
    static ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}
fn read() -> Result<Value, String> {
    let c = match std::fs::read_to_string(config_path()) {
        Ok(text) => Config::from_json(&text)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Config::default(),
        Err(e) => return Err(e.to_string()),
    };
    serde_json::to_value(c).map_err(|e| e.to_string())
}
fn label(key: &str) -> String {
    key.split('_').map(|word| {
        let mut chars = word.chars();
        chars.next().map(|c| c.to_uppercase().collect::<String>() + chars.as_str()).unwrap_or_default()
    }).collect::<Vec<_>>().join(" ")
}
fn number(n: f64) -> String {
    format!("{n:.3}").trim_end_matches('0').trim_end_matches('.').to_owned()
}
pub fn buttons() -> Vec<String> {
    let mut out = match read() {
        Err(e) => return vec![format!("Cannot read configuration: {e}"), "Fix config/tectonic-horizon.json, then reopen".into()],
        Ok(doc) => SECTION.with(|section| match section.get() {
            None => {
                let mut rows = vec!["Restart game after edits; new chunks only".into()];
                rows.extend(SECTIONS.iter().map(|(_, title)| format!("{title}...")));
                rows
            }
            Some(i) => {
                let mut rows = vec!["Back to Tectonic settings".into(),
                    "Restart game after edits; new chunks only".into(),
                    format!("Adjustment step: {}", number(STEP.with(|s| STEPS[s.get()]))),
                    format!("Reset {} to defaults", SECTIONS[i].1)];
                for (key, value) in doc[SECTIONS[i].0].as_object().unwrap() {
                    if let Some(on) = value.as_bool() {
                        rows.push(format!("{}: {}", label(key), if on { "ON" } else { "OFF" }));
                    } else {
                        rows.push(format!("{}: {} (-)", label(key), number(value.as_f64().unwrap())));
                        rows.push(format!("{}: {} (+)", label(key), number(value.as_f64().unwrap())));
                    }
                }
                rows
            }
        }),
    };
    ERROR.with(|e| { if let Some(e) = e.borrow().as_ref() { out.push(format!("Save failed: {e}")); } });
    out
}
fn write(doc: &Value) -> Result<(), String> {
    let text = serde_json::to_string_pretty(doc).map_err(|e| e.to_string())?;
    Config::from_json(&text)?;
    let path = config_path();
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let temp = path.with_extension(format!("json.menu-{}", std::process::id()));
    let result = (|| -> std::io::Result<()> {
        let mut f = std::fs::OpenOptions::new().create_new(true).write(true).open(&temp)?;
        f.write_all(format!("{text}\n").as_bytes())?;
        f.sync_all()?;
        std::fs::rename(&temp, &path)
    })();
    if result.is_err() { let _ = std::fs::remove_file(&temp); }
    result.map_err(|e| e.to_string())
}
fn change(button: usize) -> Result<(), String> {
    // Never overwrite a malformed or unreadable configuration.
    let mut doc = read()?;
    let Some(i) = SECTION.with(Cell::get) else {
        if (1..=SECTIONS.len()).contains(&button) { SECTION.with(|s| s.set(Some(button - 1))); }
        return Ok(());
    };
    match button {
        0 => { SECTION.with(|s| s.set(None)); return Ok(()); }
        1 => return Ok(()),
        2 => { STEP.with(|s| s.set((s.get() + 1) % STEPS.len())); return Ok(()); }
        3 => {
            doc[SECTIONS[i].0] = serde_json::to_value(Config::default()).unwrap()[SECTIONS[i].0].clone();
            return write(&doc);
        }
        _ => {}
    }
    let fields = doc[SECTIONS[i].0].as_object_mut().unwrap();
    let mut row = 4;
    for (key, value) in fields {
        if let Some(on) = value.as_bool() {
            if button == row { *value = Value::Bool(!on); return write(&doc); }
            row += 1;
        } else {
            if button == row || button == row + 1 {
                let integer = value.is_i64();
                let step = STEP.with(|s| STEPS[s.get()]);
                let step = if integer { step.max(1.0) } else { step };
                let (min, max) = match key.as_str() {
                    "snow_start_offset" => (-512.0, 512.0),
                    "monument_offset" => (-128.0, 128.0),
                    "vertical_scale" | "continents_scale" | "erosion_scale" | "ridge_scale" | "noise_scale" | "temperature_scale" | "vegetation_scale" => (0.001, 100.0),
                    _ => (-100.0, 100.0),
                };
                let n = (value.as_f64().unwrap() + if button == row { -step } else { step }).clamp(min, max);
                let n = (n * 1_000_000.0).round() / 1_000_000.0;
                *value = if integer { Value::from(n as i64) } else { Value::from(n) };
                return write(&doc);
            }
            row += 2;
        }
    }
    Ok(())
}
pub fn click(button: usize) {
    ERROR.with(|e| *e.borrow_mut() = change(button).err());
}
