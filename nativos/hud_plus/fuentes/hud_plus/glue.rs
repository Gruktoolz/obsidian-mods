// HUD+: the game side. Everything that reads Horizon's private state, builds
// the snapshot the renderer draws and forwards the screen's input lives here;
// the renderer itself is `official_hud_plus`, which has no game references and
// is unit-tested on its own.
//
// The module lives inside `game`, beside `Game` itself, so the helpers the
// game calls take the private `Game`. They are `pub(super)` — visible in
// `game` and its descendants, exactly `Game`'s own visibility — so the
// interface lint sees no more-public item taking a private type.
use super::official_hud_plus as m;
use m::Painter as _;

use crate::game::Game;
use horizon_client::hud::HudBatch;
use horizon_client::mod_api::NativeScreenView;
use horizon_client::text::Text;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Configuration files
// ---------------------------------------------------------------------------

pub(super) fn config_path() -> Option<PathBuf> {
    Game::native_config_dir()
        .ok()
        .map(|dir| dir.join(m::CONFIG_FILE))
}

pub(super) fn config() -> m::Config {
    let path = config_path();
    m::config(|| path.map(|p| m::Config::load(&p)).unwrap_or_default())
}

pub(super) fn save(cfg: &m::Config) {
    if let Some(path) = config_path() {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = cfg.save(&path);
    }
}

fn commit() {
    let cfg = m::config(m::Config::default);
    save(&cfg);
}

// ---------------------------------------------------------------------------
// The mod's own screen
// ---------------------------------------------------------------------------

pub(super) fn open(game: &mut Game<'_>, mode: m::ScreenMode) -> bool {
    if !game.native_on(m::ID) || !game.native_window() {
        return false;
    }
    if game.native_mod_screen() == Some(m::ID) {
        m::set_mode(mode);
        return true;
    }
    if !game.native_playing() || !game.native_mod_screen_open(m::ID) {
        return false;
    }
    m::set_mode(mode);
    let cfg = config();
    let selected = m::ModuleId::ALL
        .iter()
        .copied()
        .find(|id| cfg.module(*id).enabled);
    m::edit_editor(|e| {
        e.selected = selected;
        e.drag = None;
        e.scroll = 0;
    });
    true
}

/// Closes or switches the screen a settings row asked for.
pub(super) fn apply_pending(game: &mut Game<'_>) {
    let Some(mode) = m::take_pending() else {
        return;
    };
    if mode == m::ScreenMode::Closed {
        game.native_mod_screen_close(m::ID);
        m::set_mode(m::ScreenMode::Closed);
        return;
    }
    if game.native_mod_screen() == Some(m::ID) {
        m::set_mode(mode);
    } else {
        open(game, mode);
    }
}

/// Keeps the state in step with the game closing the screen (Escape, another
/// screen opening, leaving the world).
pub(super) fn sync(game: &Game<'_>) {
    if m::mode() != m::ScreenMode::Closed && game.native_mod_screen() != Some(m::ID) {
        m::set_mode(m::ScreenMode::Closed);
        m::edit_editor(|e| e.drag = None);
    }
}

// ---------------------------------------------------------------------------
// The snapshot
// ---------------------------------------------------------------------------

fn item_name(id: &str) -> String {
    let (namespace, path) = id.split_once(':').unwrap_or(("minecraft", id));
    for prefix in ["item", "block"] {
        let key = format!("{prefix}.{namespace}.{path}");
        let name = horizon_client::lang::get_or(&key, "");
        if !name.is_empty() {
            return name;
        }
    }
    m::pretty_id(id)
}

fn item_view(game: &Game<'_>, stack: &horizon_client::items::ItemStack) -> m::ItemView {
    let max_damage = game
        .res
        .items()
        .get(&stack.item)
        .map_or(0, |info| info.max_damage);
    m::ItemView {
        id: stack.item.to_string(),
        name: item_name(&stack.item),
        count: stack.count,
        damage: stack.damage,
        max_damage,
    }
}

