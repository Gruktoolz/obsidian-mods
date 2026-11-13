// Feather-style visual in-game HUD editor over the playing world.
use super::Game;
use crate::game::basic_hud as m;
use crate::game::basic_hud_render as render;
use crate::game::basic_hud_i18n as i18n;
use horizon_client::hud::HudBatch;
use std::cell::{Cell, RefCell};
use winit::keyboard::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum HexField {
    Text,
    Accent,
    Background,
}

struct ThemePreset {
    name: &'static str,
    text: u32,
    accent: u32,
    background: u32,
    opacity: u8,
}

const THEMES: [ThemePreset; 5] = [
    ThemePreset {
        name: "Feather",
        text: m::WidgetConfig::DEFAULT_TEXT,
        accent: 0x70_C9_FF,
        background: 0x10_15_1D,
        opacity: 60,
    },
    ThemePreset {
        name: "Neon",
        text: 0x00_FF_CC,
        accent: 0xFF_00_55,
        background: 0x0A_0E_17,
        opacity: 80,
    },
    ThemePreset {
        name: "Minimal",
        text: 0xEE_EE_EE,
        accent: 0xDD_DD_DD,
        background: 0x00_00_00,
        opacity: 25,
    },
    ThemePreset {
        name: "Gold",
        text: 0xFF_EE_99,
        accent: 0xFF_B3_00,
        background: 0x18_14_08,
        opacity: 70,
    },
    ThemePreset {
        name: "Light",
        text: 0x11_11_11,
        accent: 0x00_66_CC,
        background: 0xF2_F2_F2,
        opacity: 85,
    },
];

pub(super) struct Editor {
    pub(super) draft: m::Config,
    baseline: m::Config,
    exit_prompt: bool,
    pub(super) selected: Option<m::Widget>,
    pub(super) dragging_widget: Option<m::Widget>,
    pub(super) drag_offset: [f32; 2],
    pub(super) hitboxes: Vec<(m::Widget, m::Rect)>,
    pub(super) viewport: [f32; 2],
    pub(super) error: Option<String>,
    pub(super) drawer_open: bool,
    pub(super) drawer_scroll: f32,
    pub(super) theme_index: usize,
    pub(super) hex_input: Option<(HexField, String)>,
    pub(super) return_to_game: bool,
    pub(super) chrome_hidden: bool,
    pub(super) tab_subpanel: bool,
    pub(super) tutorial_step: Option<usize>,
    pub(super) pending_reset: Option<m::ResetTarget>,
}

impl Editor {
    pub(super) fn new(initial: m::Config, viewport: [f32; 2], return_to_game: bool) -> Self {
        Self {
            baseline: initial.clone(),
            exit_prompt: false,
            draft: initial,
            selected: None,
            dragging_widget: None,
            drag_offset: [0.0, 0.0],
            hitboxes: Vec::new(),
            viewport,
            error: None,
            drawer_open: viewport[0] > 360.0,
            drawer_scroll: 0.0,
            theme_index: 0,
            hex_input: None,
            return_to_game,
            chrome_hidden: false,
            tab_subpanel: false,
            tutorial_step: None,
            pending_reset: None,
        }
    }
}

