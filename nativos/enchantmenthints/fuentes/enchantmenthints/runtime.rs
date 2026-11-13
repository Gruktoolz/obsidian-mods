// Enchantment Hints: a short hint under each enchantment in item tooltips.
//
// Original implementation for Horizon (a horizon-mixins native mod). This file is
// both the standalone, unit-tested Cargo library and the injected
// `crate::game::enchantmenthints` module. It has no Horizon dependency: the
// adapter (`native.rs`) hands it the tooltip's lines as plain text, a text
// measure and the selected language's lookup, and applies what it answers.

use serde::{Deserialize, Serialize};

/// The mod's id: its row in Options > Mods and its settings folder
/// (`horizon-mods/.horizon-mod-data/enchantmenthints/`).
pub const ID: &str = "enchantmenthints";
pub const VERSION: &str = "0.1.0";
pub const NAME: &str = "Enchantment Hints";
pub const DESCRIPTION: &str =
    "A short hint under each enchantment in item tooltips (always, or while Shift is held).";
/// The configuration, in the mod's settings folder: what the settings page writes.
pub const CONFIG_FILE: &str = "config.json";
/// What goes in front of each hint line, so it reads as part of the line above.
pub const INDENT: &str = " ";
/// A resource pack's key for an enchantment's hint (`<prefix>.<namespace>.<path>`):
/// it wins over the built-in text, in any language; an empty value hides the hint.
pub const KEY_PREFIX: &str = "enchantmenthints";
/// The widths a click on the page cycles through, in GUI pixels (0: no wrapping).
pub const WIDTH_STEPS: [u32; 6] = [120, 160, 200, 240, 320, 0];
/// Widths `config.json` may hold (or 0).
pub const WIDTH_RANGE: std::ops::RangeInclusive<u32> = 40..=1000;
/// Longest error shown on the page (the whole one goes to stderr).
const ERROR_WIDTH: usize = 40;

/// The languages the built-in text is written in. Any other language gets the
/// English text, unless a resource pack gives it its own (`KEY_PREFIX`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Ru,
}
impl Lang {
    /// The built-in language for a game language code (`en_us`, `ru_ru`), if any.
    pub fn shipped(code: &str) -> Option<Self> {
        let code = code.to_ascii_lowercase();
        if code == "ru_ru" || code.starts_with("ru_") {
            Some(Self::Ru)
        } else if code.starts_with("en_") {
            Some(Self::En)
        } else {
            None
        }
    }
    /// The language for the mod's own labels: the built-in one, else English.
    pub fn for_code(code: &str) -> Self {
        Self::shipped(code).unwrap_or(Self::En)
    }
    fn pick(self, en: &'static str, ru: &'static str) -> &'static str {
        match self {
            Self::En => en,
            Self::Ru => ru,
        }
    }
}