fn mapping(game: &mut Game<'_>, name: &str) -> (bool, String) {
    game.native_mapping(name)
        .map_or((false, String::new()), |(down, label)| (down, label))
}

/// The local clock and date, in the player's time zone.
fn real_time() -> ([u32; 3], [u32; 3]) {
    use chrono::{Datelike, Timelike};
    let now = chrono::Local::now();
    (
        [now.hour(), now.minute(), now.second()],
        [
            now.year().clamp(0, 9999) as u32,
            now.month(),
            now.day(),
        ],
    )
}

thread_local! {
    static LAST_SNAPSHOT: std::cell::RefCell<Option<m::Snapshot>> =
        const { std::cell::RefCell::new(None) };
}

fn store_snapshot(s: &m::Snapshot) {
    LAST_SNAPSHOT.with(|c| *c.borrow_mut() = Some(s.clone()));
}

/// The last snapshot, for the `&self` screen hooks, which cannot refresh it.
pub(super) fn cached_snapshot() -> m::Snapshot {
    LAST_SNAPSHOT.with(|c| c.borrow().clone()).unwrap_or_default()
}

pub(super) fn snapshot(game: &mut Game<'_>) -> m::Snapshot {
    let mut s = m::Snapshot {
        screen: game.screen.id(),
        ..m::Snapshot::default()
    };
    let Some(player) = game.native_player() else {
        store_snapshot(&s);
        return s;
    };
    s.in_world = true;
    s.position = [player.x, player.y, player.z];
    s.yaw = player.yaw;
    s.pitch = player.pitch;
    s.dimension = player.dimension.to_owned();

    if let Some(p) = game.player.as_mut() {
        s.health = p.health.health();
        s.max_health = p
            .attribute("minecraft:max_health")
            .unwrap_or(20.0) as f32;
        s.absorption = p.health.absorption();
        s.armor_points = p
            .attribute("minecraft:armor")
            .or_else(|_| p.attribute("armor"))
            .unwrap_or(0.0) as i32;
        s.food = p.health.food.food;
        s.saturation = p.health.food.saturation;
        s.exhaustion = p.health.food.exhaustion;
        s.regen_ticks = p.health.food.timer;
        s.air = p.air;
        s.max_air = 300;
        s.speed =
            (p.velocity.x * p.velocity.x + p.velocity.z * p.velocity.z).sqrt() * 20.0;
        s.survival = !p.spectator && !game.creative;
        s.creative = game.creative;
    }

    s.xp_level = game.experience.level;
    s.xp_progress = game.experience.progress;

    if let Some(clock) = game.native_clock() {
        s.game_time = clock.game_time;
        s.day_time = clock.day_time;
    }
    let (fps, low_fps, _) = game.native_frame_rate();
    s.fps = fps;
    s.low_fps = low_fps;
    s.frame_ms = if fps == 0 {
        0.0
    } else {
        1000.0 / fps as f32
    };
    s.cps = game.native_clicks();

    s.selected = game.selected;
    s.inventory = game
        .inventory
        .iter()
        .map(|stack| stack.as_ref().map(|st| item_view(game, st)))
        .collect();
    s.armor_items = game
        .armor
        .iter()
        .map(|stack| stack.as_ref().map(|st| item_view(game, st)))
        .collect();
    s.offhand = game.offhand.as_ref().map(|st| item_view(game, st));
    s.held = game.held().map(|st| item_view(game, st));
    s.held_food = game
        .held()
        .and_then(|st| game.res.items().get(&st.item))
        .and_then(|info| info.food);
    s.held_food_name = game
        .held()
        .map_or_else(String::new, |st| item_name(&st.item));

    s.effects = game
        .potions
        .effects
        .list
        .iter()
        .map(|e| {
            let def = horizon_client_core::potions::effect(e.id);
            m::EffectView {
                id: e.id.to_owned(),
                name: horizon_client::lang::get_or(
                    &horizon_client_core::potions::description_id(e.id),
                    &m::pretty_id(e.id),
                ),
                sprite: horizon_client_core::potions::sprite(e.id),
                amplifier: e.amplifier,
                duration: e.duration,
                ambient: e.ambient,
                beneficial: def.is_some_and(|d| {
                    d.category == horizon_client_core::potions::Category::Beneficial
                }),
            }
        })
        .collect();
    s.arrows = s.arrows_in_inventory();
    s.totems = s.totems_in_inventory();

    let (forward, fl) = mapping(game, "key.forward");
    let (back, bl) = mapping(game, "key.back");
    let (left, ll) = mapping(game, "key.left");
    let (right, rl) = mapping(game, "key.right");
    let (jump, jl) = mapping(game, "key.jump");
    let (sneak, sl) = mapping(game, "key.sneak");
    let (sprint, spl) = mapping(game, "key.sprint");
    let (attack, al) = mapping(game, "key.attack");
    let (use_item, ul) = mapping(game, "key.use");
    s.keys = m::KeysView {
        forward,
        back,
        left,
        right,
        jump,
        sneak,
        sprint,
        attack,
        use_: use_item,
        labels: vec![fl, bl, ll, rl, jl, sl, spl, al, ul],
        mouse: [attack, false, use_item],
    };

    let (time, date) = real_time();
    s.real_time = time;
    s.real_date = date;
    store_snapshot(&s);
    s
}