thread_local! {
    static EDITOR: RefCell<Option<Editor>> = const { RefCell::new(None) };
    static GUIDE_OPENED: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn active() -> bool {
    EDITOR.with(|e| e.borrow().is_some())
}

pub(super) fn guide_pending(game: &Game<'_>) -> bool {
    !GUIDE_OPENED.with(Cell::get) && m::should_show_guide(
        game.native_setting(m::ID, m::GUIDE_SEEN_KEY).as_deref(),
        m::config(|| game.native_setting(m::ID, "config"), |c| c.tutorial_seen),
    )
}

pub(super) fn open(game: &mut Game<'_>) {
    let show_guide = guide_pending(game);
    let initial = m::config(|| game.native_setting(m::ID, "config"), m::Config::clone);
    let marker_ok = if game.native_setting(m::ID, m::GUIDE_SEEN_KEY).as_deref() != Some("1") {
        m::mark_guide_seen(
            || game.native_setting(m::ID, m::GUIDE_SEEN_KEY),
            |value| game.native_set_setting(m::ID, m::GUIDE_SEEN_KEY, value),
        )
    } else { true };
    GUIDE_OPENED.with(|seen| seen.set(true));
    let size = game.gui_size();
    let return_to_game = game.options_to_game;
    game.options_to_game = true;
    game.capture(false);
    EDITOR.with(|e| {
        let mut editor = Editor::new(initial, [size.0, size.1], return_to_game);
        editor.tutorial_step = show_guide.then_some(0);
        if !marker_ok {
            editor.error = Some(i18n::tr(editor.draft.language(&game.settings.language), "Error: failed to save settings to disk!").to_owned());
        }
        *e.borrow_mut() = Some(editor);
    });
}

fn request_close(game: &mut Game<'_>) {
    game.dragging = false;
    let dirty = EDITOR.with(|e| {
        let mut editor = e.borrow_mut();
        let Some(ed) = editor.as_mut() else { return false; };
        if !m::should_confirm_exit(&ed.draft, &ed.baseline) { return false; }
        ed.exit_prompt = true;
        ed.pending_reset = None;
        ed.tutorial_step = None;
        ed.hex_input = None;
        ed.dragging_widget = None;
        ed.chrome_hidden = false;
        true
    });
    if !dirty { close(game); }
}

fn save_and_close(game: &mut Game<'_>) {
    let draft = EDITOR.with(|e| e.borrow().as_ref().map(|ed| ed.draft.clone()));
    let Some(draft) = draft else { return; };
    let ok = m::commit_verified(
        &draft,
        || game.native_setting(m::ID, "config"),
        |line| game.native_set_setting(m::ID, "config", line),
    );
    if ok {
        close(game);
    } else {
        EDITOR.with(|e| {
            if let Some(ed) = e.borrow_mut().as_mut() {
                ed.error = Some(i18n::tr(ed.draft.language(&game.settings.language), "Error: failed to save settings to disk!").to_owned());
            }
        });
    }
}

pub(super) fn close(game: &mut Game<'_>) {
    game.dragging = false;
    let return_to_game = EDITOR.with(|e| {
        e.borrow_mut().take().map(|ed| ed.return_to_game).unwrap_or(false)
    });
    if return_to_game && game.screen == super::Screen::Options && game.player.is_some() && game.title.is_none() {
        game.screen = super::Screen::Playing;
        game.keys.clear();
        game.capture(true);
        game.options_to_game = false;
    } else {
        game.options_to_game = false;
        if game.screen == super::Screen::Options {
            if let horizon_client::options_ui::Page::NativeMod(i) = game.ui.options.page() {
                game.native_settings_refresh(i);
            }
        }
    }
}

fn draw_btn(
    game: &Game<'_>,
    batch: &mut HudBatch,
    rect: [f32; 4],
    text: &str,
    bg: u32,
    text_color: u32,
) {
    let [x, y, w, h] = rect;
    game.native_fill(batch, rect, bg);
    let border_color = 0x40FF_FFFF;
    game.native_fill(batch, [x, y, w, 1.0], border_color);
    game.native_fill(batch, [x, y + h - 1.0, w, 1.0], border_color);
    game.native_fill(batch, [x, y, 1.0, h], border_color);
    game.native_fill(batch, [x + w - 1.0, y, 1.0, h], border_color);

    let tw = game.native_text_width(text);
    let max_tw = (w - 4.0).max(1.0);
    let scale = if tw > max_tw { max_tw / tw } else { 1.0 };
    let start = batch.vertices.len();
    let tx = x + (w - tw * scale) * 0.5;
    let ty = y + (h - 8.0 * scale) * 0.5;
    game.native_text(batch, text, [tx, ty], text_color & 0x00FF_FFFF, true);
    if scale < 1.0 {
        batch.scale_since(start, tx, ty, scale, scale);
    }
}

fn in_rect(point: [f32; 2], rect: [f32; 4]) -> bool {
    let [px, py] = point;
    let [x, y, w, h] = rect;
    px >= x && px <= x + w && py >= y && py <= y + h
}

fn help_line(game: &Game<'_>, batch: &mut HudBatch, text: &str, at: [f32; 2], width: f32, color: u32) {
    let scale = (width / game.native_text_width(text).max(1.0)).min(1.0);
    let start = batch.vertices.len();
    game.native_text(batch, text, at, color, true);
    batch.scale_since(start, at[0], at[1], scale, scale);
}

fn content_height(selected: Option<m::Widget>, tab_subpanel: bool) -> f32 {
    if tab_subpanel {
        160.0
    } else {
        match selected {
            Some(m::Widget::Fps) => 260.0,
            Some(m::Widget::ArmorMini) => 246.0,
            Some(m::Widget::Effects) => 228.0,
            Some(m::Widget::Pickups) => 246.0,
            Some(w) if w.supports_icons_only() => 212.0,
            Some(_) => 196.0,
            None => m::WIDGETS.len() as f32 * 15.0 + 105.0,
        }
    }
}

// Drawing and input must use the same bounds, including the error banner.
fn drawer_bounds(viewport: [f32; 2], error: bool) -> [f32; 4] {
    let w = 136.0f32.min(viewport[0] * 0.45);
    let y = if error { 40.0 } else { 24.0 };
    [viewport[0] - w - 3.0, y, w, (viewport[1] - y - 3.0).max(36.0)]
}

fn tab_bounds(viewport: [f32; 2]) -> [f32; 4] {
    [viewport[0] - 31.0, 5.0, 28.0, 14.0]
}

fn hint_bounds(game: &Game<'_>, viewport: [f32; 2], hitboxes: &[(m::Widget, m::Rect)], language: &str) -> [f32; 4] {
    let text = i18n::tr(&language, "[Tab] Show controls");
    let w = (game.native_text_width(text) + 8.0).min(viewport[0]);
    let h = 13.0f32.min(viewport[1]);
    let bottom = (viewport[1] - h - 4.0).max(0.0);
    let right = (viewport[0] - w - 4.0).max(0.0);
    let center = ((viewport[0] - w) * 0.5).max(0.0);
    let candidates = [[center, bottom, w, h], [center, 4.0, w, h], [4.0, bottom, w, h], [right, bottom, w, h]];
    let overlap = |r: &[f32; 4]| hitboxes.iter().map(|(_, b)| {
        ((r[0] + r[2]).min(b.x + b.w) - r[0].max(b.x)).max(0.0)
            * ((r[1] + r[3]).min(b.y + b.h) - r[1].max(b.y)).max(0.0)
    }).sum::<f32>();
    candidates.into_iter().min_by(|a, b| overlap(a).total_cmp(&overlap(b))).unwrap_or([0.0, 0.0, w, h])
}

pub(super) fn draw(game: &mut Game<'_>, batch: &mut HudBatch) -> bool {
    let (draft_clone, selected, error, drawer_open, drawer_scroll, theme_idx, hex_info, chrome_hidden, tab_subpanel, tutorial_step, pending_reset, exit_prompt) = {
        let opt = EDITOR.with(|e| e.borrow().as_ref().map(|ed| (
            ed.draft.clone(),
            ed.selected,
            ed.error.clone(),
            ed.drawer_open,
            ed.drawer_scroll,
            ed.theme_index,
            ed.hex_input.clone(),
            ed.chrome_hidden,
            ed.tab_subpanel,
            ed.tutorial_step,
            ed.pending_reset,
            ed.exit_prompt,
        )));
        match opt {
            Some(vals) => vals,
            None => return false,
        }
    };

    let language = draft_clone.language(&game.settings.language).to_owned();
    let viewport = [batch.width, batch.height];
    EDITOR.with(|e| {
        if let Some(ed) = e.borrow_mut().as_mut() {
            ed.viewport = viewport;
        }
    });

    // 1. Draw widgets preview over the world
    let hitboxes = render::draw(game, batch, &draft_clone, true);
    EDITOR.with(|e| {
        if let Some(ed) = e.borrow_mut().as_mut() {
            ed.hitboxes = hitboxes.clone();
        }
    });

    // Preview Tab if tab subpanel is active
    if tab_subpanel {
        crate::game::basic_hud_tab::draw(game, batch, &draft_clone, true);
    }

    // 2. Highlight selected widget bounding box and badge
    if let Some(sel) = selected {
        if let Some((_, r)) = hitboxes.iter().find(|(w, _)| *w == sel) {
            let sel_color = 0xFF33_AAFF;
            game.native_fill(batch, [r.x - 1.0, r.y - 1.0, r.w + 2.0, 1.5], sel_color);
            game.native_fill(batch, [r.x - 1.0, r.y + r.h - 0.5, r.w + 2.0, 1.5], sel_color);
            game.native_fill(batch, [r.x - 1.0, r.y - 1.0, 1.5, r.h + 2.0], sel_color);
            game.native_fill(batch, [r.x + r.w - 0.5, r.y - 1.0, 1.5, r.h + 2.0], sel_color);

            let scale = draft_clone.widget(sel).scale;
            let info = format!("{} ({:.1}x)", i18n::tr(&language, sel.title(false)), scale);
            let info_w = game.native_text_width(&info);
            let badge_y = (r.y - 11.0).max(0.0);
            game.native_fill(batch, [r.x, badge_y, info_w + 4.0, 10.0], 0xEE11_1620);
            game.native_text(batch, &info, [r.x + 2.0, badge_y + 1.0], 0x00FF_FFFF, true);
        }
    }

    // If chrome is hidden, show discreet hint and return early
    if chrome_hidden {
        let hint_text = i18n::tr(&language, "[Tab] Show controls");
        let r = hint_bounds(game, viewport, &hitboxes, &language);
        game.native_fill(batch, r, 0xCC10_1622);
        game.native_text(batch, hint_text, [r[0] + 4.0, r[1] + 2.0], 0x70C9FF, true);
        return true;
    }

    // 3. Top Toolbar: Feather-style compact dark bar
    let bar_y = 3.0;
    let bar_h = 18.0;
    let bar_w = (viewport[0] - 6.0).max(100.0);
    game.native_fill(batch, [3.0, bar_y, bar_w, bar_h], 0xEE10_1622);
    game.native_fill(batch, [3.0, bar_y + bar_h - 1.0, bar_w, 1.0], 0xFF22_2C3D);

    // Toolbar buttons: Save, Cancel, Reset, Snap, Theme, Panel, Help, Tab hint
    let btn_h = 14.0;
    let btn_y = bar_y + 2.0;

    // Save
    let save_text = i18n::tr(&language, "Save");
    draw_btn(game, batch, [5.0, btn_y, 44.0, btn_h], save_text, 0xFF2B_78D4, 0xFF_FFFF);

    // Cancel
    let cancel_text = i18n::tr(&language, "Cancel");
    draw_btn(game, batch, [52.0, btn_y, 38.0, btn_h], cancel_text, 0xCC2B_3444, 0xDD_DDDD);

    // Reset
    let reset_text = if tab_subpanel {
        i18n::tr(&language, "Reset Tab")
    } else {
        match selected {
            Some(_) => i18n::tr(&language, "Reset"),
            None => i18n::tr(&language, "Reset All"),
        }
    };
    draw_btn(game, batch, [93.0, btn_y, 44.0, btn_h], reset_text, 0xCC2B_3444, 0xDD_DDDD);

    // Snap
    let snap_bg = if draft_clone.snap { 0xFF1B_60A5 } else { 0xCC2B_3444 };
    let snap_text = i18n::tr(&language, if draft_clone.snap { "Snap:ON" } else { "Snap:OFF" });
    draw_btn(game, batch, [140.0, btn_y, 44.0, btn_h], snap_text, snap_bg, 0xFF_FFFF);

    // Theme
    let theme_obj = &THEMES[theme_idx % THEMES.len()];
    let theme_label = i18n::tr(&language, theme_obj.name);
    let theme_btn_text = format!("{}:{}", i18n::tr(&language, "Theme"), theme_label);
    draw_btn(game, batch, [187.0, btn_y, 48.0, btn_h], &theme_btn_text, 0xCC2B_3444, 0xDD_DDDD);

    // Panel
    let panel_bg = if drawer_open { 0xFF1B_60A5 } else { 0xCC2B_3444 };
    let panel_text = i18n::tr(&language, if drawer_open { "Panel:ON" } else { "Panel:OFF" });
    draw_btn(game, batch, [238.0, btn_y, 46.0, btn_h], panel_text, panel_bg, 0xFF_FFFF);

    // Help / Tutorial replay
    if viewport[0] >= 340.0 {
        draw_btn(game, batch, [287.0, btn_y, 22.0, btn_h], "?", 0xCC2B_3444, 0xDD_DDDD);
    }

    // Tab hint
    if viewport[0] >= 320.0 {
        draw_btn(game, batch, tab_bounds(viewport), "Tab", 0x8820_2A38, 0xAA_AAAA);
    }

    // Error banner if any
    let top_offset = 23.0;
    if let Some(ref err) = error {
        game.native_fill(batch, [3.0, top_offset, bar_w, 14.0], 0xEE99_1111);
        game.native_text(batch, &err, [8.0, top_offset + 3.0], 0xFF_FFFF, true);
    }

    // 4. Collapsible Drawer (Properties / Widget List / Tab Settings)
    if drawer_open {
        let [drawer_x, drawer_y, drawer_w, drawer_h] = drawer_bounds(viewport, error.is_some());

        game.native_fill(batch, [drawer_x, drawer_y, drawer_w, drawer_h], 0xF012_1722);
        game.native_fill(batch, [drawer_x, drawer_y, drawer_w, 1.0], 0xFF26_3244);
        game.native_fill(batch, [drawer_x, drawer_y, 1.0, drawer_h], 0xFF26_3244);

        // Drawer Header
        let header_y = drawer_y + 2.0;
        if selected.is_some() || tab_subpanel {
            draw_btn(game, batch, [drawer_x + 3.0, header_y, 16.0, 12.0], "<", 0xCC2B_3444, 0xFF_FFFF);
        }
        let title_text = if tab_subpanel {
            i18n::tr(&language, "Tab Settings")
        } else {
            match selected {
                Some(w) => i18n::tr(&language, w.title(false)),
                None => i18n::tr(&language, "HUD Widgets"),
            }
        };
        let title_x = if selected.is_some() || tab_subpanel { drawer_x + 22.0 } else { drawer_x + 6.0 };
        game.native_text(batch, title_text, [title_x, header_y + 2.0], 0x70C9FF, true);
        draw_btn(game, batch, [drawer_x + drawer_w - 15.0, header_y, 12.0, 12.0], "x", 0xCC44_2222, 0xFF_FFFF);

        // Content Area
        let content_y = drawer_y + 16.0;
        let content_h = (drawer_h - 34.0).max(1.0);
        let total_h = content_height(selected, tab_subpanel);
        let max_scroll = (total_h - content_h + 4.0).max(0.0);
        let cur_scroll = drawer_scroll.clamp(0.0, max_scroll);

        if max_scroll > 0.0 {
            let footer_y = drawer_y + drawer_h - 15.0;
            draw_btn(game, batch, [drawer_x + drawer_w - 34.0, footer_y, 13.0, 12.0], "^", 0xCC2B_3444, 0xFF_FFFF);
            draw_btn(game, batch, [drawer_x + drawer_w - 18.0, footer_y, 13.0, 12.0], "v", 0xCC2B_3444, 0xFF_FFFF);
        }

        let can_draw = |item_y: f32, item_h: f32| -> bool {
            item_y >= content_y - 1.0 && item_y + item_h <= content_y + content_h + 1.0
        };

        if tab_subpanel {
            // Tab Settings subpanel view
            let mut y = content_y + 2.0 - cur_scroll;

            // Enabled toggle
            if can_draw(y, 14.0) {
                let en_text = i18n::tr(&language, "Enabled:");
                help_line(game, batch, en_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let toggle_bg = if draft_clone.tab.enabled { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let toggle_text = if draft_clone.tab.enabled { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], toggle_text, toggle_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Numeric ping
            if can_draw(y, 14.0) {
                let np_text = i18n::tr(&language, "Numeric ping:");
                help_line(game, batch, np_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let np_bg = if draft_clone.tab.numeric_ping { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let np_text_val = if draft_clone.tab.numeric_ping { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], np_text_val, np_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Ping bars
            if can_draw(y, 14.0) {
                let pb_text = i18n::tr(&language, "Ping bars:");
                help_line(game, batch, pb_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let pb_bg = if draft_clone.tab.ping_bars { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let pb_text_val = if draft_clone.tab.ping_bars { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], pb_text_val, pb_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Highlight self
            if can_draw(y, 14.0) {
                let hs_text = i18n::tr(&language, "Highlight self:");
                help_line(game, batch, hs_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let hs_bg = if draft_clone.tab.highlight_self { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let hs_text_val = if draft_clone.tab.highlight_self { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], hs_text_val, hs_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Show count
            if can_draw(y, 14.0) {
                let sc_text = i18n::tr(&language, "Player count:");
                help_line(game, batch, sc_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                let sc_bg = if draft_clone.tab.show_count { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let sc_text_val = if draft_clone.tab.show_count { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], sc_text_val, sc_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Sorting
            if can_draw(y, 14.0) {
                let st_text = i18n::tr(&language, "Sort:");
                help_line(game, batch, st_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 62.0).max(1.0), 0xDD_DDDD);
                let sort_label = match draft_clone.tab.sort {
                    1 => i18n::tr(&language, "Name"),
                    2 => i18n::tr(&language, "Ping"),
                    _ => i18n::tr(&language, "Vanilla"),
                };
                draw_btn(game, batch, [drawer_x + drawer_w - 52.0, y, 48.0, 13.0], sort_label, 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 16.0;

            // Scale +/-
            if can_draw(y, 14.0) {
                let sc_text = i18n::tr(&language, "Scale:");
                help_line(game, batch, sc_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                let val = format!("{:.1}x", draft_clone.tab.scale);
                let tw = game.native_text_width(&val);
                game.native_text(batch, &val, [drawer_x + drawer_w - 42.0 + (26.0 - tw) * 0.5, y + 3.0], 0xFF_FFFF, true);
                draw_btn(game, batch, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 16.0;

            // Opacity +/-
            if can_draw(y, 14.0) {
                let op_text = i18n::tr(&language, "Opacity:");
                help_line(game, batch, op_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                let val = format!("{}%", draft_clone.tab.opacity);
                let tw = game.native_text_width(&val);
                game.native_text(batch, &val, [drawer_x + drawer_w - 42.0 + (26.0 - tw) * 0.5, y + 3.0], 0xFF_FFFF, true);
                draw_btn(game, batch, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 18.0;

            // Reset Tab
            if can_draw(y, 14.0) {
                let rst_text = i18n::tr(&language, "Reset Tab");
                draw_btn(game, batch, [drawer_x + 10.0, y, drawer_w - 20.0, 13.0], rst_text, 0xCC33_2222, 0xFF_FFFF);
            }
        } else if let Some(sel) = selected {
            let wcfg = draft_clone.widget(sel);
            let mut y = content_y + 2.0 - cur_scroll;

            // Enabled toggle
            if can_draw(y, 14.0) {
                let en_text = i18n::tr(&language, "Enabled:");
                help_line(game, batch, en_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let toggle_bg = if draft_clone.enabled(sel) { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                let toggle_text = if draft_clone.enabled(sel) { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], toggle_text, toggle_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Scale +/-
            if can_draw(y, 14.0) {
                let sc_text = i18n::tr(&language, "Scale:");
                help_line(game, batch, sc_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                let val = format!("{:.1}x", wcfg.scale);
                let tw = game.native_text_width(&val);
                game.native_text(batch, &val, [drawer_x + drawer_w - 42.0 + (26.0 - tw) * 0.5, y + 3.0], 0xFF_FFFF, true);
                draw_btn(game, batch, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 16.0;

            // Opacity +/-
            if can_draw(y, 14.0) {
                let op_text = i18n::tr(&language, "Opacity:");
                help_line(game, batch, op_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                let val = format!("{}%", wcfg.opacity);
                let tw = game.native_text_width(&val);
                game.native_text(batch, &val, [drawer_x + drawer_w - 42.0 + (26.0 - tw) * 0.5, y + 3.0], 0xFF_FFFF, true);
                draw_btn(game, batch, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 16.0;

            // Padding +/-
            if can_draw(y, 14.0) {
                let pd_text = i18n::tr(&language, "Padding:");
                help_line(game, batch, pd_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 66.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                let val = format!("{:.0}px", wcfg.padding);
                let tw = game.native_text_width(&val);
                game.native_text(batch, &val, [drawer_x + drawer_w - 42.0 + (26.0 - tw) * 0.5, y + 3.0], 0xFF_FFFF, true);
                draw_btn(game, batch, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 16.0;

            // Toggles: Shadow, Compact, Icons
            if can_draw(y, 14.0) {
                let sh_bg = if wcfg.shadow { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                draw_btn(game, batch, [drawer_x + 4.0, y, 38.0, 13.0], i18n::tr(&language, "Shadow"), sh_bg, 0xFF_FFFF);
                let cp_bg = if wcfg.compact { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                draw_btn(game, batch, [drawer_x + 45.0, y, 42.0, 13.0], i18n::tr(&language, "Compact"), cp_bg, 0xFF_FFFF);
                let ic_bg = if wcfg.icons { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                draw_btn(game, batch, [drawer_x + 90.0, y, 38.0, 13.0], i18n::tr(&language, "Icons"), ic_bg, 0xFF_FFFF);
            }
            y += 18.0;

            if sel.supports_icons_only() {
                if can_draw(y, 14.0) {
                    let bg = if wcfg.icons_only { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                    draw_btn(game, batch, [drawer_x + 4.0, y, drawer_w - 8.0, 13.0], i18n::tr(&language, "Icons + values"), bg, 0xFF_FFFF);
                }
                y += 16.0;
            }

            // Colors: Text
            if can_draw(y, 22.0) {
                let tx_label = i18n::tr(&language, "Text Color:");
                help_line(game, batch, tx_label, [drawer_x + 5.0, y + 1.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let swatches = [0xFF_FFFF, 0xFF_AA00, 0x55_FF55, 0x55_FFFF, 0xFF_5555];
                for (si, &sc) in swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    game.native_fill(batch, [sx, y + 10.0, 14.0, 10.0], 0xFF00_0000 | sc);
                }
                draw_btn(game, batch, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0], "#HEX", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 24.0;

            // Colors: Accent
            if can_draw(y, 22.0) {
                let ac_label = i18n::tr(&language, "Accent Color:");
                help_line(game, batch, ac_label, [drawer_x + 5.0, y + 1.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let swatches = [0x70_C9FF, 0x33_88FF, 0xFF_D700, 0x00_FFAA, 0xBA_68C8];
                for (si, &sc) in swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    game.native_fill(batch, [sx, y + 10.0, 14.0, 10.0], 0xFF00_0000 | sc);
                }
                draw_btn(game, batch, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0], "#HEX", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 24.0;

            // Colors: Background
            if can_draw(y, 22.0) {
                let bg_label = i18n::tr(&language, "BG Color:");
                help_line(game, batch, bg_label, [drawer_x + 5.0, y + 1.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let swatches = [0x10_151D, 0x00_0000, 0x1E_222A, 0x1A_1424, 0x24_1414];
                for (si, &sc) in swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    game.native_fill(batch, [sx, y + 10.0, 14.0, 10.0], 0xFF00_0000 | sc);
                }
                draw_btn(game, batch, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0], "#HEX", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 24.0;

            // Nudge buttons
            if can_draw(y, 14.0) {
                let nd_label = i18n::tr(&language, "Nudge:");
                help_line(game, batch, nd_label, [drawer_x + 5.0, y + 3.0], (drawer_w - 91.0).max(1.0), 0xDD_DDDD);
                draw_btn(game, batch, [drawer_x + 50.0, y, 16.0, 13.0], "<", 0xCC2B_3444, 0xFF_FFFF);
                draw_btn(game, batch, [drawer_x + 69.0, y, 16.0, 13.0], ">", 0xCC2B_3444, 0xFF_FFFF);
                draw_btn(game, batch, [drawer_x + 88.0, y, 16.0, 13.0], "^", 0xCC2B_3444, 0xFF_FFFF);
                draw_btn(game, batch, [drawer_x + 107.0, y, 16.0, 13.0], "v", 0xCC2B_3444, 0xFF_FFFF);
            }
            y += 18.0;

            // Reset this widget button
            if can_draw(y, 14.0) {
                let rst_text = i18n::tr(&language, "Reset Widget");
                draw_btn(game, batch, [drawer_x + 10.0, y, drawer_w - 20.0, 13.0], rst_text, 0xCC33_2222, 0xFF_FFFF);
            }
            y += 16.0;

            // If Effects, option for all effects
            if sel == m::Widget::Effects && can_draw(y, 14.0) {
                let all_text = i18n::tr(&language, "All effects:");
                help_line(game, batch, all_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let all_bg = if draft_clone.all_effects { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                let all_btn_text = if draft_clone.all_effects { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], all_btn_text, all_bg, 0xFF_FFFF);
            }

            if sel == m::Widget::Pickups && can_draw(y, 14.0) {
                let label = i18n::tr(&language, "Display time:");
                help_line(game, batch, label, [drawer_x + 5.0, y + 3.0], (drawer_w - 68.0).max(1.0), 0xDD_DDDD);
                let value = i18n::fmt(&language, "{0} s", &[&draft_clone.pickup_seconds.to_string()]);
                draw_btn(game, batch, [drawer_x + drawer_w - 62.0, y, 14.0, 13.0], "-", 0xCC2B_3444, 0xFF_FFFF);
                help_line(game, batch, &value, [drawer_x + drawer_w - 46.0, y + 3.0], 27.0, 0xFF_FFFF);
                draw_btn(game, batch, [drawer_x + drawer_w - 18.0, y, 14.0, 13.0], "+", 0xCC2B_3444, 0xFF_FFFF);
            }

            // If FPS: fps_label, fps_lows, fps_auto, plus scale note
            if sel == m::Widget::Fps {
                if can_draw(y, 14.0) {
                    let lbl_text = i18n::tr(&language, "FPS label:");
                    help_line(game, batch, lbl_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                    let lbl_bg = if draft_clone.fps_label { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let lbl_btn = if draft_clone.fps_label { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], lbl_btn, lbl_bg, 0xFF_FFFF);
                }
                y += 16.0;

                if can_draw(y, 14.0) {
                    let lows_text = i18n::tr(&language, "1% lows:");
                    help_line(game, batch, lows_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                    let lows_bg = if draft_clone.fps_lows { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let lows_btn = if draft_clone.fps_lows { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], lows_btn, lows_bg, 0xFF_FFFF);
                }
                y += 16.0;

                if can_draw(y, 14.0) {
                    let auto_text = i18n::tr(&language, "Auto-hide:");
                    help_line(game, batch, auto_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                    let auto_bg = if draft_clone.fps_auto { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let auto_btn = if draft_clone.fps_auto { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], auto_btn, auto_bg, 0xFF_FFFF);
                }
                y += 15.0;

                if can_draw(y, 12.0) {
                    let note = i18n::tr(&language, "Hides 1% if scale <0.85");
                    game.native_text(batch, note, [drawer_x + 5.0, y + 2.0], 0x88_AAAA, true);
                }
                y += 14.0;
            }

            // If ArmorMini: mini_anchor and mini_numbers
            if sel == m::Widget::ArmorMini {
                if can_draw(y, 14.0) {
                    let anch_text = i18n::tr(&language, "Anchor:");
                    help_line(game, batch, anch_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                    let anch_bg = if draft_clone.mini_anchor { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let anch_btn = if draft_clone.mini_anchor { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], anch_btn, anch_bg, 0xFF_FFFF);
                }
                y += 16.0;

                if can_draw(y, 14.0) {
                    let num_text = i18n::tr(&language, "Dur. numbers:");
                    help_line(game, batch, num_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                    let num_bg = if draft_clone.mini_numbers { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let num_btn = if draft_clone.mini_numbers { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0], num_btn, num_bg, 0xFF_FFFF);
                }
            }

            // Hex input overlay prompt if active
            if let Some((field, ref hex_buf)) = hex_info {
                let prompt_y = content_y + content_h - 22.0;
                game.native_fill(batch, [drawer_x + 2.0, prompt_y, drawer_w - 4.0, 20.0], 0xEE11_1622);
                let f_name = match field {
                    HexField::Text => i18n::tr(&language, "Text"),
                    HexField::Accent => i18n::tr(&language, "Accent"),
                    HexField::Background => i18n::tr(&language, "BG"),
                };
                let p_text = format!("{}: #{}_", f_name, hex_buf);
                game.native_text(batch, &p_text, [drawer_x + 6.0, prompt_y + 2.0], 0x70C9FF, true);
                game.native_text(batch, "Enter / Esc", [drawer_x + 6.0, prompt_y + 11.0], 0xDDDDDD, true);
            }
        } else {
            // Widget List view
            let mut y = content_y + 2.0 - cur_scroll;
            for w in m::WIDGETS {
                if can_draw(y, 13.0) {
                    let w_title = i18n::tr(&language, w.title(false));
                    draw_btn(game, batch, [drawer_x + 4.0, y, drawer_w - 38.0, 13.0], w_title, 0xCC20_2838, 0xFF_FFFF);
                    let toggle_bg = if draft_clone.enabled(w) { 0xFF1E_7E34 } else { 0xFF72_1C24 };
                    let toggle_text = if draft_clone.enabled(w) { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                    draw_btn(game, batch, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0], toggle_text, toggle_bg, 0xFF_FFFF);
                }
                y += 15.0;
            }

            y += 4.0;
            // Global toggles: Health tags, FPS lows
            if can_draw(y, 13.0) {
                let ht_text = i18n::tr(&language, "Health tags:");
                help_line(game, batch, ht_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let ht_bg = if draft_clone.health_tags { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                let ht_btn = if draft_clone.health_tags { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0], ht_btn, ht_bg, 0xFF_FFFF);
            }
            y += 15.0;

            if can_draw(y, 13.0) {
                let fps_text = i18n::tr(&language, "FPS 1% lows:");
                help_line(game, batch, fps_text, [drawer_x + 5.0, y + 3.0], (drawer_w - 42.0).max(1.0), 0xDD_DDDD);
                let fps_bg = if draft_clone.fps_lows { 0xFF1E_7E34 } else { 0xCC2B_3444 };
                let fps_btn = if draft_clone.fps_lows { i18n::tr(&language, "ON") } else { i18n::tr(&language, "OFF") };
                draw_btn(game, batch, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0], fps_btn, fps_bg, 0xFF_FFFF);
            }
            y += 16.0;

            // Tab settings subpanel button
            if can_draw(y, 14.0) {
                let tab_btn_text = i18n::tr(&language, "[Tab] Tab list settings");
                draw_btn(game, batch, [drawer_x + 4.0, y, drawer_w - 8.0, 14.0], tab_btn_text, 0xFF20_4466, 0xFF_FFFF);
            }
            y += 16.0;

            // Tutorial replay button
            if can_draw(y, 14.0) {
                let tut_btn_text = i18n::tr(&language, "? Help & Tutorial");
                draw_btn(game, batch, [drawer_x + 4.0, y, drawer_w - 8.0, 14.0], tut_btn_text, 0xCC24_3344, 0xFF_FFFF);
            }
            y += 16.0;
            if can_draw(y, 10.0) {
                game.native_text(batch, i18n::tr(&language, "Language"), [drawer_x + 5.0, y + 1.0], 0xDD_DDDD, true);
            }
            y += 11.0;
            if can_draw(y, 13.0) {
                draw_btn(game, batch, [drawer_x + 4.0, y, 14.0, 13.0], "<", 0xCC2B_3444, 0xFF_FFFF);
                let label = if draft_clone.language == "auto" { i18n::tr(&language, "Auto") } else { i18n::language_name(&draft_clone.language) };
                draw_btn(game, batch, [drawer_x + 21.0, y, drawer_w - 42.0, 13.0], label, 0xCC2B_3444, 0xFF_FFFF);
                draw_btn(game, batch, [drawer_x + drawer_w - 18.0, y, 14.0, 13.0], ">", 0xCC2B_3444, 0xFF_FFFF);
            }
        }
    }

    if exit_prompt {
        let [mx, my, mw, mh] = exit_bounds(viewport);
        game.native_fill(batch, [0.0, 0.0, viewport[0], viewport[1]], 0x9900_0000);
        game.native_fill(batch, [mx, my, mw, mh], 0xFF12_1722);
        help_line(game, batch, i18n::tr(&language, "Unsaved changes"), [mx + 10.0, my + 10.0], mw - 20.0, 0xFF_C840);
        help_line(game, batch, i18n::tr(&language, "Save changes before leaving?"), [mx + 10.0, my + 29.0], mw - 20.0, 0xFF_FFFF);
        if let Some(error) = &error {
            help_line(game, batch, error, [mx + 10.0, my + 46.0], mw - 20.0, 0xFF_7070);
        }
        for (rect, (key, color)) in exit_buttons(viewport).into_iter().zip([
            ("Save", 0xFF2B_78D4), ("Discard changes", 0xCC72_1C24), ("Stay", 0xCC2B_3444),
        ]) {
            draw_btn(game, batch, rect, i18n::tr(&language, key), color, 0xFF_FFFF);
        }
        let hint = format!("Enter: {} / Esc: {}", i18n::tr(&language, "Save"), i18n::tr(&language, "Stay"));
        help_line(game, batch, &hint, [mx + 10.0, my + mh - 15.0], mw - 20.0, 0xAA_AAAA);
        return true;
    }

    if let Some(target) = pending_reset {
        let [mx, my, mw, mh] = reset_bounds(viewport);
        game.native_fill(batch, [0.0, 0.0, viewport[0], viewport[1]], 0x9900_0000);
        game.native_fill(batch, [mx, my, mw, mh], 0xFF12_1722);
        help_line(game, batch, i18n::tr(&language, "Are you sure you want to reset?"), [mx + 10.0, my + 10.0], mw - 20.0, 0xFF_C840);
        let scope = match target {
            m::ResetTarget::All => i18n::tr(&language, "Reset all HUD settings").to_owned(),
            m::ResetTarget::Tab => i18n::tr(&language, "Reset Tab settings").to_owned(),
            m::ResetTarget::Widget(w) => format!("{}: {}", i18n::tr(&language, "Reset this widget"), i18n::tr(&language, w.title(false))),
        };
        help_line(game, batch, &scope, [mx + 10.0, my + 29.0], mw - 20.0, 0xFF_FFFF);
        let bw = (mw - 30.0) * 0.5;
        draw_btn(game, batch, [mx + 10.0, my + mh - 39.0, bw, 15.0], i18n::tr(&language, "Cancel"), 0xFF2B_78D4, 0xFF_FFFF);
        draw_btn(game, batch, [mx + 20.0 + bw, my + mh - 39.0, bw, 15.0], i18n::tr(&language, "Yes, reset"), 0xCC72_1C24, 0xFF_FFFF);
        help_line(game, batch, i18n::tr(&language, "Enter / Esc: Cancel"), [mx + 10.0, my + mh - 15.0], mw - 20.0, 0xAA_AAAA);
        return true;
    }

    // First settings visit, or an explicit replay through the help button.
    if let Some(step) = tutorial_step {
        let mw = (viewport[0] * 0.65).clamp(240.0, 360.0);
        let mh = 110.0;
        let mx = (viewport[0] - mw) * 0.5;
        let my = (viewport[1] - mh) * 0.5;

        // Scrim over world and controls
        game.native_fill(batch, [0.0, 0.0, viewport[0], viewport[1]], 0x7700_0000);

        // Modal background and border
        game.native_fill(batch, [mx, my, mw, mh], 0xF010_1622);
        let border = 0xFF2B_78D4;
        game.native_fill(batch, [mx, my, mw, 1.5], border);
        game.native_fill(batch, [mx, my + mh - 1.5, mw, 1.5], border);
        game.native_fill(batch, [mx, my, 1.5, mh], border);
        game.native_fill(batch, [mx + mw - 1.5, my, 1.5, mh], border);

        let (title, line1, line2) = match step {
            0 => (
                i18n::tr(&language, "1/5 Move Widgets"),
                i18n::tr(&language, "Click and drag any widget across the screen."),
                i18n::tr(&language, "Toggle 'Snap' in top bar to align."),
            ),
            1 => (
                i18n::tr(&language, "2/5 Scale & Style"),
                i18n::tr(&language, "Scroll mouse wheel over a widget to scale it."),
                i18n::tr(&language, "In the side panel, tune colors, shadow and BG."),
            ),
            2 => (
                i18n::tr(&language, "3/5 FPS Options"),
                i18n::tr(&language, "In FPS options: toggle 'FPS' label and 1% lows."),
                i18n::tr(&language, "Auto-hide hides details when scale is < 0.85x."),
            ),
            3 => (
                i18n::tr(&language, "4/5 ArmorMini & Tab"),
                i18n::tr(&language, "ArmorMini shows equipment durability by hotbar."),
                i18n::tr(&language, "In Tab settings, customize ping, bars, and scale."),
            ),
            _ => (
                i18n::tr(&language, "5/5 Saving Changes"),
                i18n::tr(&language, "Click 'Save' to persist your settings to disk."),
                i18n::tr(&language, "Leaving with edits asks whether to save them."),
            ),
        };

        game.native_text(batch, title, [mx + 10.0, my + 8.0], 0x70C9FF, true);
        help_line(game, batch, line1, [mx + 10.0, my + 26.0], mw - 20.0, 0xFF_FFFF);
        help_line(game, batch, line2, [mx + 10.0, my + 42.0], mw - 20.0, 0xCC_CCCC);

        let btn_y = my + mh - 22.0;
        let btn_h = 14.0;
        if step < 4 {
            let skip_txt = i18n::tr(&language, "Skip");
            draw_btn(game, batch, [mx + 10.0, btn_y, 64.0, btn_h], skip_txt, 0xCC2B_3444, 0xDD_DDDD);
            let next_txt = i18n::tr(&language, "Next >");
            draw_btn(game, batch, [mx + mw - 70.0, btn_y, 60.0, btn_h], next_txt, 0xFF2B_78D4, 0xFF_FFFF);
        } else {
            let done_txt = i18n::tr(&language, "Done");
            draw_btn(game, batch, [mx + mw - 70.0, btn_y, 60.0, btn_h], done_txt, 0xFF1E_7E34, 0xFF_FFFF);
        }
    }

    true
}

fn reset_bounds(viewport: [f32; 2]) -> [f32; 4] {
    let width = (viewport[0] * 0.7).clamp(230.0, 360.0);
    let height = 100.0;
    [(viewport[0] - width) * 0.5, (viewport[1] - height) * 0.5, width, height]
}

fn exit_bounds(viewport: [f32; 2]) -> [f32; 4] {
    let width = (viewport[0] * 0.8).clamp(260.0, 420.0);
    let height = 110.0;
    [(viewport[0] - width) * 0.5, (viewport[1] - height) * 0.5, width, height]
}

fn exit_buttons(viewport: [f32; 2]) -> [[f32; 4]; 3] {
    let [x, y, w, h] = exit_bounds(viewport);
    let bw = (w - 40.0) / 3.0;
    std::array::from_fn(|i| [x + 10.0 + i as f32 * (bw + 10.0), y + h - 39.0, bw, 15.0])
}

pub(super) fn mouse(game: &mut Game<'_>, pressed: bool, right: bool) -> bool {
    let exit = EDITOR.with(|e| e.borrow().as_ref().filter(|ed| ed.exit_prompt).map(|ed| ed.viewport));
    if let Some(viewport) = exit {
        game.dragging = false;
        if pressed && !right {
            let cursor = game.native_cursor();
            let buttons = exit_buttons(viewport);
            if in_rect(cursor, buttons[0]) { save_and_close(game); }
            else if in_rect(cursor, buttons[1]) { close(game); }
            else if in_rect(cursor, buttons[2]) {
                EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() { ed.exit_prompt = false; } });
            }
        }
        return true;
    }
    let request = EDITOR.with(|e| e.borrow().as_ref().and_then(|ed| ed.pending_reset.map(|target| (target, ed.viewport))));
    if let Some((target, viewport)) = request {
        game.dragging = false;
        if pressed && !right {
            let cursor = game.native_cursor();
            let [mx, my, mw, mh] = reset_bounds(viewport);
            let bw = (mw - 30.0) * 0.5;
            let cancel = in_rect(cursor, [mx + 10.0, my + mh - 39.0, bw, 15.0]);
            let confirm = in_rect(cursor, [mx + 20.0 + bw, my + mh - 39.0, bw, 15.0]);
            if cancel || confirm {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.pending_reset = None;
                        ed.dragging_widget = None;
                        if confirm { ed.draft.reset_target(target); }
                    }
                });
            }
        }
        return true;
    }
    let language = EDITOR.with(|e| e.borrow().as_ref().map(|ed| ed.draft.language(&game.settings.language).to_owned())).unwrap_or_else(|| game.settings.language.clone());
    let cursor = game.native_cursor();
    let viewport = EDITOR.with(|e| e.borrow().as_ref().map_or([640.0, 360.0], |ed| ed.viewport));

    // Full modal consumption for the tutorial
    let tut_step = EDITOR.with(|e| e.borrow().as_ref().and_then(|ed| ed.tutorial_step));
    if let Some(step) = tut_step {
        if !pressed || right {
            return true;
        }
        let mw = (viewport[0] * 0.65).clamp(240.0, 360.0);
        let mh = 110.0;
        let mx = (viewport[0] - mw) * 0.5;
        let my = (viewport[1] - mh) * 0.5;
        let btn_y = my + mh - 22.0;
        let btn_h = 14.0;

        if step < 4 {
            // Skip button
            if in_rect(cursor, [mx + 10.0, btn_y, 64.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.draft.tutorial_seen = true;
                        ed.tutorial_step = None;
                    }
                });
                return true;
            }
            // Next button
            if in_rect(cursor, [mx + mw - 70.0, btn_y, 60.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.tutorial_step = Some(step + 1);
                    }
                });
                return true;
            }
        } else {
            // Done button
            if in_rect(cursor, [mx + mw - 70.0, btn_y, 60.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.draft.tutorial_seen = true;
                        ed.tutorial_step = None;
                    }
                });
                return true;
            }
        }
        return true;
    }

    if pressed && EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.hex_input.is_some())) {
        return true;
    }

    // Right mouse button: consume ignore or select only without drag/button activation
    if right {
        let over_controls = EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| {
            !ed.chrome_hidden && (cursor[1] <= 21.0 || (ed.drawer_open && in_rect(cursor, drawer_bounds(ed.viewport, ed.error.is_some()))))
        }));
        if over_controls { return true; }
        if pressed {
            let hitboxes = EDITOR.with(|e| e.borrow().as_ref().map(|ed| ed.hitboxes.clone())).unwrap_or_default();
            let mut hit = None;
            for (w, r) in hitboxes.iter().rev() {
                if r.contains(cursor) {
                    hit = Some(*w);
                    break;
                }
            }
            EDITOR.with(|e| {
                if let Some(ed) = e.borrow_mut().as_mut() {
                    ed.selected = hit;
                    ed.hex_input = None;
                    ed.tab_subpanel = false;
                    ed.dragging_widget = None;
                }
            });
            game.dragging = false;
        }
        return true;
    }

    if !pressed {
        game.dragging = false;
        EDITOR.with(|e| {
            if let Some(ed) = e.borrow_mut().as_mut() {
                ed.dragging_widget = None;
            }
        });
        return true;
    }

    let chrome_hidden = EDITOR.with(|e| e.borrow().as_ref().map(|ed| ed.chrome_hidden).unwrap_or(false));

    // A color prompt owns input until Enter or Escape; never click through it.
    if EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.hex_input.is_some())) {
        return true;
    }
    let restore = EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| {
        ed.chrome_hidden && in_rect(cursor, hint_bounds(game, ed.viewport, &ed.hitboxes, &language))
    }));
    if restore {
        EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() { ed.chrome_hidden = false; } });
        return true;
    }

    if !chrome_hidden {
        let viewport = EDITOR.with(|e| e.borrow().as_ref().map_or([640.0, 360.0], |ed| ed.viewport));
        if viewport[0] >= 320.0 && in_rect(cursor, tab_bounds(viewport)) {
            EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() { ed.chrome_hidden = true; } });
            return true;
        }
        // 1. Toolbar clicks
        let bar_y = 3.0;
        let btn_y = bar_y + 2.0;
        let btn_h = 14.0;
        if cursor[1] >= btn_y && cursor[1] <= btn_y + btn_h {
            // Save: [5.0, btn_y, 44.0, btn_h]
            if in_rect(cursor, [5.0, btn_y, 44.0, btn_h]) {
                save_and_close(game);
                return true;
            }

            // Cancel: [52.0, btn_y, 38.0, btn_h]
            if in_rect(cursor, [52.0, btn_y, 38.0, btn_h]) {
                request_close(game);
                return true;
            }

            // Reset: [93.0, btn_y, 44.0, btn_h]
            if in_rect(cursor, [93.0, btn_y, 44.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        if ed.tab_subpanel {
                            ed.pending_reset = Some(m::ResetTarget::Tab);
                        } else if let Some(sel) = ed.selected {
                            ed.pending_reset = Some(m::ResetTarget::Widget(sel));
                        } else {
                            ed.pending_reset = Some(m::ResetTarget::All);
                            ed.selected = None;
                            ed.tab_subpanel = false;
                            ed.dragging_widget = None;
                            ed.hex_input = None;
                            game.dragging = false;
                        }
                    }
                });
                return true;
            }

            // Snap: [140.0, btn_y, 44.0, btn_h]
            if in_rect(cursor, [140.0, btn_y, 44.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.draft.snap = !ed.draft.snap;
                    }
                });
                return true;
            }

            // Theme: [187.0, btn_y, 48.0, btn_h]
            if in_rect(cursor, [187.0, btn_y, 48.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.theme_index = (ed.theme_index + 1) % THEMES.len();
                        let t = &THEMES[ed.theme_index];
                        if let Some(sel) = ed.selected {
                            let w = ed.draft.widget_mut(sel);
                            w.text = t.text;
                            w.accent = t.accent;
                            w.background = t.background;
                            w.opacity = t.opacity;
                        } else {
                            for w in m::WIDGETS {
                                let cfg = ed.draft.widget_mut(w);
                                cfg.text = t.text;
                                cfg.accent = t.accent;
                                cfg.background = t.background;
                                cfg.opacity = t.opacity;
                            }
                        }
                    }
                });
                return true;
            }

            // Panel: [238.0, btn_y, 46.0, btn_h]
            if in_rect(cursor, [238.0, btn_y, 46.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.drawer_open = !ed.drawer_open;
                    }
                });
                return true;
            }

            // Help: [287.0, btn_y, 22.0, btn_h]
            if viewport[0] >= 340.0 && in_rect(cursor, [287.0, btn_y, 22.0, btn_h]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.tutorial_step = Some(0);
                    }
                });
                return true;
            }
        }

        // The rest of the toolbar is also chrome, never a widget drag target.
        if cursor[1] <= 21.0 { return true; }

        // 2. Drawer clicks
        let (drawer_open, drawer_x, drawer_y, drawer_w, drawer_h, drawer_scroll, selected, tab_subpanel) = {
            let opt = EDITOR.with(|e| e.borrow().as_ref().map(|ed| {
                let [dx, dy, dw, dh] = drawer_bounds(ed.viewport, ed.error.is_some());
                (ed.drawer_open, dx, dy, dw, dh, ed.drawer_scroll, ed.selected, ed.tab_subpanel)
            }));
            match opt {
                Some(v) => v,
                None => (false, 0.0, 0.0, 0.0, 0.0, 0.0, None, false),
            }
        };

        if drawer_open && in_rect(cursor, [drawer_x, drawer_y, drawer_w, drawer_h]) {
            let header_y = drawer_y + 2.0;

            // Close [X] button
            if in_rect(cursor, [drawer_x + drawer_w - 15.0, header_y, 12.0, 12.0]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.drawer_open = false;
                    }
                });
                return true;
            }

            // Back [<] button if selected or tab_subpanel
            if (selected.is_some() || tab_subpanel) && in_rect(cursor, [drawer_x + 3.0, header_y, 16.0, 12.0]) {
                EDITOR.with(|e| {
                    if let Some(ed) = e.borrow_mut().as_mut() {
                        ed.selected = None;
                        ed.hex_input = None;
                        ed.tab_subpanel = false;
                    }
                });
                return true;
            }

            let content_y = drawer_y + 16.0;
            let content_h = (drawer_h - 34.0).max(1.0);
            let total_h = content_height(selected, tab_subpanel);
            let max_scroll = (total_h - content_h + 4.0).max(0.0);
            let cur_scroll = drawer_scroll.clamp(0.0, max_scroll);

            let footer_y = drawer_y + drawer_h - 15.0;
            let up = [drawer_x + drawer_w - 34.0, footer_y, 13.0, 12.0];
            let down = [drawer_x + drawer_w - 18.0, footer_y, 13.0, 12.0];
            if max_scroll > 0.0 && (in_rect(cursor, up) || in_rect(cursor, down)) {
                let step = if in_rect(cursor, up) { -32.0 } else { 32.0 };
                EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() {
                    ed.drawer_scroll = (cur_scroll + step).clamp(0.0, max_scroll);
                } });
                return true;
            }

            let can_hit = |item_y: f32, item_h: f32, item_rect: [f32; 4]| -> bool {
                item_y >= content_y - 1.0 && item_y + item_h <= content_y + content_h + 1.0 && in_rect(cursor, item_rect)
            };

            if tab_subpanel {
                // Tab Settings clicks
                let mut y = content_y + 2.0 - cur_scroll;

                // Enabled toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.enabled = !ed.draft.tab.enabled;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Numeric ping toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.numeric_ping = !ed.draft.tab.numeric_ping;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Ping bars toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.ping_bars = !ed.draft.tab.ping_bars;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Highlight self toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.highlight_self = !ed.draft.tab.highlight_self;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Player count toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.show_count = !ed.draft.tab.show_count;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Sorting cycle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 52.0, y, 48.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.sort = (ed.draft.tab.sort + 1) % 3;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Scale -
                if can_hit(y, 13.0, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.scale = (ed.draft.tab.scale - 0.1).clamp(0.5, 1.5);
                        }
                    });
                    return true;
                }
                // Scale +
                if can_hit(y, 13.0, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.scale = (ed.draft.tab.scale + 0.1).clamp(0.5, 1.5);
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Opacity -
                if can_hit(y, 13.0, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.opacity = ed.draft.tab.opacity.saturating_sub(5);
                        }
                    });
                    return true;
                }
                // Opacity +
                if can_hit(y, 13.0, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.tab.opacity = (ed.draft.tab.opacity + 5).min(100);
                        }
                    });
                    return true;
                }
                y += 18.0;

                // Reset Tab
                if can_hit(y, 13.0, [drawer_x + 10.0, y, drawer_w - 20.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.pending_reset = Some(m::ResetTarget::Tab);
                        }
                    });
                    return true;
                }

                return true;
            } else if let Some(sel) = selected {
                let mut y = content_y + 2.0 - cur_scroll;

                // Enabled toggle
                if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let cur = ed.draft.enabled(sel);
                            ed.draft.set_enabled(sel, !cur);
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Scale -
                if can_hit(y, 13.0, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.scale = (w.scale - 0.1).clamp(0.5, 3.0);
                        }
                    });
                    return true;
                }
                // Scale +
                if can_hit(y, 13.0, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.scale = (w.scale + 0.1).clamp(0.5, 3.0);
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Opacity -
                if can_hit(y, 13.0, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.opacity = w.opacity.saturating_sub(10);
                        }
                    });
                    return true;
                }
                // Opacity +
                if can_hit(y, 13.0, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.opacity = (w.opacity + 10).min(100);
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Padding -
                if can_hit(y, 13.0, [drawer_x + drawer_w - 56.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.padding = (w.padding - 1.0).clamp(0.0, 12.0);
                        }
                    });
                    return true;
                }
                // Padding +
                if can_hit(y, 13.0, [drawer_x + drawer_w - 16.0, y, 12.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.padding = (w.padding + 1.0).clamp(0.0, 12.0);
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Toggles: Shadow, Compact, Icons
                if can_hit(y, 13.0, [drawer_x + 4.0, y, 38.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.shadow = !w.shadow;
                        }
                    });
                    return true;
                }
                if can_hit(y, 13.0, [drawer_x + 45.0, y, 42.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.compact = !w.compact;
                        }
                    });
                    return true;
                }
                if can_hit(y, 13.0, [drawer_x + 90.0, y, 38.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            let w = ed.draft.widget_mut(sel);
                            w.icons = !w.icons;
                            if !w.icons { w.icons_only = false; }
                        }
                    });
                    return true;
                }
                y += 18.0;

                if sel.supports_icons_only() {
                    if can_hit(y, 13.0, [drawer_x + 4.0, y, drawer_w - 8.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                let style = ed.draft.widget_mut(sel);
                                style.icons_only = !style.icons_only;
                                if style.icons_only { style.icons = true; }
                            }
                        });
                        return true;
                    }
                    y += 16.0;
                }

                // Color swatches: Text
                let text_swatches = [0xFF_FFFF, 0xFF_AA00, 0x55_FF55, 0x55_FFFF, 0xFF_5555];
                for (si, &sc) in text_swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    if can_hit(y, 20.0, [sx, y + 10.0, 14.0, 10.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.widget_mut(sel).text = sc;
                            }
                        });
                        return true;
                    }
                }
                if can_hit(y, 20.0, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.hex_input = Some((HexField::Text, String::new()));
                        }
                    });
                    return true;
                }
                y += 24.0;

                // Color swatches: Accent
                let accent_swatches = [0x70_C9FF, 0x33_88FF, 0xFF_D700, 0x00_FFAA, 0xBA_68C8];
                for (si, &sc) in accent_swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    if can_hit(y, 20.0, [sx, y + 10.0, 14.0, 10.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.widget_mut(sel).accent = sc;
                            }
                        });
                        return true;
                    }
                }
                if can_hit(y, 20.0, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.hex_input = Some((HexField::Accent, String::new()));
                        }
                    });
                    return true;
                }
                y += 24.0;

                // Color swatches: Background
                let bg_swatches = [0x10_151D, 0x00_0000, 0x1E_222A, 0x1A_1424, 0x24_1414];
                for (si, &sc) in bg_swatches.iter().enumerate() {
                    let sx = drawer_x + 5.0 + si as f32 * 17.0;
                    if can_hit(y, 20.0, [sx, y + 10.0, 14.0, 10.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.widget_mut(sel).background = sc;
                            }
                        });
                        return true;
                    }
                }
                if can_hit(y, 20.0, [drawer_x + drawer_w - 38.0, y + 10.0, 32.0, 10.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.hex_input = Some((HexField::Background, String::new()));
                        }
                    });
                    return true;
                }
                y += 24.0;

                // Nudge buttons
                let nudge = |dx: f32, dy: f32| {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            if let Some((_, r)) = ed.hitboxes.iter().find(|(w, _)| *w == sel) {
                                let scale = ed.draft.widget(sel).scale.clamp(0.5, 3.0);
                                let base_w = if r.w > 0.0 { r.w / scale } else { 40.0 };
                                let base_h = if r.h > 0.0 { r.h / scale } else { 20.0 };
                                let at = [r.x + dx, r.y + dy];
                                ed.draft.move_widget(sel, at, [base_w, base_h], ed.viewport, false);
                            }
                        }
                    });
                };
                if can_hit(y, 13.0, [drawer_x + 50.0, y, 16.0, 13.0]) {
                    nudge(-1.0, 0.0);
                    return true;
                }
                if can_hit(y, 13.0, [drawer_x + 69.0, y, 16.0, 13.0]) {
                    nudge(1.0, 0.0);
                    return true;
                }
                if can_hit(y, 13.0, [drawer_x + 88.0, y, 16.0, 13.0]) {
                    nudge(0.0, -1.0);
                    return true;
                }
                if can_hit(y, 13.0, [drawer_x + 107.0, y, 16.0, 13.0]) {
                    nudge(0.0, 1.0);
                    return true;
                }
                y += 18.0;

                // Reset this widget
                if can_hit(y, 13.0, [drawer_x + 10.0, y, drawer_w - 20.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.pending_reset = Some(m::ResetTarget::Widget(sel));
                        }
                    });
                    return true;
                }
                y += 16.0;

                if sel == m::Widget::Effects && can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.all_effects = !ed.draft.all_effects;
                        }
                    });
                    return true;
                }

                if sel == m::Widget::Pickups {
                    let minus = can_hit(y, 13.0, [drawer_x + drawer_w - 62.0, y, 14.0, 13.0]);
                    let plus = can_hit(y, 13.0, [drawer_x + drawer_w - 18.0, y, 14.0, 13.0]);
                    if minus || plus {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.pickup_seconds = if plus { ed.draft.pickup_seconds.saturating_add(1) }
                                    else { ed.draft.pickup_seconds.saturating_sub(1) }.clamp(1, 60);
                            }
                        });
                        return true;
                    }
                }

                // If FPS: toggles for fps_label, fps_lows, fps_auto
                if sel == m::Widget::Fps {
                    if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.fps_label = !ed.draft.fps_label;
                            }
                        });
                        return true;
                    }
                    y += 16.0;

                    if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.fps_lows = !ed.draft.fps_lows;
                            }
                        });
                        return true;
                    }
                    y += 16.0;

                    if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.fps_auto = !ed.draft.fps_auto;
                            }
                        });
                        return true;
                    }
                    y += 15.0;
                    y += 14.0;
                }

                // If ArmorMini: toggles for mini_anchor, mini_numbers
                if sel == m::Widget::ArmorMini {
                    if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.mini_anchor = !ed.draft.mini_anchor;
                            }
                        });
                        return true;
                    }
                    y += 16.0;

                    if can_hit(y, 13.0, [drawer_x + drawer_w - 32.0, y, 28.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.draft.mini_numbers = !ed.draft.mini_numbers;
                            }
                        });
                        return true;
                    }
                }
            } else {
                // Widget List view
                let mut y = content_y + 2.0 - cur_scroll;
                for w in m::WIDGETS {
                    if can_hit(y, 13.0, [drawer_x + 4.0, y, drawer_w - 38.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                ed.selected = Some(w);
                                ed.hex_input = None;
                                ed.tab_subpanel = false;
                            }
                        });
                        return true;
                    }
                    if can_hit(y, 13.0, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0]) {
                        EDITOR.with(|e| {
                            if let Some(ed) = e.borrow_mut().as_mut() {
                                let cur = ed.draft.enabled(w);
                                ed.draft.set_enabled(w, !cur);
                            }
                        });
                        return true;
                    }
                    y += 15.0;
                }

                y += 4.0;
                if can_hit(y, 13.0, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.health_tags = !ed.draft.health_tags;
                        }
                    });
                    return true;
                }
                y += 15.0;

                if can_hit(y, 13.0, [drawer_x + drawer_w - 31.0, y, 26.0, 13.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.fps_lows = !ed.draft.fps_lows;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Tab settings button
                if can_hit(y, 14.0, [drawer_x + 4.0, y, drawer_w - 8.0, 14.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.tab_subpanel = true;
                            ed.selected = None;
                            ed.hex_input = None;
                            ed.drawer_scroll = 0.0;
                        }
                    });
                    return true;
                }
                y += 16.0;

                // Tutorial replay button
                if can_hit(y, 14.0, [drawer_x + 4.0, y, drawer_w - 8.0, 14.0]) {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.tutorial_step = Some(0);
                        }
                    });
                    return true;
                }
                y += 27.0;
                let direction = if can_hit(y, 13.0, [drawer_x + 4.0, y, 14.0, 13.0]) { -1 }
                    else if can_hit(y, 13.0, [drawer_x + drawer_w - 18.0, y, 14.0, 13.0]) { 1 } else { 0 };
                if direction != 0 {
                    EDITOR.with(|e| {
                        if let Some(ed) = e.borrow_mut().as_mut() {
                            ed.draft.language = i18n::next_language(&ed.draft.language, direction).to_owned();
                        }
                    });
                    return true;
                }
            }

            // Consume click anywhere inside drawer bounds
            return true;
        }
    }

    // 3. Widget hitboxes (in reverse order to select topmost)
    let hitboxes = EDITOR.with(|e| e.borrow().as_ref().map(|ed| ed.hitboxes.clone())).unwrap_or_default();
    for (w, r) in hitboxes.iter().rev() {
        if r.contains(cursor) {
            game.dragging = true;
            EDITOR.with(|e| {
                if let Some(ed) = e.borrow_mut().as_mut() {
                    ed.selected = Some(*w);
                    ed.dragging_widget = Some(*w);
                    ed.drag_offset = [cursor[0] - r.x, cursor[1] - r.y];
                    ed.hex_input = None;
                    ed.tab_subpanel = false;
                }
            });
            return true;
        }
    }

    // 4. Clicked outside in empty space: clear selection
    EDITOR.with(|e| {
        if let Some(ed) = e.borrow_mut().as_mut() {
            ed.selected = None;
            ed.hex_input = None;
        }
    });

    true
}

