// Chat Animation for Horizon: new chat messages slide up into place and fade in, the indicator
// bar at the left of a message can be hidden, and the chat's input field slides up from the
// bottom of the screen when it opens. Its settings are a screen of their own, reached from a
// "Chat Animation Settings..." button on the Options screen.
//
// Written from scratch for Horizon, after what Chat Animation by Ezzenix (MIT) does for Java.
// The chat's own drawing is left as it is: the hooks (`layout.rs`, `input.rs`) hand what Horizon
// would draw here, and it is moved, faded or trimmed.

use crate::hud::{HudAtlas, HudBatch, rgba};
use crate::hud_layout::{GuiDraw, PoseOp};
use crate::screen_draw::Draw;
use crate::text::Text;
use crate::ui;
use std::sync::Mutex;
use std::time::Instant;

pub const ID: &str = "chat_animation";
pub const NAME: &str = "Chat Animation";
pub const VERSION: &str = "0.1.3";
pub const DESCRIPTION: &str = "By tariksabanuz. New chat messages slide up smoothly and fade in, the chat's input \
field slides up when you open it, and the indicator line beside messages can be hidden. Settings: Options > Chat Animation Settings.";

/// The settings, as the config screen shows them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub animate_messages: bool,
    /// Milliseconds a new message takes to slide into place.
    pub message_ms: u32,
    pub hide_indicator: bool,
    pub animate_opacity: bool,
    pub animate_input: bool,
    pub input_ms: u32,
}

pub const DEFAULT: Config = Config {
    animate_messages: true,
    message_ms: 150,
    hide_indicator: true,
    animate_opacity: true,
    animate_input: true,
    input_ms: 170,
};

/// The duration sliders run from 0 to this many milliseconds, in steps of `STEP_MS`.
pub const MAX_MS: u32 = 750;
pub const STEP_MS: u32 = 10;

struct State {
    config: Config,
    loaded: bool,
    active: bool,
    dir: Option<std::path::PathBuf>,
    /// The chat's message count and its newest line's tick when last drawn.
    seen: Option<(u64, u64)>,
    /// When the newest message arrived, and how many lines it has.
    message: Option<(Instant, usize)>,
    /// When the chat's input field opened.
    input: Option<Instant>,
    menu: Menu,
}

#[derive(Default)]
struct Menu {
    open: bool,
    /// The slider being dragged: its row.
    dragging: Option<usize>,
    scroll: f32,
}

static STATE: Mutex<State> = Mutex::new(State {
    config: DEFAULT,
    loaded: false,
    active: true,
    dir: None,
    seen: None,
    message: None,
    input: None,
    menu: Menu { open: false, dragging: None, scroll: 0.0 },
});

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn config() -> Config {
    state().config
}

pub fn loaded() -> bool {
    state().loaded
}

/// The mod is on or off in Options > Mods.
pub fn set_active(on: bool) {
    state().active = on;
}

fn active() -> bool {
    state().active
}

// ---------------------------------------------------------------------------------------------
// Settings file: `config/chat_animation.txt`, `key=value` lines.

pub fn load(dir: Option<std::path::PathBuf>) {
    let mut s = state();
    s.loaded = true;
    if let Some(dir) = dir {
        let text = std::fs::read_to_string(dir.join("chat_animation.txt")).unwrap_or_default();
        s.config = parse(&text);
        s.dir = Some(dir);
    }
}

pub fn parse(text: &str) -> Config {
    let mut c = DEFAULT;
    for line in text.lines() {
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let (k, v) = (k.trim(), v.trim());
        let yes = v != "0" && v != "false";
        let ms = v.parse::<u32>().ok().map(|n| n.min(MAX_MS));
        match k {
            "animate_messages" => c.animate_messages = yes,
            "message_ms" => c.message_ms = ms.unwrap_or(c.message_ms),
            "hide_indicator" => c.hide_indicator = yes,
            "animate_opacity" => c.animate_opacity = yes,
            "animate_input" => c.animate_input = yes,
            "input_ms" => c.input_ms = ms.unwrap_or(c.input_ms),
            _ => {}
        }
    }
    c
}