// ---------------------------------------------------------------------------
// Painting through a HudBatch
// ---------------------------------------------------------------------------

pub(super) struct GamePainter<'a, 'g> {
    pub game: &'a Game<'g>,
    pub batch: &'a mut HudBatch,
}

impl m::Painter for GamePainter<'_, '_> {
    fn screen(&self) -> (f32, f32) {
        (self.batch.width, self.batch.height)
    }
    fn width(&self, text: &str) -> f32 {
        self.game.native_text_width(text)
    }
    fn fill(&mut self, rect: m::Rect, color: u32) {
        self.game
            .native_fill(self.batch, [rect.x, rect.y, rect.w, rect.h], color);
    }
    fn text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        color: u32,
        shadow: bool,
        scale: f32,
    ) -> f32 {
        let width = self.game.native_text_width(text) * scale;
        if (scale - 1.0).abs() < 0.001 {
            self.game.native_text(self.batch, text, [x, y], color, shadow);
        } else {
            let start = self.batch.vertices.len();
            self.game.native_text(self.batch, text, [x, y], color, shadow);
            self.batch.scale_since(start, x, y, scale, scale);
        }
        width
    }
    fn sprite(&mut self, name: &str, rect: m::Rect) -> bool {
        if self.sprite_named(name, rect) {
            return true;
        }
        // Vanilla's GUI sprites are packed under `gui/sprites/`; effect icons
        // (`mob_effect/...`) sit at the atlas root.
        if !name.starts_with("gui/sprites/") && !name.starts_with("mob_effect/") {
            let prefixed = format!("gui/sprites/{name}");
            return self.sprite_named(&prefixed, rect);
        }
        false
    }
    fn sprite_region(&mut self, name: &str, src: [f32; 4], rect: m::Rect, color: u32) -> bool {
        self.sprite_at(name, Some(src), rect, color)
    }
    fn sprite_tinted(&mut self, name: &str, rect: m::Rect, color: u32) -> bool {
        self.sprite_at(name, None, rect, color)
    }
    fn item(&mut self, item: &m::ItemView, x: f32, y: f32, size: f32) -> bool {
        self.game.native_item(self.batch, &item.id, [x, y], size)
    }
}

impl GamePainter<'_, '_> {
    /// The sprite's place in the atlas, trying vanilla's `gui/sprites/` prefix.
    fn atlas_sprite(&self, name: &str) -> Option<horizon_client::hud::Rect> {
        let atlas = &self.game.gui.atlas;
        if let Some(r) = atlas.sprites.get(name) {
            return Some(*r);
        }
        if !name.starts_with("gui/sprites/") && !name.starts_with("mob_effect/") {
            return atlas.sprites.get(&format!("gui/sprites/{name}")).copied();
        }
        None
    }