pub(super) fn drag(game: &mut Game<'_>) -> bool {
    if EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.exit_prompt || ed.tutorial_step.is_some() || ed.pending_reset.is_some())) {
        return true;
    }
    let cursor = game.native_cursor();
    EDITOR.with(|e| {
        if let Some(ed) = e.borrow_mut().as_mut() {
            if let Some(w) = ed.dragging_widget {
                let at = [cursor[0] - ed.drag_offset[0], cursor[1] - ed.drag_offset[1]];
                if let Some((_, r)) = ed.hitboxes.iter().find(|(hw, _)| *hw == w) {
                    let scale = ed.draft.widget(w).scale.clamp(0.5, 3.0);
                    let base_w = if r.w > 0.0 { r.w / scale } else { 40.0 };
                    let base_h = if r.h > 0.0 { r.h / scale } else { 20.0 };
                    ed.draft.move_widget(w, at, [base_w, base_h], ed.viewport, ed.draft.snap);
                }
            }
        }
    });
    true
}

pub(super) fn scroll(game: &mut Game<'_>, amount: f32) -> bool {
    if EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.exit_prompt || ed.tutorial_step.is_some() || ed.hex_input.is_some() || ed.pending_reset.is_some())) {
        return true;
    }
    let cursor = game.native_cursor();
    let handled = EDITOR.with(|e| {
        if let Some(ed) = e.borrow_mut().as_mut() {
            if !ed.chrome_hidden && ed.drawer_open {
                let [dx, dy, dw, dh] = drawer_bounds(ed.viewport, ed.error.is_some());
                let content_h = (dh - 34.0).max(1.0);
                let total_h = content_height(ed.selected, ed.tab_subpanel);
                let max_scroll = (total_h - content_h + 4.0).max(0.0);

                if in_rect(cursor, [dx, dy, dw, dh]) {
                    ed.drawer_scroll = (ed.drawer_scroll - amount * 16.0).clamp(0.0, max_scroll);
                    return true;
                }
            }

            // If wheel over widget or selected widget, scale it
            let target_w = ed.hitboxes.iter().rev().find(|(_, r)| r.contains(cursor)).map(|(w, _)| *w).or(ed.selected);
            if let Some(w) = target_w {
                let delta = if amount > 0.0 { 0.1 } else { -0.1 };
                let cfg = ed.draft.widget_mut(w);
                cfg.scale = (cfg.scale + delta).clamp(0.5, 3.0);
                return true;
            }

            true
        } else {
            false
        }
    });

    handled
}