/// The built-in hints: every enchantment of Minecraft 26.3
/// (`data/minecraft/enchantment/*.json`), in English and Russian. Written for
/// this mod.
pub const HINTS: [(&str, &str, &str); 43] = [
    (
        "aqua_affinity",
        "Mine at normal speed while underwater.",
        "Под водой блоки добываются с обычной скоростью.",
    ),
    (
        "bane_of_arthropods",
        "Extra damage to spiders, bees, silverfish and other arthropods, and slows them down.",
        "Больше урона паукам, пчёлам, чешуйницам и другим членистоногим; замедляет их.",
    ),
    (
        "binding_curse",
        "Once worn, it cannot be taken off until it breaks or you die.",
        "Надетый предмет нельзя снять, пока он не сломается или вы не погибнете.",
    ),
    (
        "blast_protection",
        "Reduces explosion damage and the knockback of blasts.",
        "Снижает урон от взрывов и отбрасывание взрывной волной.",
    ),
    (
        "breach",
        "Mace hits ignore part of the target's armor.",
        "Удары булавой частично пробивают броню цели.",
    ),
    (
        "channeling",
        "In a thunderstorm, a thrown trident calls lightning down on the mob or lightning rod it hits.",
        "В грозу брошенный трезубец вызывает молнию в поражённое существо или громоотвод.",
    ),
    (
        "density",
        "Mace smash attacks deal more damage for every block you fell.",
        "Сокрушительный удар булавой наносит больше урона за каждый блок падения.",
    ),
    (
        "depth_strider",
        "Walk faster underwater.",
        "Ускоряет ходьбу под водой.",
    ),
    (
        "efficiency",
        "Mine blocks faster with the right tool.",
        "Ускоряет добычу блоков подходящим инструментом.",
    ),
    (
        "feather_falling",
        "Reduces fall damage, ender pearl landings included.",
        "Снижает урон от падения, в том числе при перемещении эндер-жемчугом.",
    ),
    ("fire_aspect", "Sets the target on fire.", "Поджигает цель."),
    (
        "fire_protection",
        "Reduces fire and lava damage and shortens how long you burn.",
        "Снижает урон от огня и лавы и сокращает время горения.",
    ),
    (
        "flame",
        "Arrows set their target on fire.",
        "Стрелы поджигают цель.",
    ),
    (
        "fortune",
        "Some ores and crops drop more items.",
        "Из некоторых руд и растений выпадает больше предметов.",
    ),
    (
        "frost_walker",
        "Water under your feet freezes into frosted ice; magma blocks do not burn you.",
        "Вода под ногами замерзает в хрупкий лёд; блоки магмы не обжигают.",
    ),
    (
        "impaling",
        "The trident deals extra damage to aquatic mobs.",
        "Трезубец наносит больше урона водным существам.",
    ),
    (
        "infinity",
        "Shooting uses no regular arrows; one arrow in the inventory is enough.",
        "Обычные стрелы не тратятся; достаточно одной стрелы в инвентаре.",
    ),
    (
        "knockback",
        "Knocks targets back farther.",
        "Сильнее отбрасывает цель.",
    ),
    (
        "looting",
        "Mobs drop more loot, and rare drops come more often.",
        "Существа роняют больше добычи, а редкие предметы выпадают чаще.",
    ),
    (
        "loyalty",
        "A thrown trident flies back to you.",
        "Брошенный трезубец возвращается к владельцу.",
    ),
    (
        "luck_of_the_sea",
        "Better odds of fishing up treasure instead of junk.",
        "Повышает шанс выловить сокровище вместо мусора.",
    ),
    (
        "lunge",
        "A spear jab dashes you forward when you are on foot and well fed; costs hunger.",
        "Укол копьём делает рывок вперёд, если вы на ногах и сыты; тратит сытость.",
    ),
    ("lure", "Fish bite sooner.", "Рыба клюёт быстрее."),
    (
        "mending",
        "Experience you pick up repairs this item before it goes to your level.",
        "Подобранный опыт сначала чинит предмет, а уже потом идёт в уровень.",
    ),
    (
        "multishot",
        "Fires three projectiles at once for the cost of one.",
        "Выпускает три снаряда по цене одного.",
    ),
    (
        "piercing",
        "Crossbow arrows pass through several mobs.",
        "Стрелы арбалета пробивают несколько существ насквозь.",
    ),
    (
        "power",
        "Arrows deal more damage.",
        "Стрелы наносят больше урона.",
    ),
    (
        "projectile_protection",
        "Reduces damage from arrows, tridents, fireballs and other projectiles.",
        "Снижает урон от стрел, трезубцев, огненных шаров и других снарядов.",
    ),
    (
        "protection",
        "Reduces most kinds of damage.",
        "Снижает большинство видов урона.",
    ),
    (
        "punch",
        "Arrows knock targets back farther.",
        "Стрелы сильнее отбрасывают цель.",
    ),
    (
        "quick_charge",
        "Loads the crossbow faster.",
        "Ускоряет зарядку арбалета.",
    ),
    (
        "respiration",
        "Hold your breath longer underwater.",
        "Позволяет дольше не дышать под водой.",
    ),
    (
        "riptide",
        "In water or rain, the trident carries you along instead of being thrown.",
        "В воде или под дождём трезубец не бросается, а уносит вас за собой.",
    ),
    (
        "sharpness",
        "Extra damage to every target.",
        "Дополнительный урон по любой цели.",
    ),
    (
        "silk_touch",
        "Mined blocks drop themselves: glass, ores, grass blocks and more.",
        "Добытые блоки выпадают сами собой: стекло, руда, дёрн и другие.",
    ),
    (
        "smite",
        "Extra damage to the undead: zombies, skeletons, withers and more.",
        "Больше урона нежити: зомби, скелетам, иссушителям и другим.",
    ),
    (
        "soul_speed",
        "Run faster on soul sand and soul soil, wearing the boots down.",
        "Ускоряет бег по песку и почве душ, изнашивая ботинки.",
    ),
    (
        "sweeping_edge",
        "Sweep attacks deal more damage to the mobs around the target.",
        "Размашистые удары наносят больше урона существам вокруг цели.",
    ),
    (
        "swift_sneak",
        "Move faster while sneaking.",
        "Ускоряет передвижение крадучись.",
    ),
    (
        "thorns",
        "Attackers sometimes take damage back; the armor wears faster.",
        "Нападающий иногда получает ответный урон; броня изнашивается быстрее.",
    ),
    (
        "unbreaking",
        "The item loses durability less often.",
        "Предмет реже теряет прочность.",
    ),
    (
        "vanishing_curse",
        "The item disappears when you die instead of dropping.",
        "При вашей смерти предмет исчезает, а не выпадает.",
    ),
    (
        "wind_burst",
        "A falling mace smash bounces you back up into the air.",
        "Сокрушительный удар булавой в падении подбрасывает вас вверх.",
    ),
];

/// `minecraft:sharpness` (or `sharpness`) as its namespace and path.
fn split_id(id: &str) -> (&str, &str) {
    id.split_once(':').unwrap_or(("minecraft", id))
}

/// The built-in hint for a vanilla enchantment.
pub fn builtin(id: &str, lang: Lang) -> Option<&'static str> {
    let (namespace, path) = split_id(id);
    if namespace != "minecraft" {
        return None;
    }
    HINTS
        .iter()
        .find(|(name, _, _)| *name == path)
        .map(|&(_, en, ru)| lang.pick(en, ru))
}

