// Chat Heads Rust: which chat lines name a player, where that player's head
// goes, and which skin the head shows. Written for Horizon from the behaviour
// of dzwdz's Chat Heads (MPL-2.0) and Domanffe's Horizon port (MIT); no Java
// code and no original assets. Cracked-safe by design: the sender is read
// from the message's own `<Name>` prefix, which vanilla chat writes the same
// way on offline-mode (cracked) servers as anywhere else - no Mojang auth,
// no signed chat and no UUID is needed. This file is also the bundle's Cargo
// library, so its tests run without a Horizon build.

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "chat_heads_rust";
pub const NAME: &str = "Chat Heads Rust";
pub const VERSION: &str = "1.0.2";
pub const DESCRIPTION: &str = "A player's head before their name on every chat line they wrote, on single player and on servers. Microsoft and offline-mode (cracked) players alike: names from the line, skins from the game's own tab list, face and hat layer drawn.";

/// The head's side and the face's corner in a player skin, in GUI pixels: a
/// skin is 64x64 and its face is the 8x8 at (8,8). A line's text moves right
/// by 10 for the head (8) and 2 of gap - the chat's own crate makes that room.
pub const HEAD: f32 = 8.0;
pub const SKIN: f32 = 64.0;
/// The chat's own translate: text laid out at 0 shows at scale * 4.
pub const CHAT_X: f32 = 4.0;

/// The mod's settings, one file per key in the mod's folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// Heads on the chat's lines.
    pub heads: bool,
    /// Heads on your own lines too (off: you know what you look like).
    pub own: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { heads: true, own: true }
    }
}

impl Settings {
    /// From the stored values; the usual values for the ones missing.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        let flag = |key: &str, default: bool| match read(key) {
            Some(v) => v.trim() != "false",
            None => default,
        };
        Self {
            heads: flag("heads", true),
            own: flag("own", true),
        }
    }

    pub fn saved(&self) -> [(&'static str, String); 2] {
        let on = |b: bool| (if b { "true" } else { "false" }).to_owned();
        [("heads", on(self.heads)), ("own", on(self.own))]
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Chat heads: {}", on(self.heads)),
            format!("Show your own head: {}", on(self.own)),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.heads = !self.heads,
            1 => self.own = !self.own,
            _ => {}
        }
    }
}

/// The player a chat line is from: the `<Name>` the chat puts first, when
/// Name is a profile name (1-16 of `A-Z a-z 0-9 _`). This is the whole
/// cracked-mode trick: vanilla (and vanilla-format plugins) write
/// `<Name> message` on every server, offline-mode included, so no UUID,
/// no signature and no Mojang account is needed to know who spoke. A system
/// line, a `/me` line, a `Name: message` line and a wrapped continuation
/// have no such prefix and get no head.
/// Strips the legacy `§` colour and format codes a server may put in a line,
/// so `§a<Steve>§r hi` and `<Steve> hi` read the same.
pub fn strip_codes(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\u{00a7}' {
            chars.next(); // the code character is dropped with the marker
        } else {
            out.push(c);
        }
    }
    out
}

fn is_profile_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 16
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// The player a chat line is from, strict form: the `<Name>` the chat puts
/// first, when Name is a profile name. This is the whole cracked-mode trick:
/// vanilla (and vanilla-format plugins) write `<Name> message` on every
/// server, offline-mode included, so no UUID, no signature and no Mojang
/// account is needed to know who spoke.
pub fn sender(line: &str) -> Option<&str> {
    let name = line.strip_prefix('<')?.split_once('>')?.0;
    is_profile_name(name).then_some(name)
}

/// The name-like start of a bare formatted line: a profile name, then a
/// separator the server's chat format puts after it (`Steve: hi`,
/// `Steve > hi`, `Steve » hi`, `Steve joined`). Gives where the name starts
/// and ends, in bytes of the stripped line.
pub fn bare_sender(line: &str) -> Option<(usize, usize, &str)> {
    let token: usize = line
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_alphanumeric() || *c == '_')
        .count();
    if token == 0 || token > 16 {
        return None;
    }
    let (name, rest) = line.split_at(token);
    if !is_profile_name(name) {
        return None;
    }
    let rest = rest.trim_start_matches(|c: char| c.is_whitespace());
    let sep = rest.strip_prefix(':').or_else(|| rest.strip_prefix('>'))
        .or_else(|| rest.strip_prefix('\u{00bb}')) // »
        .or_else(|| rest.strip_prefix('-'));
    match sep {
        // The separator may carry spaces on either side ("Steve > hi").
        Some(r) if r.starts_with(' ') || r.is_empty() => Some((0, token, name)),
        // ":" and ">" and a "»" split even flush against the name
        // ("Steve»hi"); a "-" needs its space, or every hyphenated word
        // after a name would split it.
        Some(_) if !rest.starts_with('-') => Some((0, token, name)),
        _ => None,
    }
}