pub(super) fn key(game: &mut Game<'_>, key: KeyCode, text: Option<&str>) -> bool {
    if EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.exit_prompt)) {
        match key {
            KeyCode::Enter => save_and_close(game),
            KeyCode::Escape => EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() { ed.exit_prompt = false; } }),
            _ => {}
        }
        return true;
    }
    if EDITOR.with(|e| e.borrow().as_ref().is_some_and(|ed| ed.pending_reset.is_some())) {
        if matches!(key, KeyCode::Escape | KeyCode::Enter) {
            EDITOR.with(|e| { if let Some(ed) = e.borrow_mut().as_mut() { ed.pending_reset = None; } });
        }
        return true;
    }
    // Sane modal consumption for tutorial
    let tut_step = EDITOR.with(|e| e.borrow().as_ref().and_then(|ed| ed.tutorial_step));
    if let Some(step) = tut_step {
        if key == KeyCode::Escape {
            EDITOR.with(|e| {
                if let Some(ed) = e.borrow_mut().as_mut() {
                    ed.draft.tutorial_seen = true;
                    ed.tutorial_step = None;
                }
            });
            return true;
        }
        if key == KeyCode::Enter {
            EDITOR.with(|e| {
                if let Some(ed) = e.borrow_mut().as_mut() {
                    if step < 4 {
                        ed.tutorial_step = Some(step + 1);
                    } else {
                        ed.draft.tutorial_seen = true;
                        ed.tutorial_step = None;
                    }
                }
            });
            return true;
        }
        return true;
    }

    // Escape key
    if key == KeyCode::Escape {
        let has_hex = EDITOR.with(|e| e.borrow().as_ref().map_or(false, |ed| ed.hex_input.is_some()));
        if has_hex {
            EDITOR.with(|e| {
                if let Some(ed) = e.borrow_mut().as_mut() {
                    ed.hex_input = None;
                }
            });
            return true;
        }
        let subpanel_or_sel = EDITOR.with(|e| {
            if let Some(ed) = e.borrow_mut().as_mut() {
                if ed.tab_subpanel {
                    ed.tab_subpanel = false;
                    return true;
                }
                if ed.selected.is_some() {
                    ed.selected = None;
                    return true;
                }
            }
            false
        });
        if subpanel_or_sel {
            return true;
        }
        request_close(game);
        return true;
    }

    // Tab key to toggle chrome visibility (only when not editing hex)
    if key == KeyCode::Tab {
        let toggled = EDITOR.with(|e| {
            if let Some(ed) = e.borrow_mut().as_mut() {
                if ed.hex_input.is_none() && ed.tutorial_step.is_none() {
                    ed.chrome_hidden = !ed.chrome_hidden;
                    return true;
                }
            }
            false
        });
        if toggled {
            return true;
        }
    }

    // Hex input typing
    let hex_handled = EDITOR.with(|e| {
        let mut ed = e.borrow_mut();
        let ed = match ed.as_mut() {
            Some(ed) => ed,
            None => return false,
        };

        if let Some((field, ref mut buf)) = ed.hex_input {
            if key == KeyCode::Enter {
                if let Ok(color) = u32::from_str_radix(buf, 16) {
                    if let Some(sel) = ed.selected {
                        let w = ed.draft.widget_mut(sel);
                        match field {
                            HexField::Text => w.text = color,
                            HexField::Accent => w.accent = color,
                            HexField::Background => w.background = color,
                        }
                    }
                }
                ed.hex_input = None;
                return true;
            }
            if key == KeyCode::Backspace {
                buf.pop();
                return true;
            }
            {
                let physical = match key {
                    KeyCode::KeyA => "A", KeyCode::KeyB => "B", KeyCode::KeyC => "C",
                    KeyCode::KeyD => "D", KeyCode::KeyE => "E", KeyCode::KeyF => "F",
                    _ => "",
                };
                let t = text.filter(|t| t.chars().any(|c| c.is_ascii_hexdigit())).unwrap_or(physical);
                for c in t.chars() {
                    if c.is_ascii_hexdigit() && buf.len() < 6 {
                        buf.push(c.to_ascii_uppercase());
                    }
                }
                if buf.len() == 6 {
                    if let Ok(color) = u32::from_str_radix(buf, 16) {
                        if let Some(sel) = ed.selected {
                            let w = ed.draft.widget_mut(sel);
                            match field {
                                HexField::Text => w.text = color,
                                HexField::Accent => w.accent = color,
                                HexField::Background => w.background = color,
                            }
                        }
                    }
                }
                return true;
            }
        }

        false
    });

    if hex_handled {
        return true;
    }

    // Arrow keys nudge
    let arrow_handled = EDITOR.with(|e| {
        let mut ed = e.borrow_mut();
        let ed = match ed.as_mut() {
            Some(ed) => ed,
            None => return false,
        };

        let sel = match ed.selected {
            Some(s) => s,
            None => return false,
        };

        let step = if ed.draft.snap { 8.0 } else { 1.0 };
        let (dx, dy) = match key {
            KeyCode::ArrowLeft => (-step, 0.0),
            KeyCode::ArrowRight => (step, 0.0),
            KeyCode::ArrowUp => (0.0, -step),
            KeyCode::ArrowDown => (0.0, step),
            _ => return false,
        };

        if let Some((_, r)) = ed.hitboxes.iter().find(|(w, _)| *w == sel) {
            let scale = ed.draft.widget(sel).scale.clamp(0.5, 3.0);
            let base_w = if r.w > 0.0 { r.w / scale } else { 40.0 };
            let base_h = if r.h > 0.0 { r.h / scale } else { 20.0 };
            let at = [r.x + dx, r.y + dy];
            ed.draft.move_widget(sel, at, [base_w, base_h], ed.viewport, false);
            return true;
        }

        false
    });

    if arrow_handled {
        return true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_creation_and_defaults() {
        let mut initial = m::Config::default();
        initial.tutorial_seen = true;
        let ed = Editor::new(initial.clone(), [800.0, 600.0], true);
        assert_eq!(ed.draft, initial);
        assert!(ed.return_to_game);
        assert!(!ed.chrome_hidden);
        assert_eq!(ed.selected, None);
        assert_eq!(ed.drawer_scroll, 0.0);
        assert!(!ed.tab_subpanel);
        assert_eq!(ed.tutorial_step, None);

        // First open triggers tutorial
        let mut first = m::Config::default();
        first.tutorial_seen = false;
        let ed_first = Editor::new(first, [800.0, 600.0], true);
        assert_eq!(ed_first.tutorial_step, None);
    }

    #[test]
    fn content_height_dynamic() {
        assert!(content_height(Some(m::Widget::Effects), false) > content_height(Some(m::Widget::Durability), false));
        assert!(content_height(Some(m::Widget::Fps), false) > content_height(Some(m::Widget::Durability), false));
        assert!(content_height(Some(m::Widget::ArmorMini), false) > content_height(Some(m::Widget::Durability), false));
        assert!(content_height(None, false) > content_height(None, true));
    }

    #[test]
    fn hex_field_parsing() {
        let parsed = u32::from_str_radix("FF55AA", 16);
        assert_eq!(parsed, Ok(0xFF55AA));
        assert!(u32::from_str_radix("INVALID", 16).is_err());
    }

    #[test]
    fn in_rect_helper() {
        let r = [10.0, 20.0, 30.0, 40.0];
        assert!(in_rect([10.0, 20.0], r));
        assert!(in_rect([40.0, 60.0], r));
        assert!(!in_rect([9.0, 20.0], r));
        assert!(!in_rect([41.0, 60.0], r));
    }
}