/// An enchantment's hint in language `code`, `lookup` being the selected
/// language's translations (with Horizon, the game's own map: resource packs
/// layered over `en_us`). In order:
/// 1. `enchantmenthints.<ns>.<path>` from a resource pack (empty: no hint);
/// 2. the built-in text, when it is written in that language;
/// 3. `enchantment.<ns>.<path>.desc`, the key other description packs use
///    (modded enchantments, other languages);
/// 4. the built-in English text.
pub fn describe(id: &str, code: &str, lookup: impl Fn(&str) -> Option<String>) -> Option<String> {
    let (namespace, path) = split_id(id);
    if let Some(own) = lookup(&format!("{KEY_PREFIX}.{namespace}.{path}")) {
        let own = own.trim();
        return (!own.is_empty()).then(|| own.to_owned());
    }
    if let Some(text) = Lang::shipped(code).and_then(|lang| builtin(id, lang)) {
        return Some(text.to_owned());
    }
    if let Some(text) = lookup(&format!("enchantment.{namespace}.{path}.desc"))
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
    {
        return Some(text);
    }
    builtin(id, Lang::En).map(str::to_owned)
}

/// When the hints show.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    /// Under every enchantment, all the time.
    Always,
    /// Only while a Shift key is held (else, optionally, a one-line reminder).
    Shift,
}
/// Which stacks get hints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Items {
    /// Enchanted books (stored enchantments) and enchanted gear.
    All,
    /// Enchanted books only.
    Books,
}

/// The chat colours a click on the page cycles through (black left out: it
/// cannot be read on a tooltip): the name `config.json` holds, `0xRRGGBB`,
/// and the English and Russian labels.
pub const COLOURS: [(&str, u32, &str, &str); 15] = [
    ("dark_gray", 0x55_55_55, "dark gray", "тёмно-серый"),
    ("gray", 0xAA_AA_AA, "gray", "серый"),
    ("white", 0xFF_FF_FF, "white", "белый"),
    ("yellow", 0xFF_FF_55, "yellow", "жёлтый"),
    ("gold", 0xFF_AA_00, "gold", "золотой"),
    ("green", 0x55_FF_55, "green", "зелёный"),
    ("dark_green", 0x00_AA_00, "dark green", "тёмно-зелёный"),
    ("aqua", 0x55_FF_FF, "aqua", "голубой"),
    ("dark_aqua", 0x00_AA_AA, "dark aqua", "бирюзовый"),
    ("blue", 0x55_55_FF, "blue", "синий"),
    ("dark_blue", 0x00_00_AA, "dark blue", "тёмно-синий"),
    ("light_purple", 0xFF_55_FF, "light purple", "розовый"),
    ("dark_purple", 0xAA_00_AA, "dark purple", "фиолетовый"),
    ("red", 0xFF_55_55, "red", "красный"),
    ("dark_red", 0xAA_00_00, "dark red", "тёмно-красный"),
];

/// A colour as `config.json` holds it: a chat colour's name, or `#RRGGBB`.
pub fn parse_colour(colour: &str) -> Option<u32> {
    let colour = colour.trim();
    if let Some(hex) = colour.strip_prefix('#') {
        return (hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()))
            .then(|| u32::from_str_radix(hex, 16).ok())
            .flatten();
    }
    COLOURS
        .iter()
        .find(|c| c.0.eq_ignore_ascii_case(colour))
        .map(|c| c.1)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
pub struct Config {
    pub mode: Mode,
    /// In `shift` mode, a line telling the player to hold Shift.
    pub shift_reminder: bool,
    /// A chat colour's name (`dark_gray`) or `#RRGGBB`.
    pub colour: String,
    /// The widest a hint line gets, in GUI pixels; longer hints wrap. 0: never wrap.
    pub wrap_width: u32,
    pub items: Items,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Always,
            shift_reminder: true,
            colour: "dark_gray".into(),
            wrap_width: 200,
            items: Items::All,
        }
    }
}
impl Config {
    pub fn from_json(source: &str) -> Result<Self, String> {
        let config: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        config.validate()?;
        Ok(config)
    }
    /// The file the settings page writes: every key, readable by `from_json`.
    pub fn to_json(&self) -> String {
        let mut json = serde_json::to_string_pretty(self).unwrap_or_default();
        json.push('\n');
        json
    }
    pub fn validate(&self) -> Result<(), String> {
        if parse_colour(&self.colour).is_none() {
            return Err(format!(
                "colour must be a chat colour name or #RRGGBB, not {:?}",
                self.colour
            ));
        }
        if self.wrap_width != 0 && !WIDTH_RANGE.contains(&self.wrap_width) {
            return Err(format!(
                "wrapWidth must be 0 or {}..={}",
                WIDTH_RANGE.start(),
                WIDTH_RANGE.end()
            ));
        }
        Ok(())
    }
    /// The hints' colour, `0xRRGGBB`.
    pub fn rgb(&self) -> u32 {
        parse_colour(&self.colour).unwrap_or(COLOURS[0].1)
    }
}

/// `Enchantments.tooltip`'s order: the ones in `#minecraft:tooltip_order`
/// first, in its order, then the rest as the stack lists them.
pub fn in_tooltip_order<'a, S: AsRef<str>, T: AsRef<str>>(
    order: &[S],
    levels: &'a [(T, u32)],
) -> Vec<(&'a str, u32)> {
    let mut out = Vec::with_capacity(levels.len());
    for id in order {
        if let Some((e, l)) = levels.iter().find(|(e, _)| e.as_ref() == id.as_ref()) {
            out.push((e.as_ref(), *l));
        }
    }
    for (e, l) in levels {
        if !order.iter().any(|id| id.as_ref() == e.as_ref()) {
            out.push((e.as_ref(), *l));
        }
    }
    out
}

