// Shared HUD renderer for game HUD and editor preview overlay.
use super::Game;
use crate::game::basic_hud as m;
use crate::game::basic_hud_data as data;
use crate::game::basic_hud_i18n as i18n;
use crate::game::basic_hud_logic as logic;
use horizon_client::hud::HudBatch;

pub const ROW_HEIGHT: f32 = 11.0;
pub const COMPACT_ROW_HEIGHT: f32 = 9.5;
pub const ICON_SIZE: f32 = 10.0;
pub const COMPACT_ICON_SIZE: f32 = 8.5;
pub const GRID_MAX_WIDTH: f32 = 120.0;

fn draw_values(game: &mut Game<'_>, batch: &mut HudBatch, config: &m::Config, widget: m::Widget,
    rows: &[data::Row], icon_size: f32, preview: bool) -> Option<m::Rect> {
    let rows: Vec<_> = rows.iter().filter(|row| !matches!(row.icon, data::Icon::None)).collect();
    if rows.is_empty() { return None; }
    let style = config.widget(widget);
    let scale = style.scale.clamp(0.5, 3.0);
    let padding = style.padding.clamp(0.0, 12.0);
    let max_width = (if widget == m::Widget::ArmorMini { 180.0 } else { GRID_MAX_WIDTH }).min(batch.width / scale);
    let available_text = (max_width - padding * 2.0 - icon_size - 3.0).max(1.0);
    let text_metrics: Vec<_> = rows.iter().map(|row| {
        let width = game.native_text_width(&row.value);
        let font_scale = (available_text / width.max(1.0)).min(1.0);
        (width * font_scale, font_scale)
    }).collect();
    let widths: Vec<_> = rows.iter().zip(&text_metrics).map(|(row, (width, _))|
        icon_size + if row.value.is_empty() { 0.0 } else { 3.0 + width }).collect();
    let (positions, base) = m::icon_value_layout(&widths, icon_size.max(8.0), padding, max_width);
    let rect = m::widget_rect(config, widget, base, [batch.width, batch.height]);
    let start = batch.vertices.len();
    if style.opacity > 0 {
        let alpha = (u32::from(style.opacity.min(100)) * 255 + 50) / 100;
        game.native_fill(batch, [rect.x, rect.y, base[0], base[1]], (alpha << 24) | (style.background & 0xFF_FFFF));
    }
    for ((row, at), (_, font_scale)) in rows.iter().zip(&positions).zip(&text_metrics) {
        let x = rect.x + at[0];
        let y = rect.y + at[1];
        match &row.icon {
            data::Icon::Effect(sprite) => {
                if let Some([sw, sh]) = game.native_sprite_size(sprite) {
                    game.native_sprite(batch, sprite, [0.0, 0.0, sw, sh], [x, y, icon_size, icon_size]);
                }
            }
            data::Icon::Item(stack) => {
                let first = batch.vertices.len();
                game.draw_item_model(batch, stack, x, y);
                batch.scale_since(first, x, y, icon_size / 16.0, icon_size / 16.0);
            }
            data::Icon::None => {}
        }
        if !row.value.is_empty() {
            let tx = x + icon_size + 3.0;
            let ty = y + (icon_size.max(8.0) - 8.0 * font_scale) * 0.5;
            let first = batch.vertices.len();
            game.native_text(batch, &row.value, [tx, ty], logic::widget_text_color(style.text, row.color), style.shadow);
            batch.scale_since(first, tx, ty, *font_scale, *font_scale);
        }
    }
    if preview && rows.iter().any(|row| row.sample) {
        let label = format!("[{}]", i18n::tr(config.language(&game.settings.language), "Sample"));
        let font_scale = ((base[0] - 2.0).max(1.0) / game.native_text_width(&label).max(1.0)).min(1.0);
        let y = (rect.y - 10.0).max(0.0);
        let first = batch.vertices.len();
        game.native_text(batch, &label, [rect.x + 1.0, y], style.accent & 0xFF_FFFF, style.shadow);
        batch.scale_since(first, rect.x + 1.0, y, font_scale, font_scale);
    }
    if preview {
        if !config.enabled(widget) {
            game.native_fill(batch, [rect.x, rect.y, base[0], 1.0], 0xFFAA_4455);
            game.native_fill(batch, [rect.x, rect.y + base[1] - 1.0, base[0], 1.0], 0xFFAA_4455);
        }
    }
    batch.scale_since(start, rect.x, rect.y, scale, scale);
    Some(rect)
}

