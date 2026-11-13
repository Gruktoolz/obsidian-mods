// ChatHeads' own half: which chat line names a player, where that player's
// head and the line's text go, and the one setting. No game types here;
// `glue.rs` draws with them. This file is also the bundle's Cargo library, so
// its tests run without a Horizon build.

pub const ID: &str = "chatheads";
pub const NAME: &str = "ChatHeads";
pub const VERSION: &str = "0.1.1";
pub const DESCRIPTION: &str = "A player's head left of every chat line they wrote.";

/// The head's side and the face's corner in a player skin, in GUI pixels: a
/// skin is 64x64 and its face is the 8x8 at (8,8). A line's text moves right
/// by 10 for the head (8) and 2 of gap — `shift.rs` makes that room.
pub const HEAD: f32 = 8.0;
pub const SKIN: f32 = 64.0;
pub const FACE: f32 = 8.0;
/// The chat's own translate, from `Chat::layout`'s frame: text at 0 shows at
/// `scale * 4`.
const CHAT_X: f32 = 4.0;

/// The setting's key, one file in the mod's folder.
const KEY: &str = "heads";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub heads: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { heads: true }
    }
}

impl Settings {
    /// The settings `read` answers its key for: `false` turns the heads off,
    /// anything else (or nothing) keeps them on.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        let mut settings = Self::default();
        if let Some(value) = read(KEY) {
            settings.heads = value.trim() != "false";
        }
        settings
    }
    /// The key with the value to keep.
    pub fn saved(&self) -> [(&'static str, String); 1] {
        [(KEY, self.heads.to_string())]
    }
    /// Its page's button, one click per value.
    pub fn labels(&self) -> Vec<String> {
        vec![format!(
            "Chat heads: {}",
            if self.heads { "ON" } else { "OFF" }
        )]
    }
    /// Button `button` of its page went: the next value.
    pub fn click(&mut self, button: usize) {
        if button == 0 {
            self.heads = !self.heads;
        }
    }
}

/// The player a line is from: the `<Name>` the chat puts first, when Name is
/// a profile name (1-16 of `A-Z a-z 0-9 _`). A system line, a `/me` line or
/// a wrapped continuation has no such prefix and gets no head.
pub fn sender(line: &str) -> Option<&str> {
    let name = line.strip_prefix('<')?.split_once('>')?.0;
    let valid = !name.is_empty()
        && name.len() <= 16
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    valid.then_some(name)
}

/// A stable id for a name the game resolved no skin for, so a player keeps
/// one of the default skins (`tab_list::default_skin`).
pub fn seed(name: &str) -> u128 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.to_ascii_lowercase().bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    u128::from(hash)
}

/// Where a line's head goes, in HUD pixels: in the place the line's text had
/// before it moved right (`text_x`, `text_y` are the layout's own, inside the
/// chat's scaled frame), the chat's translate and scale wrapped as
/// `Chat::layout` does. The third value is the head's side.
pub fn head(text_x: i32, text_y: i32, scale: f32) -> (f32, f32, f32) {
    (
        scale * (text_x as f32 + CHAT_X),
        scale * text_y as f32,
        HEAD * scale,
    )
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
    fn system_and_broken_lines_have_none() {
        assert_eq!(sender("[Server] restarted"), None);
        assert_eq!(sender("Steve: hello"), None);
        assert_eq!(sender("Saved screenshot as 2026-10-30.png"), None);
        assert_eq!(sender(""), None);
        assert_eq!(sender("<3 hearts"), None);
        assert_eq!(sender("<> x"), None);
        assert_eq!(sender("<a b> x"), None);
        assert_eq!(sender("<0123456789abcdefg> x"), None);
        // A wrapped continuation starts mid-message, never with `<`.
        assert_eq!(sender("and the rest of it"), None);
        assert_eq!(sender(" <Steve> shifted"), None);
    }

    #[test]
    fn the_head_sits_where_the_line_used_to_start() {
        // The line's text at 0 shows at 4 (the chat's translate), scale and all.
        assert_eq!(head(0, 50, 1.0), (4.0, 50.0, 8.0));
        assert_eq!(head(0, 50, 2.0), (8.0, 100.0, 16.0));
        // Its right edge keeps the 2-pixel gap to the text, which moved right
        // by 10: 8 of head, 2 of gap.
        for scale in [1.0, 1.5, 2.0] {
            let (x, _, side) = head(0, 0, scale);
            assert_eq!(x + side + 2.0 * scale, scale * (CHAT_X + 10.0));
        }
    }

    #[test]
    fn the_setting_starts_on_and_reads_what_it_saved() {
        let settings = Settings::default();
        assert!(settings.heads);
        assert_eq!(settings.labels(), vec!["Chat heads: ON"]);
        assert_eq!(settings.saved(), [("heads", "true".to_owned())]);
        assert_eq!(
            Settings::load(|key| (key == "heads").then(|| "false".to_owned())),
            Settings { heads: false }
        );
        assert_eq!(Settings::load(|_| None), Settings { heads: true });
        assert_eq!(
            Settings::load(|_| Some("  true ".to_owned())),
            Settings { heads: true }
        );
    }

    #[test]
    fn a_click_turns_the_heads_off_and_back() {
        let mut settings = Settings::default();
        settings.click(0);
        assert!(!settings.heads);
        assert_eq!(settings.labels(), vec!["Chat heads: OFF"]);
        assert_eq!(settings.saved(), [("heads", "false".to_owned())]);
        settings.click(0);
        assert!(settings.heads);
        // No other button: the page has one.
        settings.click(1);
        assert!(settings.heads);
    }

    #[test]
    fn a_seed_is_steadier_than_the_name_it_is_made_from() {
        assert_eq!(seed("Steve"), seed("steve"));
        assert_ne!(seed("Steve"), seed("Alex"));
    }
}
