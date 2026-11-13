use super::umbx_flair_catalog::{self as catalog, CARD_H, CARD_W, Layout, clip, inside};
use super::umbx_flair_glue::{self as glue, State};
use super::umbx_flair_items::{SLOTS, Slot, THEMES};
use super::umbx_flair_preview::{self as preview, Look};
use super::umbx_flair_render as render;
use super::{Game, Screen};
use horizon_client::hud::HudBatch;
use horizon_client::text::Text;
use horizon_client::title::Mode;
use horizon_client::ui;

const BACKDROP: u32 = 0xB006_0609;
const BORDER: u32 = 0xFF2B_2B3B;
const PANEL: u32 = 0xFF12_1219;
const HEADER: u32 = 0xFF18_1822;
const SIDEBAR: u32 = 0xFF15_151D;
const SIDE: u32 = 0xFF16_1620;
const CARD: u32 = 0xFF1D_1D29;
const CARD_TOP: u32 = 0xFF26_2636;
const HOVER: u32 = 0xFF27_2738;
const SELECTED: u32 = 0xFF25_2140;
const TRACK: u32 = 0xFF22_222E;
const CHIP_OFF: u32 = 0xFF2C_2C3C;
const ACCENT: u32 = 0xFF9B_6BFF;
const TEXT: u32 = 0xEC_ECF4;
const MUTED: u32 = 0x86_869A;
const TITLE_BUTTON_Y: f32 = 96.0;
const PAUSE_GRID: [f32; 2] = [212.0, 166.0];
const PAUSE_LINKS: f32 = 24.0;
const PAUSE_BUTTON: [f32; 3] = [4.0, 98.0, 52.0];
const ICON_INSET: f32 = 4.0;
const NAME_H: f32 = 14.0;
const SHORT: [&str; 5] = ["Cape", "Hat", "Back", "Pet", "Aura"];

pub(super) fn previewing(game: &Game<'_>) -> bool {
    game.title.is_none() && game.player_model.is_some()
}

pub(super) fn button(game: &Game<'_>, (w, h): (f32, f32)) -> Option<[f32; 4]> {
    if matches!(game.screen, Screen::Options | Screen::Win) || game.server_packs.prompt.is_some() || game.code_of_conduct.is_some() || game.dialog_open() {
        return None;
    }
    if let Some(title) = &game.title {
        let (w, h) = (w.ceil(), h.ceil());
        return matches!(title.mode, Mode::Main).then(|| [(w / 2.0).floor() - 100.0, (h / 4.0).floor() + TITLE_BUTTON_Y, 200.0, 20.0]);
    }
    if game.screen != Screen::Paused || !game.pause_menu || game.player.is_none() {
        return None;
    }
    let links = if game.server_links().is_empty() { 0.0 } else { PAUSE_LINKS };
    let x0 = ((w - PAUSE_GRID[0]) * 0.5).floor();
    let y0 = ((h - PAUSE_GRID[1] - links) * 0.25).floor();
    Some([x0 + PAUSE_BUTTON[0], y0 + PAUSE_BUTTON[1], PAUSE_BUTTON[2], 20.0])
}

pub(super) fn draw(game: &Game<'_>, batch: &mut HudBatch) {
    if !game.native_on(catalog::ID) {
        return;
    }
    let mut flair = glue::state();
    let Some(rect) = button(game, (batch.width, batch.height)) else {
        flair.open = false;
        return;
    };
    if !flair.open {
        let hover = ui::hovered(game.cursor, rect[0], rect[1], rect[2], rect[3]);
        ui::button(batch, &game.gui.atlas, rect[0], rect[1], rect[2], rect[3], &Text::literal("Flair"), true, hover);
        return;
    }
    let layout = Layout::new(batch.width, batch.height, previewing(game));
    let cursor = [game.cursor.0, game.cursor.1];
    frame(game, batch, &flair, &layout, cursor);
    cards(game, batch, &flair, &layout, cursor);
    wearing(game, batch, &flair, &layout, cursor);
    let mut strokes = Vec::new();
    if let Some(i) = flair.worn(Slot::Aura) {
        flair.entries[i].item.stroke(f64::from(flair.seconds()), f64::from(flair.preview_yaw), flair.theme, &mut strokes);
    }
    let covered = flair.worn(Slot::Cape).is_some() || flair.worn(Slot::Back).is_some();
    let look = Look { yaw: flair.preview_yaw, strokes, covered };
    drop(flair);
    let [x, y, w, h] = layout.side;
    match layout.preview {
        Some([px, py, pw, ph]) => {
            centered(game, batch, "Preview", [px, py + 6.0, pw], MUTED);
            centered(game, batch, "Drag to turn", [px, py + ph - 12.0, pw], MUTED);
            preview::player(game, batch, [px + 4.0, py + 18.0, pw - 8.0, ph - 32.0], &look);
        }
        None => {
            let middle = y + ((h - layout.wearing[3]) / 2.0).floor();
            centered(game, batch, "Open the catalog in a", [x, middle - 10.0, w], MUTED);
            centered(game, batch, "world to see yourself", [x, middle + 2.0, w], MUTED);
        }
    }
}