fn draw_mini(game: &mut Game<'_>, batch: &mut HudBatch, config: &m::Config, preview: bool) -> Option<m::Rect> {
    let widget = m::Widget::ArmorMini;
    if !preview && !config.enabled(widget) { return None; }
    let mut equipment: Vec<_> = game.armor.iter().cloned().collect();
    let sample = equipment.iter().all(Option::is_none);
    if sample && !preview { return None; }
    if sample {
        equipment = ["minecraft:diamond_helmet", "minecraft:elytra", "minecraft:diamond_leggings", "minecraft:diamond_boots"]
            .into_iter().enumerate().map(|(i, name)| {
                let mut item = horizon_client::items::ItemStack::new(name, 1);
                item.damage = [30, 120, 280, 0][i];
                Some(item)
            }).collect();
    }
    let style = config.widget(widget);
    let is_icons_only = style.icons_only && widget.supports_icons_only();
    if is_icons_only {
        let rows: Vec<_> = equipment.iter().flatten().map(|item| {
            let info = data::durability(game, item);
            data::Row {
                icon: data::Icon::Item(item.clone()), text: String::new(), sample,
                value: info.map_or_else(String::new, |(left, _)| left.to_string()),
                color: info.map_or(m::INK, |(left, max)| m::durability_colour(left, max)),
            }
        }).collect();
        return draw_values(game, batch, config, widget, &rows, 12.0, preview);
    }
    let padding = style.padding.clamp(0.0, 12.0);
    let caption_h = if sample { 10.0 } else { 0.0 };
    let numbers_h = if config.mini_numbers && !is_icons_only { 7.0 } else { 0.0 };
    let base = [56.0 + padding * 2.0, (if is_icons_only { 12.0 } else { 16.0 }) + padding * 2.0 + numbers_h + caption_h];
    let rect = m::widget_rect(config, widget, base, [batch.width, batch.height]);
    let start = batch.vertices.len();
    if style.opacity > 0 {
        let alpha = (u32::from(style.opacity.min(100)) * 255 + 50) / 100;
        game.native_fill(batch, [rect.x, rect.y, base[0], base[1]], (alpha << 24) | (style.background & 0xFF_FFFF));
    }
    let language = config.language(&game.settings.language).to_owned();
    let lang = language.as_str();
    if sample {
        let label = format!("[{}]", i18n::tr(lang, "Sample"));
        game.native_text(batch, &label, [rect.x + padding, rect.y + padding], style.accent & 0xFF_FFFF, style.shadow);
    }
    for (slot, item) in equipment.iter().enumerate() {
        let Some(item) = item else { continue; };
        let x = rect.x + padding + slot as f32 * 14.0;
        let y = rect.y + padding + caption_h;
        if style.icons || is_icons_only {
            let icon_start = batch.vertices.len();
            game.draw_item_model(batch, item, x, y);
            batch.scale_since(icon_start, x, y, 0.75, 0.75);
        }
        if !is_icons_only {
            if let Some((left, max)) = data::durability(game, item) {
                let status = m::durability_colour(left, max);
                let color = if style.accent != m::WidgetConfig::DEFAULT_ACCENT { style.accent } else { status };
                let fraction = (left as f32 / max as f32).clamp(0.0, 1.0);
                game.native_fill(batch, [x, y + 13.0, 12.0, 2.0], 0xFF00_0000);
                game.native_fill(batch, [x, y + 13.0, 12.0 * fraction, 1.0], 0xFF00_0000 | (color & 0xFF_FFFF));
                if config.mini_numbers {
                    let text = left.to_string();
                    let scale = (12.0 / game.native_text_width(&text).max(1.0)).min(0.6);
                    let tx = x + (12.0 - game.native_text_width(&text) * scale) * 0.5;
                    let ty = y + 16.0;
                    let text_start = batch.vertices.len();
                    game.native_text(batch, &text, [tx, ty], logic::widget_text_color(style.text, status), style.shadow);
                    batch.scale_since(text_start, tx, ty, scale, scale);
                }
            }
        }
    }
    if preview && !config.enabled(widget) {
        game.native_fill(batch, [rect.x, rect.y, base[0], 1.0], 0xFFAA_4455);
        game.native_fill(batch, [rect.x, rect.y + base[1] - 1.0, base[0], 1.0], 0xFFAA_4455);
    }
    batch.scale_since(start, rect.x, rect.y, style.scale.clamp(0.5, 3.0), style.scale.clamp(0.5, 3.0));
    Some(rect)
}

