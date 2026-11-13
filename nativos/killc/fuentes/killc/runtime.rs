// KillC for Horizon, the module crate::game::killc and the Cargo library for cargo test.
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;

// Vanilla's world border limit.
const MAX_RADIUS: u64 = 29_999_983;
const DEFAULTS: &str = "{\n  \"enabled\": true,\n  \"useRandomSpawn\": true,\n  \"randomSpawnRadius\": 200\n}\n";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub enabled: bool,
    pub random_spawn: bool,
    pub radius: u64,
}

/// config/killc.json with the Fabric keys, missing or broken keys keep their defaults.
pub fn parse(text: &str) -> Config {
    let v: serde_json::Value = serde_json::from_str(text).unwrap_or_default();
    Config {
        enabled: v["enabled"].as_bool().unwrap_or(true),
        random_spawn: v["useRandomSpawn"].as_bool().unwrap_or(true),
        radius: v["randomSpawnRadius"].as_u64().unwrap_or(200).clamp(1, MAX_RADIUS),
    }
}

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(parse(DEFAULTS));
    // Console lines for the next server tick: a player's hook can't run them.
    static QUEUE: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    // Players who died, respawned at random once they are alive again.
    static DEAD: RefCell<HashSet<u128>> = RefCell::new(HashSet::new());
}

/// A world opens: read the config, writing the defaults when it is missing.
pub fn start(dir: Option<&Path>) -> Config {
    let text = dir.and_then(|d| {
        let path = d.join("killc.json");
        std::fs::read_to_string(&path).ok().or_else(|| {
            let _ = std::fs::create_dir_all(d).and_then(|()| std::fs::write(&path, DEFAULTS));
            None
        })
    });
    let config = parse(text.as_deref().unwrap_or(DEFAULTS));
    CONFIG.with(|c| *c.borrow_mut() = config.clone());
    QUEUE.with(|q| q.borrow_mut().clear());
    DEAD.with(|d| d.borrow_mut().clear());
    config
}

pub fn config() -> Config {
    CONFIG.with(|c| c.borrow().clone())
}

/// None when not ours, Ok(answer) to kill yourself, Err(name) to kill another player.
pub fn parse_command(line: &str) -> Option<Result<&'static str, &str>> {
    let mut words = line.split_whitespace();
    let word = words.next()?;
    let target = words.next();
    match (word, target) {
        ("suicide", None) => Some(Ok("☠ You have committed suicide!")),
        ("killc", None) => Some(Ok("☠ You have killed yourself!")),
        ("killc", Some(name)) if words.next().is_none() => Some(Err(name)),
        _ => None,
    }
}

pub fn queue(line: String) {
    QUEUE.with(|q| q.borrow_mut().push(line));
}

pub fn take_queue() -> Vec<String> {
    QUEUE.with(|q| std::mem::take(&mut *q.borrow_mut()))
}

pub fn died(uuid: u128) {
    DEAD.with(|d| d.borrow_mut().insert(uuid));
}

/// Whether `uuid` died and this is the first time they are seen alive again.
pub fn respawned(uuid: u128) -> bool {
    DEAD.with(|d| d.borrow_mut().remove(&uuid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_fabric_config() {
        assert_eq!(parse(DEFAULTS), Config { enabled: true, random_spawn: true, radius: 200 });
        let c = parse(r#"{"enabled": false, "randomSpawnRadius": 0}"#);
        assert_eq!(c, Config { enabled: false, random_spawn: true, radius: 1 });
        assert_eq!(parse("not json"), parse(DEFAULTS));
    }

    #[test]
    fn parses_commands() {
        assert!(matches!(parse_command("suicide"), Some(Ok(_))));
        assert!(matches!(parse_command("killc"), Some(Ok(_))));
        assert_eq!(parse_command("killc Steve"), Some(Err("Steve")));
        assert_eq!(parse_command("killc Steve Alex"), None);
        assert_eq!(parse_command("suicide now"), None);
        assert_eq!(parse_command("kill"), None);
    }

    #[test]
    fn respawns_once() {
        died(7);
        assert!(respawned(7));
        assert!(!respawned(7));
    }
}
