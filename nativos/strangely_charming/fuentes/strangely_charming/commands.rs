// Parse test-dummy commands independently of the client; all generated commands
// run as the invoking player, so the server's ordinary summon permissions apply.
#[derive(Debug, PartialEq)]
pub enum Command {
    SpawnName(String),
    SpawnTexture { hash: String, slim: bool },
    Local { file: String, slim: bool },
    Remove,
    Help,
}
pub fn parse(line: &str) -> Option<Result<Command, &'static str>> {
    let words: Vec<_> = line.split_whitespace().collect();
    if words.first().copied() != Some("strangely_charming") {
        return None;
    }
    if words.get(1..3) == Some(&["dummy", "file"]) {
        let rest = line
            .trim()
            .strip_prefix("strangely_charming")
            .unwrap()
            .trim_start()
            .strip_prefix("dummy")
            .unwrap()
            .trim_start()
            .strip_prefix("file")
            .unwrap()
            .trim();
        let (file, slim) = rest
            .strip_suffix(" slim")
            .map_or((rest, false), |p| (p, true));
        let file = file.trim_matches('"');
        return Some(if file.is_empty() {
            Err("Provide a PNG path after dummy file.")
        } else {
            Ok(Command::Local {
                file: file.into(),
                slim,
            })
        });
    }
    Some(match words.as_slice() {
        [_, "dummy", "remove"] => Ok(Command::Remove),
        [_, "dummy", name] if valid_name(name) => Ok(Command::SpawnName((*name).into())),
        [_, "dummy", "texture", hash] if valid_hash(hash) => Ok(Command::SpawnTexture {
            hash: hash.to_ascii_lowercase(),
            slim: false,
        }),
        [_, "dummy", "texture", hash, "slim"] if valid_hash(hash) => Ok(Command::SpawnTexture {
            hash: hash.to_ascii_lowercase(),
            slim: true,
        }),
        [_] | [_, "help"] | [_, "dummy"] => Ok(Command::Help),
        _ => Err(
            "Usage: /strangely_charming dummy <username> | dummy texture <64-digit hash> [slim] | dummy remove",
        ),
    })
}
fn valid_name(name: &str) -> bool {
    (1..=16).contains(&name.len()) && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
fn valid_hash(hash: &str) -> bool {
    hash.len() == 64 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}
pub fn summon(profile: &str) -> String {
    format!(
        "/summon minecraft:mannequin ~ ~ ~2 {{Tags:[\"strangely_charming.dummy\"],NoGravity:1b,Invulnerable:1b,profile:{profile}}}"
    )
}
pub const REMOVE: &str =
    "/kill @e[type=minecraft:mannequin,tag=strangely_charming.dummy,distance=..32]";
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_names_and_fixed_skins() {
        assert_eq!(
            parse("strangely_charming dummy Evander"),
            Some(Ok(Command::SpawnName("Evander".into())))
        );
        let hash = "ab".repeat(32);
        assert_eq!(
            parse(&format!("strangely_charming dummy texture {hash} slim")),
            Some(Ok(Command::SpawnTexture { hash, slim: true }))
        );
    }
    #[test]
    fn rejects_command_and_nbt_injection() {
        for line in [
            "strangely_charming dummy a\"}",
            "strangely_charming dummy @a",
            "strangely_charming dummy texture https://example.com/x",
            "strangely_charming dummy a extra",
        ] {
            assert!(parse(line).unwrap().is_err());
        }
        assert_eq!(parse("strangely_charming_other dummy a"), None);
    }
    #[test]
    fn local_paths_keep_spaces_and_never_become_server_commands() {
        assert_eq!(
            parse("strangely_charming  dummy file \"/tmp/My Skin.png\" slim"),
            Some(Ok(Command::Local {
                file: "/tmp/My Skin.png".into(),
                slim: true
            }))
        );
        assert!(parse("strangely_charming dummy file").unwrap().is_err());
    }
    #[test]
    fn removal_targets_only_nearby_tagged_mannequins() {
        assert_eq!(
            parse("strangely_charming dummy remove"),
            Some(Ok(Command::Remove))
        );
        assert!(
            REMOVE.contains("type=minecraft:mannequin,tag=strangely_charming.dummy,distance=..32")
        );
    }
}

pub fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for b in bytes.chunks(3) {
        let x = (u32::from(b[0]) << 16)
            | (u32::from(*b.get(1).unwrap_or(&0)) << 8)
            | u32::from(*b.get(2).unwrap_or(&0));
        out.push(TABLE[((x >> 18) & 63) as usize] as char);
        out.push(TABLE[((x >> 12) & 63) as usize] as char);
        out.push(if b.len() > 1 {
            TABLE[((x >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if b.len() > 2 {
            TABLE[(x & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}
#[cfg(test)]
mod encoding_tests {
    #[test]
    fn rfc4648_vectors() {
        for (a, b) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(super::base64(a.as_bytes()), b);
        }
    }
}
