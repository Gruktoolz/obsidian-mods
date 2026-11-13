// Glue between Horizon's `Game` and `mod_armor_status` runtime state.
use crate::game::Game;
use crate::game::mod_armor_status as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::extras::Corner;
use horizon_client::extras_hud::Corners;
use horizon_client::hud::HudBatch;
use horizon_items::items::ItemStack;

struct DisplaySlot<'a> {
    stack: &'a ItemStack,
    text: Option<String>,
    color: u32,
}

pub(super) fn on_hud(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    corners: &mut Corners,
) {
    if !game.native_on(m::ID) {
        return;
    }
    let settings = {
        let g = &*game;
        m::settings(|key| g.native_setting(m::ID, key))
    };

    // Collect candidate items to display: Armor (0..4), Main hand, Off hand.
    let mut candidates: Vec<&ItemStack> = Vec::with_capacity(6);

    // Armor slots: 0 = head, 1 = chest, 2 = legs, 3 = feet
    for armor_slot in &game.armor {
        if let Some(stack) = armor_slot.as_ref() {
            candidates.push(stack);
        }
    }

    // Hands: 0 = Both, 1 = Main only, 2 = Offhand only, 3 = Off
    let show_main = settings.hands == 0 || settings.hands == 1;
    let show_off = settings.hands == 0 || settings.hands == 2;

    if show_main {
        if let Some(stack) = game.inventory.get(game.selected).and_then(Option::as_ref) {
            candidates.push(stack);
        }
    }
    if show_off {
        if let Some(stack) = game.offhand.as_ref() {
            candidates.push(stack);
        }
    }

    if candidates.is_empty() {
        return;
    }

    // Resolve durability & stack count for each candidate item.
    let items_table = game.items();
    let mut slots: Vec<DisplaySlot<'_>> = Vec::with_capacity(candidates.len());
    let mut max_text_width: f32 = 0.0;

    for stack in candidates {
        let max_damage = items_table.get(&stack.item).map_or(0, |i| i.max_damage);
        let formatted = m::format_item(
            stack.count,
            stack.damage,
            max_damage,
            settings.format,
            settings.show_count,
        );
        let (text, color) = match formatted {
            Some((t, c)) => {
                let w = game.native_text_width(&t);
                if w > max_text_width {
                    max_text_width = w;
                }
                (Some(t), c)
            }
            None => (None, m::COLOR_COUNT),
        };
        slots.push(DisplaySlot { stack, text, color });
    }

    // Row height = 18.0 px (16px icon + 2px margin)
    let row_h = 18.0;
    let box_w = 16.0 + (if max_text_width > 0.0 { max_text_width + 6.0 } else { 4.0 });
    let box_h = (slots.len() as f32 * row_h) + 2.0;

    // Calculate (x, y) based on position setting
    let (x, y) = match settings.position {
        // 0: Hotbar left
        0 => {
            let center = (batch.width * 0.5).floor();
            let has_left_offhand = !game.settings.main_hand_left && game.offhand.is_some();
            let left_boundary = if has_left_offhand { center - 122.0 } else { center - 95.0 };
            let x = (left_boundary - box_w).max(2.0);
            let y = (batch.height - box_h - 2.0).max(2.0);
            (x, y)
        }
        // 1: Hotbar right
        1 => {
            let center = (batch.width * 0.5).floor();
            let has_right_offhand = game.settings.main_hand_left && game.offhand.is_some();
            let right_boundary = if has_right_offhand { center + 122.0 } else { center + 95.0 };
            let x = right_boundary.min(batch.width - box_w - 2.0);
            let y = (batch.height - box_h - 2.0).max(2.0);
            (x, y)
        }
        // Corners: TopLeft (2), BottomLeft (3), TopRight (4), BottomRight (5)
        pos => {
            let corner = match pos {
                2 => Corner::TopLeft,
                3 => Corner::BottomLeft,
                4 => Corner::TopRight,
                _ => Corner::BottomRight,
            };
            if !game.native_corner_shown(corner) {
                return;
            }
            corners.place(corner, box_w, box_h)
        }
    };

    // Draw background box if enabled
    if settings.background {
        game.native_fill(batch, [x, y, box_w, box_h], m::BACKGROUND);
    }

    // Draw each item icon and its durability / count label
    for (i, slot) in slots.iter().enumerate() {
        let item_y = y + 1.0 + (i as f32 * row_h);
        game.draw_item_model(batch, slot.stack, x + 2.0, item_y);

        if let Some(text) = &slot.text {
            game.native_text(
                batch,
                text,
                [x + 20.0, item_y + 4.0],
                slot.color,
                true,
            );
        }
    }
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

pub(super) fn on_settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let g = &*game;
    let settings = m::settings(|key| g.native_setting(m::ID, key));
    buttons.extend(settings.labels());
}

pub(super) fn on_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    let mut settings = {
        let g = &*game;
        m::settings(|key| g.native_setting(m::ID, key))
    };
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