/// Where each block of lines (a stack's stored enchantments, then its
/// enchantments) sits in the tooltip: the index of its first line, searched
/// in order, each after the one before. None for a block that is not there
/// (hidden by `minecraft:tooltip_display`) or empty.
pub fn find_blocks(lines: &[String], blocks: &[Vec<String>]) -> Vec<Option<usize>> {
    let mut from = 0;
    blocks
        .iter()
        .map(|block| {
            if block.is_empty() || block.len() > lines.len() {
                return None;
            }
            let at = (from..=lines.len() - block.len())
                .find(|&i| lines[i..i + block.len()] == block[..])?;
            from = at + block.len();
            Some(at)
        })
        .collect()
}

/// Greedy word wrap of `text` at `width` (0: one line per paragraph), each
/// line starting with `indent`; a word wider than the width gets a line of
/// its own. `measure` is the font's width of a string.
pub fn wrap(text: &str, indent: &str, width: u32, measure: impl Fn(&str) -> f32) -> Vec<String> {
    let mut out = Vec::new();
    for paragraph in text.lines() {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if line.is_empty() {
                line = format!("{indent}{word}");
                continue;
            }
            let longer = format!("{line} {word}");
            if width == 0 || measure(&longer) <= width as f32 {
                line = longer;
            } else {
                out.push(std::mem::replace(&mut line, format!("{indent}{word}")));
            }
        }
        if !line.is_empty() {
            out.push(line);
        }
    }
    out
}

/// Lines to put in the tooltip, under its line `after`.
#[derive(Clone, Debug, PartialEq)]
pub struct Insert {
    pub after: usize,
    pub lines: Vec<String>,
}

/// The reminder shown in `shift` mode while Shift is up.
pub fn reminder(lang: Lang) -> &'static str {
    lang.pick(
        "Hold Shift for enchantment hints",
        "Зажмите Shift, чтобы увидеть подсказки к чарам",
    )
}

/// What a tooltip gets. `lines` is the tooltip as plain text; `blocks` the
/// stack's stored enchantments and enchantments, each as `(id, its line)` in
/// tooltip order; `describe` an enchantment's hint. Sorted by `after`.
pub fn plan(
    lines: &[String],
    blocks: &[Vec<(String, String)>],
    config: &Config,
    shift_down: bool,
    lang: Lang,
    mut describe: impl FnMut(&str) -> Option<String>,
    measure: impl Fn(&str) -> f32,
) -> Vec<Insert> {
    let texts: Vec<Vec<String>> = blocks
        .iter()
        .map(|b| b.iter().map(|(_, line)| line.clone()).collect())
        .collect();
    let mut hints = Vec::new();
    for (block, at) in blocks.iter().zip(find_blocks(lines, &texts)) {
        let Some(at) = at else { continue };
        for (offset, (id, _)) in block.iter().enumerate() {
            if let Some(hint) = describe(id) {
                hints.push((at + offset, hint));
            }
        }
    }
    let Some(&(last, _)) = hints.last() else {
        return Vec::new();
    };
    let width = config.wrap_width;
    if config.mode == Mode::Shift && !shift_down {
        if !config.shift_reminder {
            return Vec::new();
        }
        // Under the last enchantment that has a hint.
        return vec![Insert {
            after: last,
            lines: wrap(reminder(lang), "", width, &measure),
        }];
    }
    hints
        .into_iter()
        .map(|(after, hint)| Insert {
            after,
            lines: wrap(&hint, INDENT, width, &measure),
        })
        .filter(|insert| !insert.lines.is_empty())
        .collect()
}

/// `lines` with `inserts` (from `plan`) put in.
pub fn apply<T: Clone>(lines: &[T], inserts: &[Insert], make: impl Fn(&str) -> T) -> Vec<T> {
    let mut out = Vec::with_capacity(lines.len() + inserts.len());
    for (i, line) in lines.iter().enumerate() {
        out.push(line.clone());
        for insert in inserts.iter().filter(|insert| insert.after == i) {
            out.extend(insert.lines.iter().map(|l| make(l)));
        }
    }
    out
}

/// The configuration as read from [`CONFIG_FILE`]: Err when the file is broken
/// (no hints then, until it is fixed or reset).
pub type Loaded = Result<Config, String>;

/// A button of the settings page (Options > Mods > Enchantment Hints > Settings).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setting {
    /// The file is broken: its error. A click reads it again.
    Broken,
    Mode,
    /// Shown in `shift` mode only.
    Reminder,
    Colour,
    Width,
    Items,
    /// Every setting back to its default.
    Defaults,
}
/// What a click on the settings page asks of the game.
#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    /// Write this configuration to the file; it applies at once.
    Save(Config),
    /// Read the file again (after editing it by hand).
    Reload,
}

fn on_off(on: bool, lang: Lang) -> &'static str {
    match (on, lang) {
        (true, Lang::En) => "ON",
        (false, Lang::En) => "OFF",
        (true, Lang::Ru) => "ВКЛ",
        (false, Lang::Ru) => "ВЫКЛ",
    }
}

/// The page's buttons, in order.
pub fn page(loaded: &Loaded) -> Vec<Setting> {
    let Ok(c) = loaded else {
        return vec![Setting::Broken, Setting::Defaults];
    };
    let mut page = vec![Setting::Mode];
    if c.mode == Mode::Shift {
        page.push(Setting::Reminder);
    }
    page.extend([
        Setting::Colour,
        Setting::Width,
        Setting::Items,
        Setting::Defaults,
    ]);
    page
}