/// The sender a line names, with the server's own player list as the
/// fallback: the strict `<Name>` form first; else a bare `Name:` /
/// `Name >` start; else the earliest player-list name the line mentions
/// (longest first, case-blind), so rank and nick formats the strict forms
/// miss still get the right head. Gives the name and where it starts, in
/// bytes of the stripped line.
pub fn match_sender<'a>(stripped: &'a str, known: &'a [String]) -> Option<(usize, &'a str)> {
    if let Some(name) = sender(stripped) {
        let start = stripped.find('<').unwrap_or(0) + 1;
        return Some((start, name));
    }
    if let Some((start, end, name)) = bare_sender(stripped) {
        let _ = end;
        return Some((start, name));
    }
    let lower = stripped.to_ascii_lowercase();
    let mut best: Option<(usize, &str)> = None;
    for player in known {
        if player.is_empty() {
            continue;
        }
        if let Some(at) = lower.find(&player.to_ascii_lowercase()) {
            let better = match best {
                Some((b_at, b_name)) => at < b_at || (at == b_at && player.len() > b_name.len()),
                None => true,
            };
            if better {
                best = Some((at, player.as_str()));
            }
        }
    }
    best
}

/// A stable pick of the default skin for a name the game resolved no skin
/// for, so a cracked player keeps one identity instead of flickering: an
/// FNV-1a hash of the lowercased name, split to Steve or Alex.
pub fn seed(name: &str) -> u128 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.to_ascii_lowercase().bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    u128::from(hash)
}

/// Where the head's layers sit in a skin's texture space: the face at (8,8)
/// and the hat overlay at (40,8), both 8x8, as `(u0, v0, u1, v1)` in skin
/// pixels. The hat goes over the face, so hair, hats and glasses show.
pub const FACE_X: f32 = 8.0;
pub const FACE_Y: f32 = 8.0;
pub const HAT_X: f32 = 40.0;
pub const HAT_Y: f32 = 8.0;

pub fn layers(skin_u0: f32, skin_v0: f32, du: f32, dv: f32) -> [(f32, f32, f32, f32); 2] {
    let layer = |ox: f32, oy: f32| {
        (
            skin_u0 + du * ox,
            skin_v0 + dv * oy,
            skin_u0 + du * (ox + HEAD),
            skin_v0 + dv * (oy + HEAD),
        )
    };
    [layer(FACE_X, FACE_Y), layer(HAT_X, HAT_Y)]
}