pub fn to_text(c: &Config) -> String {
    format!(
        "animate_messages={}\nmessage_ms={}\nhide_indicator={}\nanimate_opacity={}\nanimate_input={}\ninput_ms={}\n",
        u8::from(c.animate_messages),
        c.message_ms,
        u8::from(c.hide_indicator),
        u8::from(c.animate_opacity),
        u8::from(c.animate_input),
        c.input_ms
    )
}

fn save(s: &State) {
    if let Some(dir) = &s.dir {
        let _ = std::fs::write(dir.join("chat_animation.txt"), to_text(&s.config));
    }
}

fn set_config(f: impl FnOnce(&mut Config)) {
    let mut s = state();
    f(&mut s.config);
    save(&s);
}

// ---------------------------------------------------------------------------------------------
// The animations.

/// Ease out: fast at first, settling softly into place.
pub fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t)
}

/// How far along an animation that started at `start` and lasts `ms` is (1: done).
fn progress(start: Option<Instant>, ms: u32, now: Instant) -> f32 {
    match start {
        Some(t) if ms > 0 => now.duration_since(t).as_secs_f32() * 1000.0 / ms as f32,
        _ => 1.0,
    }
}

fn faded(color: u32, k: f32) -> u32 {
    let a = ((color >> 24) as f32 * k.clamp(0.0, 1.0)).round() as u32;
    (a.min(255) << 24) | (color & 0x00FF_FFFF)
}

/// The chat as Horizon lays it out (`Chat::layout`), after a look at the chat: `received` its
/// message count, `newest` the tick its newest line was added and `fresh` how many lines were
/// added on that tick, `entry` a line's height, `scrolled` whether the player scrolled up.
pub fn messages(draws: &mut Vec<GuiDraw>, received: u64, newest: u64, fresh: usize, entry: i32, scrolled: bool) {
    let now = Instant::now();
    let (c, start, lines) = {
        let mut s = state();
        let seen = Some((received, newest));
        if s.seen.is_some() && s.seen != seen && fresh > 0 {
            s.message = Some((now, fresh));
        }
        s.seen = seen;
        let (start, lines) = s.message.map_or((None, 0), |(t, n)| (Some(t), n));
        (s.config, start, lines)
    };
    if !active() {
        return;
    }
    if c.hide_indicator {
        // The message tag's bar: two pixels at the left of the background.
        draws.retain(|d| !matches!(d, GuiDraw::Fill { x0: -4, x1: -2, .. }));
    }
    let t = progress(start, c.message_ms, now);
    if t >= 1.0 || lines == 0 || scrolled {
        return;
    }
    let k = ease(t);
    // The newest lines are those at the bottom: from `chat_bottom` up `lines` rows.
    let bottom = draws
        .iter()
        .filter_map(|d| match d {
            GuiDraw::Fill { x0: -4, x1, y1, .. } if *x1 > -2 => Some(*y1),
            _ => None,
        })
        .max();
    let Some(bottom) = bottom else {
        return;
    };
    let top = bottom - lines as i32 * entry;
    if c.animate_opacity {
        for d in draws.iter_mut() {
            match d {
                GuiDraw::Fill { y0, color, .. } if *y0 >= top => *color = faded(*color, k),
                GuiDraw::Text { y, color, .. } if *y >= top => *color = faded(*color, k),
                _ => {}
            }
        }
    }
    if c.animate_messages
        && let Some(GuiDraw::Push(ops)) = draws.first_mut()
    {
        // The whole chat sits lower by the new lines, and rises into place.
        ops.push(PoseOp::Translate(0.0, (1.0 - k) * (lines as i32 * entry) as f32));
    }
}

/// The chat screen opened (`ChatScreen::new`): its input field starts below the screen.
pub fn input_opened() {
    state().input = Some(Instant::now());
}