fn frame(game: &Game<'_>, batch: &mut HudBatch, flair: &State, layout: &Layout, cursor: [f32; 2]) {
    let [x, y, w, h] = layout.panel;
    game.native_fill(batch, [0.0, 0.0, batch.width, batch.height], BACKDROP);
    game.native_fill(batch, [x - 1.0, y - 1.0, w + 2.0, h + 2.0], BORDER);
    game.native_fill(batch, layout.panel, PANEL);
    let header = layout.header();
    game.native_fill(batch, header, HEADER);
    game.native_fill(batch, [x, y + header[3] - 1.0, w, 1.0], ACCENT);
    game.native_text(batch, "Flair", [x + 10.0, y + 8.0], TEXT, true);
    let tab = SLOTS[flair.category].title();
    game.native_text(batch, tab, [x + 18.0 + game.native_text_width("Flair"), y + 8.0], MUTED, false);
    let close = layout.close();
    let over = inside(close, cursor);
    if over {
        game.native_fill(batch, close, HOVER);
    }
    centered(game, batch, "x", [close[0], close[1] + 4.0, close[2]], if over { TEXT } else { MUTED });
    game.native_fill(batch, layout.sidebar(), SIDEBAR);
    for (i, slot) in SLOTS.iter().enumerate() {
        let rect = layout.tab(i);
        let chosen = i == flair.category;
        if chosen || inside(rect, cursor) {
            game.native_fill(batch, rect, if chosen { SELECTED } else { HOVER });
        }
        if chosen {
            game.native_fill(batch, [rect[0], rect[1], 2.0, rect[3]], ACCENT);
        }
        game.native_text(batch, slot.title(), [rect[0] + 8.0, rect[1] + 6.0], if chosen { TEXT } else { MUTED }, false);
        if flair.equipped[*slot as usize].is_some() {
            game.native_fill(batch, [rect[0] + rect[2] - 8.0, rect[1] + 8.0, 4.0, 4.0], ACCENT);
        }
    }
    let armor = layout.hide_armor();
    if inside(armor, cursor) {
        game.native_fill(batch, armor, HOVER);
    }
    game.native_text(batch, "Hide armor", [armor[0] + 6.0, armor[1] + 6.0], if flair.hide_armor { TEXT } else { MUTED }, false);
    switch(game, batch, [armor[0] + armor[2] - 20.0, armor[1] + 6.0], flair.hide_armor);
    game.native_fill(batch, layout.side, SIDE);
    game.native_fill(batch, [layout.side[0], layout.side[1], 1.0, layout.side[3]], BORDER);
}

fn cards(game: &Game<'_>, batch: &mut HudBatch, flair: &State, layout: &Layout, cursor: [f32; 2]) {
    let shown = flair.shown();
    let content = layout.content;
    if shown.is_empty() {
        centered(game, batch, "Nothing here yet", [content[0], content[1] + content[3] / 2.0 - 4.0, content[2]], MUTED);
        return;
    }
    let seconds = flair.seconds();
    for index in layout.visible(shown.len(), flair.scroll) {
        let entry = &flair.entries[shown[index]];
        let card = layout.card(index, flair.scroll);
        let Some(visible) = clip(card, content) else { continue };
        let worn = flair.equipped[entry.item.slot as usize].as_deref() == Some(entry.item.key.as_str());
        let hover = inside(visible, cursor) && inside(content, cursor);
        game.native_fill(batch, visible, if hover { HOVER } else { CARD });
        fill_clipped(game, batch, [card[0], card[1], card[2], 1.0], content, if worn { ACCENT } else { CARD_TOP });
        if worn {
            for edge in [[card[0], card[1] + card[3] - 1.0, card[2], 1.0], [card[0], card[1], 1.0, card[3]], [card[0] + card[2] - 1.0, card[1], 1.0, card[3]]] {
                fill_clipped(game, batch, edge, content, ACCENT);
            }
        }
        let icon = [card[0] + ICON_INSET, card[1] + ICON_INSET, CARD_W - 2.0 * ICON_INSET, CARD_H - NAME_H - 2.0 * ICON_INSET];
        if let Some(area) = clip(icon, content) {
            render::icon(game, batch, entry, seconds, flair.theme, icon, area);
        }
        let label = card[1] + CARD_H - NAME_H + 2.0;
        if label >= content[1] && label + 9.0 <= content[1] + content[3] {
            let name = fitted(game, &entry.item.name, CARD_W - 6.0);
            centered(game, batch, &name, [card[0], label, CARD_W], if worn { ACCENT & 0xFF_FFFF } else { TEXT });
        }
    }
    let max = layout.max_scroll(shown.len());
    if max > 0.0 {
        let track = [content[0] + content[2] + 3.0, content[1], 2.0, content[3]];
        let thumb = (content[3] * content[3] / (content[3] + max)).max(12.0);
        game.native_fill(batch, track, TRACK);
        game.native_fill(batch, [track[0], track[1] + (content[3] - thumb) * flair.scroll / max, 2.0, thumb], ACCENT);
    }
}