/// Draws widgets for either the live game HUD or the editor preview.
/// Returns exact screen hitboxes for all rendered/previewed widgets in GUI coordinates.
pub(super) fn draw(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    config: &m::Config,
    preview: bool,
) -> Vec<(m::Widget, m::Rect)> {
    let language = config.language(&game.settings.language).to_owned();
    let lang = language.as_str();
    let ru = lang.starts_with("ru");
    let collected = data::collect(game, config, preview, ru);
    let mut hitboxes = Vec::new();
    let viewport = [batch.width, batch.height];

    for (widget, rows) in collected {
        if widget == m::Widget::ArmorMini {
            if let Some(rect) = draw_mini(game, batch, config, preview) { hitboxes.push((widget, rect)); }
            continue;
        }
        if !preview && (!config.enabled(widget) || rows.is_empty()) {
            continue;
        }

        let wcfg = config.widget(widget);
        let padding = wcfg.padding.clamp(0.0, 12.0);
        let row_height = if wcfg.compact { COMPACT_ROW_HEIGHT } else { ROW_HEIGHT };
        let icon_size = if wcfg.compact { COMPACT_ICON_SIZE } else { ICON_SIZE };
        let cell = icon_size + 3.0;
        let is_icons_only = wcfg.icons_only && widget.supports_icons_only();

        if is_icons_only {
            if let Some(rect) = draw_values(game, batch, config, widget, &rows, icon_size, preview) {
                hitboxes.push((widget, rect));
            }
            continue;
        }

        let has_icons = wcfg.icons && rows.iter().any(|r| !matches!(r.icon, data::Icon::None));
        let icon_col_width = if has_icons { cell } else { 0.0 };

        let (base_w, base_h) = if rows.is_empty() {
            let sample_title = format!("[{}] {}", i18n::tr(lang, "Sample"), i18n::tr(lang, widget.title(false)));
            let text_w = game.native_text_width(&sample_title);
            (padding * 2.0 + text_w + icon_col_width + 4.0, padding * 2.0 + row_height)
        } else {
            let max_text_width = rows.iter().map(|r| game.native_text_width(&r.text)).fold(0.0, f32::max);
            (
                padding * 2.0 + max_text_width + icon_col_width + 4.0,
                padding * 2.0 + row_height * rows.len() as f32,
            )
        };

        let base_w = base_w.max(20.0);
        let base_h = base_h.max(row_height);

        let rect = m::widget_rect(config, widget, [base_w, base_h], viewport);
        let start = batch.vertices.len();

        // 1. Render background with ARGB opacity conversion
        let opacity = wcfg.opacity.min(100);
        if opacity > 0 {
            let a = ((opacity as u32 * 255 + 50) / 100).min(255) as u8;
            let argb = (u32::from(a) << 24) | (wcfg.background & 0x00FF_FFFF);
            game.native_fill(batch, [rect.x, rect.y, base_w, base_h], argb);
        }

        // 2. Render content rows
        if rows.is_empty() {
            let sample_title = format!("[{}] {}", i18n::tr(lang, "Sample"), i18n::tr(lang, widget.title(false)));
            let text_x = rect.x + padding + icon_col_width;
            let text_y = rect.y + padding + (row_height - 8.0) * 0.5;
            let color = if wcfg.text != m::WidgetConfig::DEFAULT_TEXT {
                wcfg.text
            } else {
                m::INK
            };
            game.native_text(batch, &sample_title, [text_x, text_y], color & 0x00FF_FFFF, wcfg.shadow);
        } else {
            for (i, row) in rows.iter().enumerate() {
                let row_top = rect.y + padding + i as f32 * row_height;
                if has_icons {
                    let icon_x = rect.x + padding;
                    let icon_y = row_top + (row_height - icon_size) * 0.5;
                    match &row.icon {
                        data::Icon::Effect(sprite) => {
                            if let Some([sw, sh]) = game.native_sprite_size(sprite) {
                                game.native_sprite(batch, sprite, [0.0, 0.0, sw, sh], [icon_x, icon_y, icon_size, icon_size]);
                            }
                        }
                        data::Icon::Item(stack) => {
                            let item_start = batch.vertices.len();
                            game.draw_item_model(batch, stack, icon_x, icon_y);
                            batch.scale_since(item_start, icon_x, icon_y, icon_size / 16.0, icon_size / 16.0);
                        }
                        data::Icon::None => {}
                    }
                }

                let text_x = rect.x + padding + icon_col_width;
                let text_y = row_top + (row_height - 8.0) * 0.5;
                let color = logic::widget_text_color(wcfg.text, row.color);

                if let Some((label, val)) = row.text.split_once(": ") {
                    let label_str = format!("{label}: ");
                    let label_w = game.native_text_width(&label_str);
                    let accent = if wcfg.accent != m::WidgetConfig::DEFAULT_ACCENT {
                        wcfg.accent
                    } else {
                        m::WidgetConfig::DEFAULT_ACCENT
                    };
                    game.native_text(batch, &label_str, [text_x, text_y], accent & 0x00FF_FFFF, wcfg.shadow);
                    game.native_text(batch, val, [text_x + label_w, text_y], color & 0x00FF_FFFF, wcfg.shadow);
                } else {
                    game.native_text(batch, &row.text, [text_x, text_y], color & 0x00FF_FFFF, wcfg.shadow);
                }
            }
        }

        // 3. In preview mode, add disabled outline if widget is disabled
        if preview && !config.enabled(widget) {
            let dis_border = 0x80FF_4444;
            game.native_fill(batch, [rect.x, rect.y, base_w, 1.0], dis_border);
            game.native_fill(batch, [rect.x, rect.y + base_h - 1.0, base_w, 1.0], dis_border);
            game.native_fill(batch, [rect.x, rect.y, 1.0, base_h], dis_border);
            game.native_fill(batch, [rect.x + base_w - 1.0, rect.y, 1.0, base_h], dis_border);
        }

        // 4. Scale vertices pushed since start around (rect.x, rect.y)
        let scale = if wcfg.scale.is_finite() {
            wcfg.scale.clamp(0.5, 3.0)
        } else {
            1.0
        };
        batch.scale_since(start, rect.x, rect.y, scale, scale);

        hitboxes.push((widget, rect));
    }

    hitboxes
}