/// Where a line's head goes, in HUD pixels: at the line's own start - the
/// spot right before the `<Name>` the chat drew there - with the chat's
/// translate and scale wrapped the way its layout does.
pub fn head(text_x: i32, text_y: i32, scale: f32) -> (f32, f32, f32) {
    (scale * (text_x as f32 + CHAT_X), scale * text_y as f32, HEAD * scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_player_line_names_its_sender() {
        assert_eq!(sender("<Steve> hello"), Some("Steve"));
        assert_eq!(sender("<a_b1> x"), Some("a_b1"));
        assert_eq!(sender("<0123456789abcdef> x"), Some("0123456789abcdef"));
        assert_eq!(sender("<Alex>"), Some("Alex"));
    }

    #[test]
    fn cracked_offline_names_work_the_same() {
        // Offline-mode servers hand out these very names; the chat line is
        // written identically, so the heuristic needs no auth at all.
        assert_eq!(sender("<xX_Pro_Xx> yo"), Some("xX_Pro_Xx"));
        assert_eq!(sender("<Player123> hi"), Some("Player123"));
        assert_eq!(sender("<zz_zz_zz> gm"), Some("zz_zz_zz"));
    }

    #[test]
    fn system_and_broken_lines_have_none() {
        assert_eq!(sender("[Server] restarted"), None);
        assert_eq!(sender("Steve: hello"), None);
        assert_eq!(sender("Saved screenshot as 2026-10-30.png"), None);
        assert_eq!(sender(""), None);
        assert_eq!(sender("<3 hearts"), None);
        assert_eq!(sender("<> x"), None);
        assert_eq!(sender("<a b> x"), None);
        assert_eq!(sender("<0123456789abcdefg> x"), None);
        assert_eq!(sender("<töltés> x"), None); // non-ASCII name: not a profile name
        assert_eq!(sender("and the rest of it"), None); // wrapped continuation
        assert_eq!(sender(" <Steve> shifted"), None);
    }

    #[test]
    fn the_head_sits_right_before_the_name() {
        // The line's text at 0 shows at 4 (the chat's translate), scale and all.
        assert_eq!(head(0, 50, 1.0), (4.0, 50.0, 8.0));
        assert_eq!(head(0, 50, 2.0), (8.0, 100.0, 16.0));
        // Its right edge keeps the 2-pixel gap to the text, which moved right
        // by 10: 8 of head, 2 of gap - the head ends where the name starts.
        for scale in [1.0, 1.5, 2.0] {
            let (x, _, side) = head(0, 0, scale);
            assert_eq!(x + side + 2.0 * scale, scale * (CHAT_X + 10.0));
        }
    }

    #[test]
    fn the_head_draws_both_skin_layers() {
        // In a 64x64 skin's texture space: the face is the 8x8 at (8,8), the
        // hat overlay the 8x8 at (40,8), drawn over it.
        assert_eq!(layers(0.0, 0.0, 1.0, 1.0), [(8.0, 8.0, 16.0, 16.0), (40.0, 8.0, 48.0, 16.0)]);
        // Any skin rectangle (an atlas sub-rect) maps the same way.
        let [face, hat] = layers(32.0, 64.0, 2.0, 2.0);
        assert_eq!(face, (48.0, 80.0, 64.0, 96.0));
        assert_eq!(hat, (112.0, 80.0, 128.0, 96.0));
    }

    #[test]
    fn colour_codes_are_stripped_before_matching() {
        assert_eq!(strip_codes("\u{00a7}a<Steve>\u{00a7}r hi"), "<Steve> hi");
        assert_eq!(strip_codes("<\u{00a7}lSteve\u{00a7}r> x"), "<Steve> x");
        assert_eq!(strip_codes("no codes"), "no codes");
        assert_eq!(strip_codes("\u{00a7}"), ""); // a bare marker at the end
    }

    #[test]
    fn bare_server_formats_name_their_sender() {
        assert_eq!(bare_sender("Steve: hello"), Some((0, 5, "Steve")));
        assert_eq!(bare_sender("Steve > hello"), Some((0, 5, "Steve")));
        assert_eq!(bare_sender("Steve\u{00bb}hello"), Some((0, 5, "Steve")));
        assert_eq!(bare_sender("Steve:hi"), Some((0, 5, "Steve")));
        // A bare name with no separator ("Steve joined") is declined here —
        // too ambiguous — and gets its head from the player-list fallback.
        assert_eq!(bare_sender("Steve joined the game"), None);
        // Not name starts: a rank prefix, a sentence, a number token, too long.
        assert_eq!(bare_sender("[Admin] Steve: hi"), None);
        assert_eq!(bare_sender("Hello there"), None);
        assert_eq!(bare_sender("0123456789abcdefg: x"), None);
    }

    #[test]
    fn the_player_list_fallback_names_ranks_and_nicks() {
        let known = vec!["Steve".to_owned(), "Alex".to_owned()];
        // The strict and bare forms win without the list.
        assert_eq!(match_sender("<Alex> hi", &known), Some((1, "Alex")));
        assert_eq!(match_sender("Steve: hi", &known), Some((0, "Steve")));
        // A rank format the strict forms miss: the earliest listed player wins.
        assert_eq!(
            match_sender("[Admin] Steve \u{00bb} hello", &known),
            Some((8, "Steve")) // [Admin]=0..7, a space, then the name at 8
        );
        // Longest name at the same spot wins (Stev vs Steve).
        let mut known = vec!["Stev".to_owned(), "Steve".to_owned()];
        known.sort();
        assert_eq!(match_sender("Steve: x", &known).map(|p| p.1), Some("Steve"));
        // No name anywhere: no head.
        assert_eq!(match_sender("[Server] restarted", &known), None);
        assert_eq!(match_sender("hello", &[]), None);
    }

    #[test]
    fn a_seed_is_steady_and_case_blind() {
        assert_eq!(seed("Steve"), seed("steve"));
        assert_ne!(seed("Steve"), seed("Alex"));
    }

    #[test]
    fn the_settings_start_on_and_round_trip() {
        let s = Settings::default();
        assert_eq!(s.labels(), vec!["Chat heads: ON", "Show your own head: ON"]);

        let stored = [("heads", "false\n"), ("own", "false")];
        let s = Settings::load(|k| stored.iter().find(|p| p.0 == k).map(|p| p.1.to_owned()));
        assert_eq!(s, Settings { heads: false, own: false });
        assert_eq!(Settings::load(|_| None), Settings::default());
    }

    #[test]
    fn a_click_cycles_each_setting() {
        let mut s = Settings::default();
        s.click(0);
        assert!(!s.heads);
        s.click(1);
        assert!(!s.own);
        s.click(2); // no such button
        assert_eq!(s, Settings { heads: false, own: false });
    }
}