impl Setting {
    /// Its button's label.
    pub fn label(self, loaded: &Loaded, lang: Lang) -> String {
        let c = match loaded {
            Ok(c) => c,
            Err(error) => {
                return match self {
                    Self::Defaults => lang.pick("Reset to defaults", "Сбросить настройки").into(),
                    _ => {
                        let mut short: String = error.chars().take(ERROR_WIDTH).collect();
                        if short.len() < error.len() {
                            short = format!("{}...", short.trim_end());
                        }
                        let reload = lang.pick("click to reload", "нажмите, чтобы перечитать");
                        format!("{CONFIG_FILE}: {short} ({reload})")
                    }
                };
            }
        };
        match self {
            Self::Broken => String::new(),
            Self::Mode => match c.mode {
                Mode::Always => lang.pick("Show hints: always", "Подсказки: всегда"),
                Mode::Shift => lang.pick(
                    "Show hints: while Shift is held",
                    "Подсказки: пока зажат Shift",
                ),
            }
            .into(),
            Self::Reminder => format!(
                "{}: {}",
                lang.pick("Hold-Shift reminder", "Напоминание про Shift"),
                on_off(c.shift_reminder, lang)
            ),
            Self::Colour => {
                let named = COLOURS
                    .iter()
                    .find(|n| n.0.eq_ignore_ascii_case(c.colour.trim()));
                let name = match named {
                    Some(&(_, _, en, ru)) => lang.pick(en, ru).to_owned(),
                    None => c.colour.trim().to_ascii_uppercase(),
                };
                format!("{}: {name}", lang.pick("Colour", "Цвет"))
            }
            Self::Width if c.wrap_width == 0 => lang
                .pick("Line width: no wrapping", "Ширина строки: без переноса")
                .into(),
            Self::Width => format!(
                "{}: {} px",
                lang.pick("Line width", "Ширина строки"),
                c.wrap_width
            ),
            Self::Items => match c.items {
                Items::All => lang.pick("Items: books and gear", "Предметы: книги и снаряжение"),
                Items::Books => lang.pick(
                    "Items: enchanted books only",
                    "Предметы: только зачарованные книги",
                ),
            }
            .into(),
            Self::Defaults => lang.pick("Reset to defaults", "Сбросить настройки").into(),
        }
    }

    /// What a click on it does to `loaded`.
    pub fn click(self, loaded: &Loaded) -> Effect {
        let Ok(c) = loaded else {
            return match self {
                Self::Defaults => Effect::Save(Config::default()),
                _ => Effect::Reload,
            };
        };
        let mut c = c.clone();
        match self {
            Self::Broken => return Effect::Reload,
            Self::Mode => {
                c.mode = match c.mode {
                    Mode::Always => Mode::Shift,
                    Mode::Shift => Mode::Always,
                }
            }
            Self::Reminder => c.shift_reminder = !c.shift_reminder,
            Self::Colour => {
                // The next named colour; a `#RRGGBB` goes back to the first.
                let now = COLOURS
                    .iter()
                    .position(|n| n.0.eq_ignore_ascii_case(c.colour.trim()));
                let next = now.map_or(0, |i| (i + 1) % COLOURS.len());
                c.colour = COLOURS[next].0.into();
            }
            Self::Width => {
                // The next step; a width between two steps goes to the one above.
                let now = c.wrap_width;
                let next = match now {
                    0 => WIDTH_STEPS[0],
                    _ => WIDTH_STEPS
                        .iter()
                        .copied()
                        .find(|&w| w == 0 || w > now)
                        .unwrap_or(0),
                };
                c.wrap_width = next;
            }
            Self::Items => {
                c.items = match c.items {
                    Items::All => Items::Books,
                    Items::Books => Items::All,
                }
            }
            Self::Defaults => c = Config::default(),
        }
        Effect::Save(c)
    }
}