fn wearing(game: &Game<'_>, batch: &mut HudBatch, flair: &State, layout: &Layout, cursor: [f32; 2]) {
    let [x, y, w, _] = layout.wearing;
    game.native_fill(batch, [x, y, w, 1.0], BORDER);
    game.native_text(batch, "Wearing", [x + 6.0, y + 5.0], MUTED, false);
    let worn = flair.wearing();
    if worn.is_empty() {
        centered(game, batch, "Nothing yet", [x, y + 30.0, w], MUTED);
        return;
    }
    let back = flair.worn(Slot::Back).is_some();
    for (k, (slot, index)) in worn.into_iter().enumerate() {
        let row = layout.wearing_row(k);
        if inside(row, cursor) {
            game.native_fill(batch, row, HOVER);
        }
        game.native_text(batch, SHORT[slot as usize], [row[0] + 3.0, row[1] + 3.0], MUTED, false);
        let toggles = glue::toggles(slot);
        let themed = slot == Slot::Aura;
        let room = row[2] - 34.0 - if toggles { 20.0 } else { 0.0 } - if themed { 16.0 } else { 0.0 };
        let hidden = slot == Slot::Cape && back;
        let name = fitted(game, &flair.entries[index].item.name, room);
        game.native_text(batch, &name, [row[0] + 31.0, row[1] + 3.0], if hidden { MUTED } else { TEXT }, false);
        if toggles {
            let chip = layout.first_person(k);
            let on = flair.first_person[slot as usize];
            game.native_fill(batch, chip, if on { ACCENT } else { CHIP_OFF });
            centered(game, batch, "1P", [chip[0], chip[1] + 2.0, chip[2]], if on { TEXT } else { MUTED });
        }
        if themed {
            let chip = layout.theme(k);
            let colors = if flair.theme == 0 { flair.entries[index].item.colors } else { THEMES[flair.theme].1 };
            game.native_fill(batch, chip, if inside(chip, cursor) { 0xFFF0_F0F8 } else { BORDER });
            game.native_fill(batch, [chip[0] + 1.0, chip[1] + 1.0, 5.0, 10.0], 0xFF00_0000 | colors[0]);
            game.native_fill(batch, [chip[0] + 6.0, chip[1] + 1.0, 5.0, 10.0], 0xFF00_0000 | colors[1]);
        }
        let remove = layout.remove(k);
        let over = inside(remove, cursor);
        if over {
            game.native_fill(batch, remove, CHIP_OFF);
        }
        centered(game, batch, "x", [remove[0], remove[1] + 2.0, remove[2]], if over { TEXT } else { MUTED });
    }
}

fn switch(game: &Game<'_>, batch: &mut HudBatch, [x, y]: [f32; 2], on: bool) {
    game.native_fill(batch, [x, y, 14.0, 8.0], if on { ACCENT } else { CHIP_OFF });
    game.native_fill(batch, [if on { x + 7.0 } else { x + 1.0 }, y + 1.0, 6.0, 6.0], 0xFFF0_F0F8);
}

fn fill_clipped(game: &Game<'_>, batch: &mut HudBatch, rect: [f32; 4], to: [f32; 4], argb: u32) {
    if let Some(r) = clip(rect, to) {
        game.native_fill(batch, r, argb);
    }
}

fn centered(game: &Game<'_>, batch: &mut HudBatch, text: &str, [x, y, w]: [f32; 3], rgb: u32) {
    game.native_text(batch, text, [x + ((w - game.native_text_width(text)) / 2.0).floor(), y], rgb, false);
}

fn fitted(game: &Game<'_>, name: &str, width: f32) -> String {
    if game.native_text_width(name) <= width {
        return name.to_owned();
    }
    let mut short: String = name.to_owned();
    while !short.is_empty() && game.native_text_width(&format!("{short}..")) > width {
        short.pop();
    }
    format!("{short}..")
}