    /// `native_sprite` with a tint, and with an optional crop: the mod draws
    /// the saturation outlines and the flashing food AppleSkin's way.
    fn sprite_at(&mut self, name: &str, src: Option<[f32; 4]>, rect: m::Rect, color: u32) -> bool {
        let Some(mut r) = self.atlas_sprite(name) else {
            return false;
        };
        let (du, dv) = ((r.u1 - r.u0) / r.w as f32, (r.v1 - r.v0) / r.h as f32);
        if let Some([x, y, w, h]) = src {
            r.u0 += x * du;
            r.v0 += y * dv;
            r.u1 = r.u0 + w * du;
            r.v1 = r.v0 + h * dv;
        }
        self.batch
            .rect_uv(rect.x, rect.y, rect.w, rect.h, r, color);
        true
    }

    /// One sprite, cropped to its whole texture: `native_sprite`'s source
    /// rectangle is in the sprite's own pixels, not fractions.
    fn sprite_named(&mut self, name: &str, rect: m::Rect) -> bool {
        let Some([sw, sh]) = self.game.native_sprite_size(name) else {
            return false;
        };
        if sw <= 0.0 || sh <= 0.0 {
            return false;
        }
        self.game.native_sprite(
            self.batch,
            name,
            [0.0, 0.0, sw, sh],
            [rect.x, rect.y, rect.w, rect.h],
        )
    }
}

// ---------------------------------------------------------------------------
// Drawing the HUD and the mod's screens
// ---------------------------------------------------------------------------

pub(super) fn draw_hud(game: &mut Game<'_>, batch: &mut HudBatch) {
    let cfg = config();
    if !cfg.enabled || !game.native_on(m::ID) {
        return;
    }
    if !game.native_playing() && game.screen.id() != "chat" {
        return;
    }
    ensure_icons(game);
    sync(game);
    let s = snapshot(game);
    let size = (batch.width, batch.height);
    m::remember_screen(size);
    let mut painter = GamePainter {
        game: &*game,
        batch,
    };
    m::draw_modules(&mut painter, &s, &cfg, false);
}

/// Uploads the AppleSkin outlines into the mod image slot once per atlas: a
/// patch costs a texture upload, so it is not sent every frame.
pub(super) fn ensure_icons(game: &mut Game<'_>) {
    use std::sync::atomic::{AtomicU64, Ordering};
    static UPLOADED: AtomicU64 = AtomicU64::new(u64::MAX);
    let revision = game.gui.atlas.revision;
    if UPLOADED.load(Ordering::Relaxed) == revision {
        return;
    }
    if game.native_mod_image(36, 9, m::apple_icons_rgba()) {
        UPLOADED.store(revision, Ordering::Relaxed);
    }
}

/// With chat open Horizon skips the chat and tab list layers, so the HUD
/// anchor there is the HUD hook; every other frame uses the tab list layer,
/// where the HUD belongs above the vanilla bars. Drawing twice is avoided by
/// the screen check.
pub(super) fn hud_fallback(game: &mut Game<'_>, batch: &mut HudBatch) {
    if game.screen.id() != "chat" {
        return;
    }
    draw_hud(game, batch);
}

/// The inventory beside an open container, for the module's "with a
/// container" mode.
pub(super) fn draw_container_inventory(
    game: &Game<'_>,
    batch: &mut HudBatch,
    screen: &NativeScreenView<'_>,
) {
    let cfg = config();
    if !cfg.enabled
        || !game.native_on(m::ID)
        || !cfg.inventory.module.enabled
        || cfg.inventory.when != m::InventoryWhen::ContainerOpen
        || !matches!(screen.id, "inventory" | "creative")
    {
        return;
    }
    let s = cached_snapshot();
    m::remember_screen((batch.width, batch.height));
    let mut painter = GamePainter { game, batch };
    let size = m::inventory_size(&painter, &s, &cfg);
    if size.0 <= 0.0 || size.1 <= 0.0 {
        return;
    }
    let [px, py, pw, ph] = screen.rect;
    let (sw, sh) = painter.screen();
    let mut x = px - size.0 - 6.0;
    if x < 4.0 {
        x = px + pw + 6.0;
    }
    if x + size.0 > sw - 4.0 {
        x = (sw - size.0) / 2.0;
    }
    let y = (py + (ph - size.1) / 2.0).clamp(4.0, (sh - size.1 - 4.0).max(4.0));
    let r = m::Rect::new(x, y, size.0, size.1);
    m::draw_module_at(&mut painter, &s, &cfg, m::ModuleId::Inventory, r);
}