/// The settings page's labels (`native_settings`).
pub fn labels(loaded: &Loaded, lang: Lang) -> Vec<String> {
    page(loaded)
        .into_iter()
        .map(|s| s.label(loaded, lang))
        .collect()
}
/// A click on button `button` of the page `labels` gave (`native_settings_click`).
pub fn click(loaded: &Loaded, button: usize) -> Option<Effect> {
    page(loaded).get(button).map(|s| s.click(loaded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// `data/minecraft/tags/enchantment/tooltip_order.json` of Minecraft 26.3:
    /// every enchantment the game has.
    const VANILLA: [&str; 43] = [
        "minecraft:binding_curse",
        "minecraft:vanishing_curse",
        "minecraft:riptide",
        "minecraft:channeling",
        "minecraft:wind_burst",
        "minecraft:frost_walker",
        "minecraft:lunge",
        "minecraft:sharpness",
        "minecraft:smite",
        "minecraft:bane_of_arthropods",
        "minecraft:impaling",
        "minecraft:power",
        "minecraft:density",
        "minecraft:breach",
        "minecraft:piercing",
        "minecraft:sweeping_edge",
        "minecraft:multishot",
        "minecraft:fire_aspect",
        "minecraft:flame",
        "minecraft:knockback",
        "minecraft:punch",
        "minecraft:protection",
        "minecraft:blast_protection",
        "minecraft:fire_protection",
        "minecraft:projectile_protection",
        "minecraft:feather_falling",
        "minecraft:fortune",
        "minecraft:looting",
        "minecraft:silk_touch",
        "minecraft:luck_of_the_sea",
        "minecraft:efficiency",
        "minecraft:quick_charge",
        "minecraft:lure",
        "minecraft:respiration",
        "minecraft:aqua_affinity",
        "minecraft:soul_speed",
        "minecraft:swift_sneak",
        "minecraft:depth_strider",
        "minecraft:thorns",
        "minecraft:loyalty",
        "minecraft:unbreaking",
        "minecraft:infinity",
        "minecraft:mending",
    ];

    /// Every character 6 pixels wide (close enough to the game font for tests).
    fn measure(s: &str) -> f32 {
        s.chars().count() as f32 * 6.0
    }
    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }
    fn no_lang(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn every_vanilla_enchantment_has_both_languages() {
        for id in VANILLA {
            for lang in [Lang::En, Lang::Ru] {
                let hint = builtin(id, lang).unwrap_or_else(|| panic!("{id} has no hint"));
                assert!(!hint.trim().is_empty(), "{id}");
                assert!(!hint.contains('§'), "{id}: formatting codes");
                assert!(hint.ends_with('.'), "{id}: {hint}");
            }
        }
        assert_eq!(
            HINTS.len(),
            VANILLA.len(),
            "a hint for a missing enchantment"
        );
        let mut names: Vec<_> = HINTS.iter().map(|h| h.0).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), HINTS.len(), "a duplicate hint");
        // The Russian text is Russian, and not just the English text again.
        for &(name, en, ru) in &HINTS {
            assert_ne!(en, ru, "{name}");
            assert!(ru.chars().any(|c| ('а'..='я').contains(&c)), "{name}");
        }
        // Sorted, so a missing one is easy to spot.
        assert!(HINTS.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn languages_pick_their_text() {
        assert_eq!(Lang::shipped("en_us"), Some(Lang::En));
        assert_eq!(Lang::shipped("en_gb"), Some(Lang::En));
        assert_eq!(Lang::shipped("ru_ru"), Some(Lang::Ru));
        assert_eq!(Lang::shipped("de_de"), None);
        assert_eq!(Lang::for_code("de_de"), Lang::En);
        let ru = describe("minecraft:mending", "ru_ru", no_lang).unwrap();
        assert!(ru.starts_with("Подобранный опыт"));
        let en = describe("mending", "en_us", no_lang).unwrap();
        assert!(en.starts_with("Experience you pick up"));
        // A language without built-in text falls back to English.
        assert_eq!(describe("minecraft:mending", "de_de", no_lang).unwrap(), en);
        // Not ours, nothing in the language: no hint.
        assert_eq!(describe("othermod:zap", "en_us", no_lang), None);
    }

    #[test]
    fn resource_packs_override_and_extend() {
        let map: HashMap<&str, &str> = HashMap::from([
            ("enchantmenthints.minecraft.sharpness", "Cuts deeper."),
            ("enchantmenthints.minecraft.smite", "  "),
            (
                "enchantment.minecraft.unbreaking.desc",
                "Other pack's text.",
            ),
            ("enchantment.othermod.zap.desc", "Zaps the target."),
            ("enchantmenthints.othermod.boom", "Booms."),
        ]);
        let lookup = |k: &str| map.get(k).map(|v| (*v).to_owned());
        // Our own key wins, in any language; empty hides the hint.
        assert_eq!(
            describe("minecraft:sharpness", "ru_ru", lookup).as_deref(),
            Some("Cuts deeper.")
        );
        assert_eq!(describe("minecraft:smite", "en_us", lookup), None);
        // Built-in text beats another pack's `.desc` in the languages it covers…
        assert_eq!(
            describe("minecraft:unbreaking", "en_us", lookup).as_deref(),
            Some("The item loses durability less often.")
        );
        // …and `.desc` fills in for other languages and modded enchantments.
        assert_eq!(
            describe("minecraft:unbreaking", "de_de", lookup).as_deref(),
            Some("Other pack's text.")
        );
        assert_eq!(
            describe("othermod:zap", "en_us", lookup).as_deref(),
            Some("Zaps the target.")
        );
        assert_eq!(
            describe("othermod:boom", "fr_fr", lookup).as_deref(),
            Some("Booms.")
        );
    }

    #[test]
    fn config_round_trips_and_checks_values() {
        let c = Config::default();
        assert_eq!(Config::from_json(&c.to_json()).unwrap(), c);
        assert_eq!(Config::from_json("{}").unwrap(), c);
        let c = Config::from_json(
            r##"{"mode":"shift","shiftReminder":false,"colour":"#12abEF","wrapWidth":0,"items":"books"}"##,
        )
        .unwrap();
        assert_eq!(c.mode, Mode::Shift);
        assert_eq!(c.rgb(), 0x12ABEF);
        assert_eq!(c.items, Items::Books);
        assert_eq!(
            Config::from_json(r#"{"colour":"GOLD"}"#).unwrap().rgb(),
            0xFFAA00
        );
        assert!(Config::from_json(r#"{"colour":"black"}"#).is_err());
        assert!(Config::from_json(r##"{"colour":"#12345"}"##).is_err());
        assert!(Config::from_json(r#"{"wrapWidth":10}"#).is_err());
        assert!(Config::from_json(r#"{"wrapWidth":5000}"#).is_err());
        assert!(Config::from_json(r#"{"mode":"never"}"#).is_err());
        assert!(
            Config::from_json(r#"{"color":"gold"}"#).is_err(),
            "unknown key"
        );
        assert!(Config::from_json("not json").is_err());
    }

    #[test]
    fn order_follows_the_tooltip_tag() {
        let order = [
            "minecraft:sharpness",
            "minecraft:unbreaking",
            "minecraft:mending",
        ];
        let levels = [
            ("modded:zap".to_owned(), 2),
            ("minecraft:mending".to_owned(), 1),
            ("minecraft:sharpness".to_owned(), 5),
        ];
        assert_eq!(
            in_tooltip_order(&order, &levels),
            vec![
                ("minecraft:sharpness", 5),
                ("minecraft:mending", 1),
                ("modded:zap", 2)
            ]
        );
    }

    #[test]
    fn blocks_are_found_in_order() {
        let lines = strings(&["Sharpness V", "Mending", "Sharpness V", "Mending", "Lore"]);
        let found = find_blocks(
            &lines,
            &[
                strings(&["Sharpness V", "Mending"]),
                strings(&["Sharpness V"]),
            ],
        );
        assert_eq!(found, vec![Some(0), Some(2)]);
        // A hidden block is skipped; the next one is still found.
        let found = find_blocks(
            &lines,
            &[strings(&["Unbreaking III"]), strings(&["Mending"])],
        );
        assert_eq!(found, vec![None, Some(1)]);
        assert_eq!(find_blocks(&lines, &[Vec::new()]), vec![None]);
        let long = strings(&["a", "b", "c", "d", "e", "f"]);
        assert_eq!(find_blocks(&lines, &[long]), vec![None]);
    }

    #[test]
    fn wrapping_keeps_words_and_indent() {
        let text = "Reduces damage from arrows, tridents and fireballs.";
        assert_eq!(wrap(text, " ", 0, measure), vec![format!(" {text}")]);
        let lines = wrap(text, " ", 120, measure);
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(line.starts_with(' ') && !line.starts_with("  "));
            assert!(measure(line) <= 120.0 || !line.trim().contains(' '));
        }
        let joined: Vec<&str> = lines.iter().flat_map(|l| l.split_whitespace()).collect();
        assert_eq!(joined.join(" "), text);
        // A word wider than the line keeps a line of its own.
        assert_eq!(
            wrap("a verylongword b", "", 30, measure),
            strings(&["a", "verylongword", "b"])
        );
        assert_eq!(wrap("one\ntwo", "", 0, measure), strings(&["one", "two"]));
        assert!(wrap("   ", " ", 100, measure).is_empty());
    }

    fn book() -> (Vec<String>, Vec<Vec<(String, String)>>) {
        let lines = strings(&["Enchanted Book", "Sharpness V", "Mending", "Lore line"]);
        let blocks = vec![
            vec![
                ("minecraft:sharpness".to_owned(), "Sharpness V".to_owned()),
                ("minecraft:mending".to_owned(), "Mending".to_owned()),
            ],
            Vec::new(),
        ];
        (lines, blocks)
    }
    fn en(id: &str) -> Option<String> {
        describe(id, "en_us", no_lang)
    }
    fn wide() -> Config {
        Config {
            wrap_width: 0,
            ..Config::default()
        }
    }

    #[test]
    fn hints_go_under_each_enchantment() {
        let (lines, blocks) = book();
        let inserts = plan(&lines, &blocks, &wide(), false, Lang::En, en, measure);
        assert_eq!(
            inserts,
            vec![
                Insert {
                    after: 1,
                    lines: vec![" Extra damage to every target.".into()]
                },
                Insert {
                    after: 2,
                    lines: vec![
                        " Experience you pick up repairs this item before it goes to your level."
                            .into()
                    ]
                },
            ]
        );
        let out = apply(&lines, &inserts, str::to_owned);
        assert_eq!(out.len(), 6);
        assert_eq!(out[0], "Enchanted Book");
        assert_eq!(out[1], "Sharpness V");
        assert_eq!(out[2], " Extra damage to every target.");
        assert_eq!(out[3], "Mending");
        assert_eq!(out[5], "Lore line");
    }

    #[test]
    fn shift_mode_shows_a_reminder_until_shift_is_held() {
        let (lines, blocks) = book();
        let shift = Config {
            mode: Mode::Shift,
            ..wide()
        };
        let up = plan(&lines, &blocks, &shift, false, Lang::Ru, en, measure);
        assert_eq!(
            up,
            vec![Insert {
                after: 2,
                lines: vec![reminder(Lang::Ru).to_owned()]
            }]
        );
        let down = plan(&lines, &blocks, &shift, true, Lang::En, en, measure);
        assert_eq!(down.len(), 2);
        let quiet = Config {
            shift_reminder: false,
            ..shift
        };
        assert!(plan(&lines, &blocks, &quiet, false, Lang::En, en, measure).is_empty());
    }

    #[test]
    fn nothing_without_a_known_enchantment() {
        let lines = strings(&["Stick", "Zap II"]);
        let blocks = vec![vec![("othermod:zap".to_owned(), "Zap II".to_owned())]];
        let shift = Config {
            mode: Mode::Shift,
            ..Config::default()
        };
        // No hint, so not even the reminder.
        assert!(plan(&lines, &blocks, &shift, false, Lang::En, en, measure).is_empty());
        // Hidden enchantments (not in the lines) get nothing either.
        let (_, blocks) = book();
        let plain = strings(&["Enchanted Book"]);
        assert!(
            plan(
                &plain,
                &blocks,
                &Config::default(),
                true,
                Lang::En,
                en,
                measure
            )
            .is_empty()
        );
    }

    #[test]
    fn gear_and_books_blocks_both_get_hints() {
        let lines = strings(&[
            "Sword",
            "Unbreaking III",
            "Sharpness V",
            "When in Main Hand:",
        ]);
        let blocks = vec![
            vec![(
                "minecraft:unbreaking".to_owned(),
                "Unbreaking III".to_owned(),
            )],
            vec![("minecraft:sharpness".to_owned(), "Sharpness V".to_owned())],
        ];
        let inserts = plan(&lines, &blocks, &wide(), false, Lang::En, en, measure);
        assert_eq!(
            inserts.iter().map(|i| i.after).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn long_hints_wrap_at_the_width() {
        let (lines, blocks) = book();
        let inserts = plan(
            &lines,
            &blocks,
            &Config::default(),
            false,
            Lang::En,
            en,
            measure,
        );
        let mending = &inserts[1];
        assert!(mending.lines.len() > 1);
        assert!(mending.lines.iter().all(|l| l.starts_with(INDENT)));
    }

    #[test]
    fn the_page_cycles_every_setting() {
        let loaded: Loaded = Ok(Config::default());
        let labels_en = labels(&loaded, Lang::En);
        assert_eq!(
            labels_en,
            strings(&[
                "Show hints: always",
                "Colour: dark gray",
                "Line width: 200 px",
                "Items: books and gear",
                "Reset to defaults",
            ])
        );
        assert_eq!(labels(&loaded, Lang::Ru)[0], "Подсказки: всегда");
        // Mode: always -> shift, which shows the reminder button.
        let Some(Effect::Save(shift)) = click(&loaded, 0) else {
            panic!()
        };
        assert_eq!(shift.mode, Mode::Shift);
        let shift: Loaded = Ok(shift);
        assert_eq!(page(&shift)[1], Setting::Reminder);
        assert_eq!(labels(&shift, Lang::En)[1], "Hold-Shift reminder: ON");
        let Some(Effect::Save(quiet)) = click(&shift, 1) else {
            panic!()
        };
        assert!(!quiet.shift_reminder);
        // Colour cycles through all names and back.
        let mut c = Config::default();
        for _ in 0..COLOURS.len() {
            let Effect::Save(next) = Setting::Colour.click(&Ok(c.clone())) else {
                panic!()
            };
            assert_ne!(next.colour, c.colour);
            c = next;
        }
        assert_eq!(c.colour, Config::default().colour);
        let custom: Loaded = Ok(Config {
            colour: "#a0b0c0".into(),
            ..Config::default()
        });
        assert_eq!(Setting::Colour.label(&custom, Lang::En), "Colour: #A0B0C0");
        let Effect::Save(named) = Setting::Colour.click(&custom) else {
            panic!()
        };
        assert_eq!(named.colour, COLOURS[0].0);
        // Width steps, an in-between value goes up, 0 (no wrap) comes back round.
        let mut widths = Vec::new();
        let mut c = Config::default();
        for _ in 0..WIDTH_STEPS.len() {
            let Effect::Save(next) = Setting::Width.click(&Ok(c)) else {
                panic!()
            };
            widths.push(next.wrap_width);
            c = next;
        }
        assert_eq!(widths, vec![240, 320, 0, 120, 160, 200]);
        let odd: Loaded = Ok(Config {
            wrap_width: 170,
            ..Config::default()
        });
        assert_eq!(
            Setting::Width.click(&odd),
            Effect::Save(Config {
                wrap_width: 200,
                ..Config::default()
            })
        );
        let odd: Loaded = Ok(Config {
            wrap_width: 900,
            ..Config::default()
        });
        let Effect::Save(c) = Setting::Width.click(&odd) else {
            panic!()
        };
        assert_eq!(c.wrap_width, 0);
        let Some(Effect::Save(books)) = click(&loaded, 3) else {
            panic!()
        };
        assert_eq!(books.items, Items::Books);
        assert_eq!(click(&loaded, 4), Some(Effect::Save(Config::default())));
        assert_eq!(click(&loaded, 5), None);
        // Every page's labels are saved as valid files.
        for button in 0..page(&loaded).len() {
            if let Some(Effect::Save(c)) = click(&loaded, button) {
                assert_eq!(Config::from_json(&c.to_json()).unwrap(), c);
            }
        }
    }

    #[test]
    fn a_broken_file_shows_its_error_and_a_reset() {
        let broken: Loaded = Err("expected value at line 1 column 1 and quite a lot more".into());
        let labels = labels(&broken, Lang::En);
        assert_eq!(labels.len(), 2);
        assert!(labels[0].starts_with("config.json: expected value"));
        assert!(labels[0].ends_with("... (click to reload)"));
        assert_eq!(click(&broken, 0), Some(Effect::Reload));
        assert_eq!(click(&broken, 1), Some(Effect::Save(Config::default())));
    }
}