/// How far below its place the input field is drawn now, in GUI pixels.
pub fn input_offset() -> i32 {
    let (c, start) = {
        let s = state();
        (s.config, s.input)
    };
    if !active() || !c.animate_input {
        return 0;
    }
    let t = progress(start, c.input_ms, Instant::now());
    if t >= 1.0 {
        return 0;
    }
    ((1.0 - ease(t)) * 14.0).round() as i32
}

/// The chat screen's draws (input field, suggestions, usage), moved down by `dy`.
pub fn shift(draws: &mut [Draw], dy: i32) {
    if dy == 0 {
        return;
    }
    for d in draws.iter_mut() {
        match d {
            Draw::Sprite { y, .. } | Draw::Blit { y, .. } | Draw::Text { y, .. } | Draw::Tooltip { y, .. } => *y += dy,
            Draw::Fill { y0, y1, .. } | Draw::Highlight { y0, y1, .. } | Draw::Scissor { y0, y1, .. } => {
                *y0 += dy;
                *y1 += dy;
            }
            Draw::NoScissor => {}
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Options > Mods: the row's buttons.

pub fn setting_labels() -> Vec<String> {
    let c = config();
    let yn = |b: bool| if b { "Yes" } else { "No" };
    vec![
        format!("Animate Messages: {}", yn(c.animate_messages)),
        format!("Message Duration: {}ms", c.message_ms),
        format!("Hide Indicator Line: {}", yn(c.hide_indicator)),
        format!("Animate Opacity: {}", yn(c.animate_opacity)),
        format!("Animate Input Field: {}", yn(c.animate_input)),
        format!("Input Field Duration: {}ms", c.input_ms),
    ]
}

/// A button of the Mods page: a switch flips, a duration goes up 50 ms (back to 50 past the most).
pub fn setting_click(button: usize) {
    let next = |ms: u32| if ms + 50 > MAX_MS { 50 } else { (ms / 50 + 1) * 50 };
    set_config(|c| match button {
        0 => c.animate_messages = !c.animate_messages,
        1 => c.message_ms = next(c.message_ms),
        2 => c.hide_indicator = !c.hide_indicator,
        3 => c.animate_opacity = !c.animate_opacity,
        4 => c.animate_input = !c.animate_input,
        5 => c.input_ms = next(c.input_ms),
        _ => {}
    });
}

// ---------------------------------------------------------------------------------------------
// The Options button.

static BUTTON: Mutex<[f32; 4]> = Mutex::new([-1000.0, -1000.0, 0.0, 0.0]);

pub const BUTTON_LABEL: &str = "Chat Animation Settings...";

static BUTTON_AT: Mutex<Option<Instant>> = Mutex::new(None);

/// Where the Options screen's layout put the button (it lays out each frame it is drawn).
pub fn set_button(r: [f32; 4]) {
    *BUTTON.lock().unwrap_or_else(|e| e.into_inner()) = r;
    *BUTTON_AT.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
}

/// Whether (`x`, `y`) is on the button. Only while the Options screen itself is being drawn:
/// another mod's screen in its place (Physics Settings) hides the button.
pub fn button_at(x: f32, y: f32) -> bool {
    let fresh = BUTTON_AT
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .is_some_and(|t| t.elapsed().as_millis() < 250);
    let r = *BUTTON.lock().unwrap_or_else(|e| e.into_inner());
    fresh && x >= r[0] && y >= r[1] && x < r[0] + r[2] && y < r[1] + r[3]
}

pub fn button_wanted() -> bool {
    active()
}

thread_local! {
    static INSIDE: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

/// The hooks call the hooked function again from inside themselves: true once per `which`
/// (1 chat, 2 input field, 4 options layout), until `leave`.
pub fn enter(which: u8) -> bool {
    INSIDE.with(|c| {
        let v = c.get();
        c.set(v | which);
        v & which == 0
    })
}

pub fn leave(which: u8) {
    INSIDE.with(|c| c.set(c.get() & !which));
}

/// Where the button goes on the Options screen, from the grid's buttons (x, y, w of each
/// 150-wide button below the FOV row) and the other wide buttons already placed: in the free
/// half of the grid's last row if no other button took it (where Credits stands alone), else in
/// the gap between the FOV row and the grid, else a new row under the grid while it stays clear
/// of Done. None: no room.
pub fn place(grid: &[[f32; 3]], others: &[[f32; 4]], w: f32, h: f32) -> Option<[f32; 4]> {
    let cx = (w / 2.0).floor();
    let gy = grid.iter().map(|g| g[1]).fold(f32::INFINITY, f32::min);
    let last = grid.iter().map(|g| g[1]).fold(f32::NEG_INFINITY, f32::max);
    if !gy.is_finite() {
        return None;
    }
    let free = |r: [f32; 4]| {
        !others.iter().any(|o| o[0] < r[0] + r[2] && r[0] < o[0] + o[2] && o[1] < r[1] + r[3] && r[1] < o[1] + o[3])
            && !grid.iter().any(|g| g[0] < r[0] + r[2] && r[0] < g[0] + g[2] && g[1] < r[1] + 20.0 && r[1] < g[1] + 20.0)
    };
    let in_last = grid.iter().filter(|g| (g[1] - last).abs() < 0.5).count();
    let half = [cx + 4.0, last, 150.0, 20.0];
    if in_last == 1 && free(half) {
        return Some(half);
    }
    let gap = [cx - 154.0, gy - 24.0, 308.0, 20.0];
    if gy - 24.0 >= 53.0 && free(gap) {
        return Some(gap);
    }
    // Beside another mod's button in that gap, if it took only half of it.
    let gap_right = [cx + 4.0, gy - 24.0, 150.0, 20.0];
    if gy - 24.0 >= 53.0 && free(gap_right) {
        return Some(gap_right);
    }
    let below = [cx - 154.0, last + 24.0, 308.0, 20.0];
    if last + 24.0 + 20.0 <= h - 31.0 && free(below) {
        return Some(below);
    }
    None
}

// ---------------------------------------------------------------------------------------------
// The config screen.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Header(&'static str),
    Switch(&'static str, usize),
    Duration(&'static str, usize),
}

/// The screen's rows: sections and their settings (the index is the setting's).
pub const ITEMS: [Item; 8] = [
    Item::Header("Messages"),
    Item::Switch("Animate Messages", 0),
    Item::Duration("Duration", 1),
    Item::Switch("Hide Indicator Line", 2),
    Item::Switch("Animate Opacity", 3),
    Item::Header("Input Field"),
    Item::Switch("Animate Input Field", 4),
    Item::Duration("Duration", 5),
];

const TITLE: &str = "Chat Animation Config";
const ROW: f32 = 25.0;
const HEADER: f32 = 24.0;
const FOOTER: f32 = 33.0;

fn switch_of(c: &Config, i: usize) -> bool {
    match i {
        0 => c.animate_messages,
        2 => c.hide_indicator,
        3 => c.animate_opacity,
        _ => c.animate_input,
    }
}

fn ms_of(c: &Config, i: usize) -> u32 {
    if i == 1 { c.message_ms } else { c.input_ms }
}

fn is_default(c: &Config, i: usize) -> bool {
    match i {
        1 | 5 => ms_of(c, i) == ms_of(&DEFAULT, i),
        _ => switch_of(c, i) == switch_of(&DEFAULT, i),
    }
}

fn reset(c: &mut Config, i: usize) {
    match i {
        0 => c.animate_messages = DEFAULT.animate_messages,
        1 => c.message_ms = DEFAULT.message_ms,
        2 => c.hide_indicator = DEFAULT.hide_indicator,
        3 => c.animate_opacity = DEFAULT.animate_opacity,
        4 => c.animate_input = DEFAULT.animate_input,
        _ => c.input_ms = DEFAULT.input_ms,
    }
}

fn flip(c: &mut Config, i: usize) {
    match i {
        0 => c.animate_messages = !c.animate_messages,
        2 => c.hide_indicator = !c.hide_indicator,
        3 => c.animate_opacity = !c.animate_opacity,
        _ => c.animate_input = !c.animate_input,
    }
}

fn set_ms(c: &mut Config, i: usize, ms: u32) {
    if i == 1 {
        c.message_ms = ms;
    } else {
        c.input_ms = ms;
    }
}

/// Where everything is, for a screen `w` x `h` scrolled by `scroll`.
pub struct Layout {
    pub top: f32,
    pub bottom: f32,
    pub content: f32,
    /// Each item's top.
    pub rows: Vec<f32>,
    pub label_x: f32,
    pub value: [f32; 2],
    pub reset: [f32; 2],
    pub done: [f32; 4],
}

pub fn layout(w: f32, h: f32, scroll: f32) -> Layout {
    let cx = (w / 2.0).floor();
    let top = HEADER;
    let bottom = (h - FOOTER).max(top);
    let content = ITEMS.len() as f32 * ROW + 6.0;
    let rows = (0..ITEMS.len()).map(|k| top + 3.0 + k as f32 * ROW - scroll).collect();
    Layout {
        top,
        bottom,
        content,
        rows,
        label_x: cx - 190.0,
        value: [cx + 45.0, 120.0],
        reset: [cx + 170.0, 20.0],
        done: [cx - 100.0, h - 27.0, 200.0, 20.0],
    }
}

fn max_scroll(l: &Layout) -> f32 {
    (l.content - (l.bottom - l.top)).max(0.0)
}

pub fn menu_is_open() -> bool {
    state().menu.open
}

pub fn menu_open() {
    let mut s = state();
    s.menu = Menu { open: true, dragging: None, scroll: 0.0 };
}

pub fn menu_close() {
    state().menu = Menu::default();
}

fn inside(r: [f32; 4], x: f32, y: f32) -> bool {
    x >= r[0] && y >= r[1] && x < r[0] + r[2] && y < r[1] + r[3]
}

/// The reset arrow: a ring open at its top right, with the arrowhead there (my own pixels).
fn reset_icon(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, color: u32) {
    const ICON: [&str; 9] = [
        "...####.#",
        "..#....##",
        ".#....###",
        "#........",
        "#........",
        "#.......#",
        ".#.....#.",
        "..#...#..",
        "...###...",
    ];
    for (j, row) in ICON.iter().enumerate() {
        for (i, ch) in row.chars().enumerate() {
            if ch == '#' {
                batch.fill(atlas, x + i as f32, y + j as f32, 1.0, 1.0, color);
            }
        }
    }
}

/// Draws the config screen in place of the Options screen.
pub fn menu_draw(batch: &mut HudBatch, atlas: &HudAtlas, cursor: (f32, f32), in_world: bool) {
    let (w, h) = (batch.width, batch.height);
    let (c, scroll, dragging) = {
        let s = state();
        (s.config, s.menu.scroll, s.menu.dragging)
    };
    let l = layout(w, h, scroll);
    ui::menu_background(batch, atlas, 0.0, 0.0, w, h, in_world);
    ui::list_background(batch, atlas, l.top, l.bottom, in_world);
    let start = batch.vertices.len();
    let in_list = cursor.1 >= l.top && cursor.1 < l.bottom;
    for (k, item) in ITEMS.iter().enumerate() {
        let y = l.rows[k];
        if y + 20.0 < l.top || y > l.bottom {
            continue;
        }
        match *item {
            Item::Header(name) => {
                ui::centered_text(batch, atlas, &Text::literal(name), 0.0, y + 4.0, w, u32::MAX);
            }
            Item::Switch(label, i) | Item::Duration(label, i) => {
                batch.text(atlas, label, l.label_x, y + 4.0, u32::MAX, true);
                let [vx, vw] = l.value;
                let hover = in_list && ui::hovered(cursor, vx, y, vw, 20.0);
                if let Item::Switch(..) = *item {
                    let on = switch_of(&c, i);
                    ui::button(batch, atlas, vx, y, vw, 20.0, &Text::literal(""), true, hover);
                    let (text, color) = if on { ("Yes", rgba(85, 255, 85, 255)) } else { ("No", rgba(255, 85, 85, 255)) };
                    ui::centered_text(batch, atlas, &Text::literal(text), vx + 2.0, y + 6.0, vw - 4.0, color);
                } else {
                    let ms = ms_of(&c, i);
                    let value = ms as f32 / MAX_MS as f32;
                    let label = Text::literal(format!("{ms}ms"));
                    ui::slider(batch, atlas, vx, y, vw, &label, value, hover, true, dragging == Some(i));
                }
                let [rx, rw] = l.reset;
                let changed = !is_default(&c, i);
                let rh = changed && in_list && ui::hovered(cursor, rx, y, rw, 20.0);
                ui::button(batch, atlas, rx, y, rw, 20.0, &Text::literal(""), changed, rh);
                reset_icon(batch, atlas, rx + 6.0, y + 6.0, u32::MAX);
            }
        }
    }
    batch.clip_since(start, 0.0, l.top, w, l.bottom);
    ui::separators(batch, atlas, l.top, l.bottom, in_world);
    ui::centered_text(batch, atlas, &Text::literal(TITLE), 0.0, 8.0, w, u32::MAX);
    let [dx, dy, dw, dh] = l.done;
    ui::button(batch, atlas, dx, dy, dw, dh, &Text::literal("Done"), true, ui::hovered(cursor, dx, dy, dw, dh));
}

/// What a press on the screen did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuOut {
    None,
    /// A button was pressed (the click sound).
    Click,
    /// Done: back to the Options screen.
    Close,
}

/// The mouse on the config screen: `pressed` down or up at (`x`, `y`).
pub fn menu_mouse(w: f32, h: f32, x: f32, y: f32, pressed: bool) -> MenuOut {
    let mut s = state();
    if !pressed {
        if s.menu.dragging.take().is_some() {
            save(&s);
        }
        return MenuOut::None;
    }
    let l = layout(w, h, s.menu.scroll);
    if inside(l.done, x, y) {
        s.menu = Menu::default();
        return MenuOut::Close;
    }
    if y < l.top || y >= l.bottom {
        return MenuOut::None;
    }
    for (k, item) in ITEMS.iter().enumerate() {
        let ry = l.rows[k];
        let i = match *item {
            Item::Switch(_, i) | Item::Duration(_, i) => i,
            Item::Header(_) => continue,
        };
        if inside([l.reset[0], ry, l.reset[1], 20.0], x, y) {
            if is_default(&s.config, i) {
                return MenuOut::None;
            }
            reset(&mut s.config, i);
            save(&s);
            return MenuOut::Click;
        }
        if inside([l.value[0], ry, l.value[1], 20.0], x, y) {
            return match *item {
                Item::Switch(..) => {
                    flip(&mut s.config, i);
                    save(&s);
                    MenuOut::Click
                }
                _ => {
                    s.menu.dragging = Some(i);
                    let ms = slider_ms(l.value, x);
                    set_ms(&mut s.config, i, ms);
                    MenuOut::Click
                }
            };
        }
    }
    MenuOut::None
}

fn slider_ms(value: [f32; 2], x: f32) -> u32 {
    let v = ui::slider_value(value[0], value[1], x);
    let steps = (MAX_MS / STEP_MS) as f32;
    ((v * steps).round() as u32) * STEP_MS
}

/// The pointer moving with the button down: the slider being dragged follows it.
pub fn menu_drag(w: f32, h: f32, x: f32) {
    let mut s = state();
    let Some(i) = s.menu.dragging else {
        return;
    };
    let l = layout(w, h, s.menu.scroll);
    let ms = slider_ms(l.value, x);
    set_ms(&mut s.config, i, ms);
}

pub fn menu_scroll(w: f32, h: f32, amount: f32) {
    let mut s = state();
    let l = layout(w, h, 0.0);
    s.menu.scroll = (s.menu.scroll - amount * 10.0).clamp(0.0, max_scroll(&l));
}