pub(super) fn draw_screen(
    game: &Game<'_>,
    batch: &mut HudBatch,
    screen: &NativeScreenView<'_>,
) {
    if !game.native_on(m::ID) {
        return;
    }
    if game.native_mod_screen() != Some(m::ID) {
        draw_container_inventory(game, batch, screen);
        return;
    }
    sync(game);
    let mode = m::mode();
    if mode == m::ScreenMode::Closed {
        return;
    }
    let cfg = config();
    let s = cached_snapshot();
    m::remember_screen((batch.width, batch.height));
    let editor = m::editor();
    let mut painter = GamePainter { game, batch };
    match mode {
        m::ScreenMode::Layout => match editor.page {
            m::EditorPage::Mods => {
                let (sw, sh) = painter.screen();
                painter.fill(m::Rect::new(0.0, 0.0, sw, sh), 0x9000_0000);
                m::draw_modules(&mut painter, &s, &cfg, true);
                m::draw_editor_tabs(&mut painter, &cfg, m::EditorPage::Mods, screen.mouse);
            }
            m::EditorPage::Vanilla => {
                m::draw_vanilla_editor(&mut painter, &cfg, editor.scroll, screen.mouse);
                m::draw_editor_tabs(&mut painter, &cfg, m::EditorPage::Vanilla, screen.mouse);
            }
        },
        m::ScreenMode::Settings => {
            m::draw_settings(&mut painter, &cfg, editor.scroll, screen.mouse)
        }
        m::ScreenMode::Closed => {}
    }
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) -> bool {
    if !pressed || !game.native_playing() || !game.native_on(m::ID) {
        return false;
    }
    let cfg = config();
    let editor = cfg.editor_hotkey.key();
    if !editor.is_empty() && key == editor {
        m::edit_editor(|e| {
            e.page = m::EditorPage::Mods;
            e.scroll = 0;
        });
        return open(game, m::ScreenMode::Layout);
    }
    let settings = cfg.settings_hotkey.key();
    if !settings.is_empty() && key == settings {
        m::edit_editor(|e| {
            e.scroll = 0;
        });
        return open(game, m::ScreenMode::Settings);
    }
    if key == "key.keyboard.v" && cfg.vanilla.enabled {
        let opened = open(game, m::ScreenMode::Layout);
        if opened {
            m::edit_editor(|e| {
                e.page = m::EditorPage::Vanilla;
                e.scroll = 0;
                e.selected_vanilla = e.selected_vanilla.or(Some(m::VanillaId::Hotbar));
            });
        }
        return opened;
    }
    false
}

/// True when the press belongs to the mod's screen: the helper cancels it.
pub(super) fn screen_press(
    game: &Game<'_>,
    screen: &NativeScreenView<'_>,
    button: i32,
) -> bool {
    if !game.native_on(m::ID) || game.native_mod_screen() != Some(m::ID) {
        return false;
    }
    match m::mode() {
        m::ScreenMode::Layout => {
            let hit = m::editor_press(screen.mouse, button);
            if hit {
                commit();
            }
            hit
        }
        m::ScreenMode::Settings => {
            if button != 0 && button != 1 {
                return false;
            }
            let scroll = m::editor().scroll;
            let cfg = config();
            let hit = m::settings_click(
                (screen.rect[2], screen.rect[3]),
                scroll,
                &cfg,
                screen.mouse,
                button,
            );
            if hit {
                commit();
            }
            hit
        }
        m::ScreenMode::Closed => false,
    }
}

