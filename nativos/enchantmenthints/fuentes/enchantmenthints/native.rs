// Enchantment Hints' adapter to the game (Horizon 142388e1c136e622652ec7505af15d8ad195e041,
// see horizon-mixin.toml). Injected as `crate::game::enchantmenthints_native`;
// binds the game-independent logic in `crate::game::enchantmenthints`
// (runtime.rs) to the running game: the tooltip's lines, the enchantment
// registry, the selected language, the font and the Shift keys.
use super::Game;
use super::enchantmenthints::{self as eh, Config, Effect, Items, Lang, Loaded};
use horizon_client::{hud::rgba, items::ItemStack, text::Text};
use std::sync::Arc;

/// `config.json` as last read; None until something needs it. Only the
/// window's game draws tooltips, but a lock keeps it safe anywhere.
static CONFIG: std::sync::Mutex<Option<Loaded>> = std::sync::Mutex::new(None);

thread_local! {
    // Set while `stack_tooltip` runs the game's own tooltip for us.
    static INSIDE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Never held while the game runs a hook of ours.
fn cached<T>(run: impl FnOnce(&mut Option<Loaded>) -> T) -> T {
    let mut cached = CONFIG
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    run(&mut cached)
}

/// `config.json` in the mod's settings folder; missing is the defaults.
fn read_config(game: &Game<'_>) -> Loaded {
    match game.native_setting(eh::ID, eh::CONFIG_FILE) {
        Some(text) => Config::from_json(&text).map_err(|e| format!("{}: {e}", eh::CONFIG_FILE)),
        None => Ok(Config::default()),
    }
}
/// Puts `loaded` in effect.
fn apply(loaded: Loaded) {
    if let Err(error) = &loaded {
        eprintln!("Enchantment Hints disabled: {error}");
    }
    cached(|c| *c = Some(loaded));
}
/// The configuration as read (once, the first time it is needed). A broken
/// file shows no hints (fails closed) and its error on the settings page.
fn loaded(game: &Game<'_>) -> Loaded {
    if let Some(loaded) = cached(|c| c.clone()) {
        return loaded;
    }
    let read = read_config(game);
    cached(|c| {
        c.get_or_insert_with(|| {
            if let Err(error) = &read {
                eprintln!("Enchantment Hints disabled: {error}");
            }
            read
        })
        .clone()
    })
}
/// The language of the mod's own labels and reminder.
fn lang() -> Lang {
    Lang::for_code(&horizon_client::lang::code())
}

impl Game<'_> {
    /// `stations::Game::stack_tooltip`'s HEAD: the game's own tooltip with a
    /// hint under each enchantment (or the Shift reminder); None leaves the
    /// tooltip to the game (no enchantments, the mod turned off in Options >
    /// Mods, a broken `config.json`, or this very call asking the game).
    pub(super) fn enchantmenthints_tooltip(&self, stack: &ItemStack) -> Option<Vec<(Text, u32)>> {
        if INSIDE.get() || !self.native_on(eh::ID) {
            return None;
        }
        let config = loaded(self).ok()?;
        // Each enchantment's line as `Enchantments::tooltip` writes it, with
        // its id: stored enchantments (books) come first, then the item's own.
        let e = &self.res.stations().enchantments;
        let order = e.tag("minecraft:tooltip_order");
        let block = |levels: &[(Arc<str>, u32)]| -> Vec<(String, String)> {
            eh::in_tooltip_order(order, levels)
                .into_iter()
                .map(|(id, level)| {
                    let (name, _) = e.full_name(id, level);
                    (id.to_owned(), Text::legacy(&name).plain())
                })
                .collect()
        };
        let stored = block(stack.stored_enchantments());
        let gear = match config.items {
            Items::All => block(stack.enchantments()),
            Items::Books => Vec::new(),
        };
        if stored.is_empty() && gear.is_empty() {
            return None;
        }

        // The game's own tooltip, through this same function.
        struct Inside;
        impl Drop for Inside {
            fn drop(&mut self) {
                INSIDE.set(false);
            }
        }
        INSIDE.set(true);
        let inside = Inside;
        let lines = self.stack_tooltip(stack);
        drop(inside);

        let plain: Vec<String> = lines.iter().map(|(text, _)| text.plain()).collect();
        let language = horizon_client::lang::current();
        let code = language
            .as_ref()
            .map_or_else(|| "en_us".to_owned(), |l| l.code.clone());
        let lookup = |key: &str| {
            language
                .as_ref()
                .and_then(|l| l.get(key))
                .map(str::to_owned)
        };
        let inserts = eh::plan(
            &plain,
            &[stored, gear],
            &config,
            self.has_shift_down(),
            lang(),
            |id| eh::describe(id, &code, lookup),
            |s| self.native_text_width(s),
        );
        if inserts.is_empty() {
            return Some(lines);
        }
        let [_, r, g, b] = config.rgb().to_be_bytes();
        let colour = rgba(r, g, b, 255);
        Some(eh::apply(&lines, &inserts, |line| {
            (Text::literal(line), colour)
        }))
    }

    /// `native_settings` (a hook point): the settings page's buttons.
    pub(super) fn enchantmenthints_settings(&self) -> Vec<String> {
        eh::labels(&loaded(self), lang())
    }

    /// `native_settings_click` (a hook point): change the setting, write
    /// `config.json` and put it in effect at once (or read the file again).
    pub(super) fn enchantmenthints_settings_click(&mut self, button: usize) {
        match eh::click(&loaded(self), button) {
            Some(Effect::Save(config)) => {
                self.native_set_setting(eh::ID, eh::CONFIG_FILE, &config.to_json());
                apply(Ok(config));
            }
            Some(Effect::Reload) => apply(read_config(self)),
            None => {}
        }
    }
}