pub(super) fn screen_drag(game: &Game<'_>, screen: &NativeScreenView<'_>) -> bool {
    if game.native_mod_screen() != Some(m::ID) || m::mode() != m::ScreenMode::Layout {
        return false;
    }
    m::editor_drag(screen.mouse)
}

pub(super) fn screen_release(game: &Game<'_>) -> bool {
    if game.native_mod_screen() != Some(m::ID) || m::mode() != m::ScreenMode::Layout {
        return false;
    }
    m::editor_release();
    commit();
    true
}

pub(super) fn screen_scroll(
    game: &Game<'_>,
    screen: &NativeScreenView<'_>,
    scroll: [f64; 2],
) -> bool {
    if !game.native_on(m::ID) || game.native_mod_screen() != Some(m::ID) {
        return false;
    }
    let steps = scroll[1] as f32;
    match m::mode() {
        m::ScreenMode::Layout => {
            let changed = m::editor_scroll(screen.mouse, steps);
            if changed {
                commit();
            }
            changed
        }
        m::ScreenMode::Settings => {
            let cfg = config();
            let scroll = m::settings_scroll(
                (screen.rect[2], screen.rect[3]),
                m::editor().scroll,
                steps,
                &cfg,
            );
            m::edit_editor(|e| e.scroll = scroll);
            true
        }
        m::ScreenMode::Closed => false,
    }
}

pub(super) fn screen_key(
    game: &Game<'_>,
    screen: &NativeScreenView<'_>,
    key: &str,
    repeat: bool,
) -> bool {
    if repeat || !game.native_on(m::ID) || game.native_mod_screen() != Some(m::ID) {
        return false;
    }
    let mode = m::mode();
    if key == "key.keyboard.tab" {
        let page = m::editor().page;
        match mode {
            m::ScreenMode::Layout => {
                if page == m::EditorPage::Mods {
                    m::edit_editor(|e| {
                        e.page = m::EditorPage::Vanilla;
                        e.scroll = 0;
                        e.selected_vanilla = e.selected_vanilla.or(Some(m::VanillaId::Hotbar));
                    });
                } else {
                    m::edit_editor(|e| e.scroll = 0);
                    m::set_mode(m::ScreenMode::Settings);
                }
            }
            m::ScreenMode::Settings => {
                m::edit_editor(|e| {
                    e.page = m::EditorPage::Mods;
                    e.scroll = 0;
                });
                m::set_mode(m::ScreenMode::Layout);
            }
            m::ScreenMode::Closed => return false,
        }
        return true;
    }
    match mode {
        m::ScreenMode::Layout => {
            let hit = m::editor_key(key);
            if hit {
                commit();
            }
            hit
        }
        m::ScreenMode::Settings => match key {
            "key.keyboard.page.up" | "key.keyboard.up" => {
                let cfg = config();
                let scroll = m::settings_scroll(
                    (screen.rect[2], screen.rect[3]),
                    m::editor().scroll,
                    4.0,
                    &cfg,
                );
                m::edit_editor(|e| e.scroll = scroll);
                true
            }
            "key.keyboard.page.down" | "key.keyboard.down" => {
                let cfg = config();
                let scroll = m::settings_scroll(
                    (screen.rect[2], screen.rect[3]),
                    m::editor().scroll,
                    -4.0,
                    &cfg,
                );
                m::edit_editor(|e| e.scroll = scroll);
                true
            }
            _ => false,
        },
        m::ScreenMode::Closed => false,
    }
}

// ---------------------------------------------------------------------------
// Tooltips
// ---------------------------------------------------------------------------

thread_local! {
    static SNEAK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub(super) fn remember_sneak(game: &mut Game<'_>) {
    let down = game
        .native_mapping("key.sneak")
        .is_some_and(|(down, _)| down);
    SNEAK.with(|s| s.set(down));
}

pub(super) fn tooltip(
    game: &Game<'_>,
    stack: &horizon_client::items::ItemStack,
    lines: &mut Vec<(Text, u32)>,
) {
    let cfg = config();
    if !cfg.enabled || !game.native_on(m::ID) {
        return;
    }
    let info = game.res.items().get(&stack.item);
    if cfg.tooltips.food {
        if let Some((food, saturation)) = info.and_then(|i| i.food) {
            // Horizon's table already holds the saturation eating gives
            // (vanilla's modifier times the nutrition times two).
            lines.push((
                Text::legacy(&format!(
                    "§7Food: §f{food} §8(§a+{saturation:.1} saturation§8)"
                )),
                u32::MAX,
            ));
        }
    }
    if cfg.tooltips.sneak_details && SNEAK.with(std::cell::Cell::get) {
        if let Some(info) = info {
            if info.max_damage > 0 {
                let left = info.max_damage.saturating_sub(stack.damage);
                lines.push((
                    Text::legacy(&format!(
                        "§7Durability: §f{left} §8/ §f{}",
                        info.max_damage
                    )),
                    u32::MAX,
                ));
            }
            if info.max_stack > 1 {
                lines.push((
                    Text::legacy(&format!("§7Max stack: §f{}", info.max_stack)),
                    u32::MAX,
                ));
            }
        }
        lines.push((
            Text::legacy(&format!("§8{}", stack.item)),
            u32::MAX,
        ));
    }
}

// ---------------------------------------------------------------------------
// Tick, layers and commands
// ---------------------------------------------------------------------------

pub(super) fn tick(game: &mut Game<'_>) {
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    sync(game);
    remember_sneak(game);
    apply_pending(game);
    if game.native_player().is_some() {
        let _ = snapshot(game);
    }
}

thread_local! {
    /// Where each layer's vertices began, so its transform touches only them.
    static LAYER_STARTS: std::cell::RefCell<Vec<(String, usize)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

pub(super) fn layer_pre(game: &Game<'_>, layer: &str, batch: &mut HudBatch, canceled: &mut bool) {
    LAYER_STARTS.with(|s| {
        let mut s = s.borrow_mut();
        // A frame that ended early (a panic caught upstream, a torn-down
        // window) must not leave stale entries behind for the next one.
        if s.len() > 32 {
            s.clear();
        }
        s.push((layer.to_owned(), batch.vertices.len()));
    });
    if layer != "minecraft:effects" {
        return;
    }
    let cfg = config();
    if cfg.enabled
        && cfg.effects.module.enabled
        && cfg.effects.hide_vanilla
        && game.native_on(m::ID)
    {
        *canceled = true;
    }
}

/// Moves and scales the vertices a vanilla layer just drew.
fn apply_vanilla_layer(game: &Game<'_>, layer: &str, batch: &mut HudBatch, start: usize) {
    if !game.native_on(m::ID) {
        return;
    }
    let cfg = config();
    let Some((from, to, scale)) =
        m::vanilla_layer_rects(&cfg, layer, (batch.width, batch.height))
    else {
        return;
    };
    if (scale - 1.0).abs() < 0.001 && to.x == from.x && to.y == from.y {
        return;
    }
    for v in &mut batch.vertices[start..] {
        v.pos[0] = to.x + (v.pos[0] - from.x) * scale;
        v.pos[1] = to.y + (v.pos[1] - from.y) * scale;
    }
}

pub(super) fn layer_post(game: &mut Game<'_>, layer: &str, batch: &mut HudBatch) {
    let start = LAYER_STARTS.with(|s| {
        let mut s = s.borrow_mut();
        match s.last() {
            Some((name, start)) if name == layer => {
                let start = *start;
                s.pop();
                start
            }
            _ => batch.vertices.len(),
        }
    });
    apply_vanilla_layer(game, layer, batch, start);
    if layer == "minecraft:tab_list" {
        draw_hud(game, batch);
    }
}

pub(super) fn options_buttons(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let cfg = config();
    buttons.push(format!(
        "HUD: {}",
        if cfg.enabled { "on" } else { "off" }
    ));
    buttons.push(format!("Theme: {}", cfg.theme.label()));
    buttons.push(format!("Profile: {}", cfg.profile.label()));
    buttons.push("Reset layout".into());
}

pub(super) fn options_click(mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    let mut cfg = config();
    match button {
        0 => cfg.enabled = !cfg.enabled,
        1 => cfg.theme = cfg.theme.next(),
        2 => {
            let next = cfg.profile.next();
            cfg.apply_profile(next);
        }
        3 => cfg.reset_layout(),
        _ => return,
    }
    m::set_config(cfg);
    save(&cfg);
}

pub(super) fn command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
    let mut words = command.split_whitespace();
    let name = words.next().unwrap_or("");
    if name != "hudplus" && name != "hud+" {
        return;
    }
    *ran = true;
    let sub = words.next().unwrap_or("");
    match sub {
        "" | "help" => {
            game.native_chat("HUD+ — H: layout editor, V: vanilla HUD, N: settings");
            game.native_chat("/hudplus on|off — turn the HUD on or off");
            game.native_chat("/hudplus layout|vanilla|settings — open a screen");
            game.native_chat("/hudplus profile <default|minimal|pvp|builder|streamer>");
            game.native_chat("/hudplus theme <dark|light|transparent|neon|high_contrast|amber>");
            game.native_chat("/hudplus reset — restore the default layout");
        }
        "on" | "off" => {
            let on = sub == "on";
            let mut cfg = config();
            cfg.enabled = on;
            m::set_config(cfg);
            save(&cfg);
            game.native_chat(if on { "HUD+ on" } else { "HUD+ off" });
        }
        "layout" => {
            m::edit_editor(|e| {
                e.page = m::EditorPage::Mods;
                e.scroll = 0;
            });
            if open(game, m::ScreenMode::Layout) {
                game.native_chat("HUD+ layout editor");
            } else {
                game.native_chat("HUD+ is not available now");
            }
        }
        "vanilla" => {
            m::edit_editor(|e| {
                e.page = m::EditorPage::Vanilla;
                e.scroll = 0;
                e.selected_vanilla = e.selected_vanilla.or(Some(m::VanillaId::Hotbar));
            });
            if open(game, m::ScreenMode::Layout) {
                game.native_chat("Vanilla HUD editor — drag, wheel scales, Tab: settings");
            } else {
                game.native_chat("HUD+ is not available now");
            }
        }
        "settings" => {
            m::edit_editor(|e| {
                e.scroll = 0;
            });
            if open(game, m::ScreenMode::Settings) {
                game.native_chat("HUD+ settings");
            } else {
                game.native_chat("HUD+ is not available now");
            }
        }
        "reset" => {
            let _ = m::edit_config(|c| {
                c.reset_layout();
                true
            });
            commit();
            game.native_chat("HUD+ layout reset");
        }
        "profile" => {
            let key = words.next().unwrap_or("");
            let found = m::Profile::ALL
                .iter()
                .copied()
                .find(|p| p.key() == key);
            match found {
                Some(profile) => {
                    let mut cfg = config();
                    cfg.apply_profile(profile);
                    m::set_config(cfg);
                    save(&cfg);
                    game.native_chat(&format!("HUD+ profile: {}", profile.label()));
                }
                None => game.native_chat(
                    "Profiles: default, minimal, pvp, builder, streamer",
                ),
            }
        }
        "theme" => {
            let key = words.next().unwrap_or("");
            let found = m::Theme::ALL
                .iter()
                .copied()
                .find(|t| t.key() == key);
            match found {
                Some(theme) => {
                    let _ = m::edit_config(|c| {
                        c.theme = theme;
                        true
                    });
                    commit();
                    game.native_chat(&format!("HUD+ theme: {}", theme.label()));
                }
                None => game.native_chat(
                    "Themes: dark, light, transparent, neon, high_contrast, amber",
                ),
            }
        }
        other => game.native_chat(&format!("HUD+ unknown subcommand: {other}")),
    }
}
