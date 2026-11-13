use super::Game;
use super::umbx_flair_catalog::{self as catalog, Hit, Layout, SCROLL_STEP};
use super::umbx_flair_glue::{self as glue, FRONT_YAW, State};
use super::umbx_flair_items::{SLOTS, Slot};
use super::umbx_flair_screen as screen;
use std::ops::ControlFlow;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

const DRAG_TURN: f32 = 1.6;

pub(super) fn window(game: &mut Game<'_>, event: &WindowEvent) -> ControlFlow<()> {
    if !game.native_on(catalog::ID) {
        return ControlFlow::Continue(());
    }
    let size = game.gui_size();
    let cursor = [game.cursor.0, game.cursor.1];
    let mut flair = glue::state();
    if !flair.open {
        let pressed = matches!(event, WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. });
        if !pressed || !screen::button(game, size).is_some_and(|rect| catalog::inside(rect, cursor)) {
            return ControlFlow::Continue(());
        }
        flair.load(game);
        flair.open = true;
        flair.scroll = 0.0;
        flair.preview_yaw = FRONT_YAW;
        game.audio.play_button_click();
        return ControlFlow::Break(());
    }
    let layout = Layout::new(size.0, size.1, screen::previewing(game));
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            let window = game.renderer.as_ref().map_or([1280, 720], |r| [r.config.width, r.config.height]);
            let x = position.x as f32 / horizon_client::hud::HudBatch::new(window).scale;
            if let Some(last) = flair.dragging {
                flair.preview_yaw = (flair.preview_yaw - (x - last) * DRAG_TURN).rem_euclid(360.0);
                flair.dragging = Some(x);
            }
            return ControlFlow::Continue(());
        }
        WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => flair.dragging = None,
        WindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(code) = event.physical_key {
                if event.state == ElementState::Pressed {
                    flair.open &= code != KeyCode::Escape;
                    game.physical_keys.insert(code);
                } else {
                    game.physical_keys.remove(&code);
                }
            }
        }
        WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => click(game, &mut flair, layout, cursor),
        WindowEvent::MouseInput { .. } => {}
        WindowEvent::MouseWheel { delta, .. } => {
            let steps = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y,
                MouseScrollDelta::PixelDelta(p) => p.y as f32 / SCROLL_STEP,
            };
            let max = layout.max_scroll(flair.shown().len());
            flair.scroll = (flair.scroll - steps * SCROLL_STEP).clamp(0.0, max);
        }
        _ => return ControlFlow::Continue(()),
    }
    ControlFlow::Break(())
}

fn click(game: &mut Game<'_>, flair: &mut State, layout: Layout, cursor: [f32; 2]) {
    let shown = flair.shown();
    let worn: Vec<(usize, bool, bool)> = flair.wearing().iter().map(|(slot, _)| (*slot as usize, glue::toggles(*slot), *slot == Slot::Aura)).collect();
    match layout.hit(cursor, SLOTS.len(), shown.len(), flair.scroll, &worn) {
        Hit::Panel => return,
        Hit::Preview => {
            flair.dragging = Some(cursor[0]);
            return;
        }
        Hit::Close | Hit::Outside => flair.open = false,
        Hit::HideArmor => flair.toggle_hide_armor(game),
        Hit::Tab(i) => {
            flair.category = i;
            flair.scroll = 0.0;
        }
        Hit::Remove(slot) => flair.equip(game, SLOTS[slot], None),
        Hit::FirstPerson(slot) => flair.toggle_first_person(game, SLOTS[slot]),
        Hit::Theme(_) => flair.next_theme(game),
        Hit::Card(i) => {
            let item = &flair.entries[shown[i]].item;
            let (slot, key): (Slot, String) = (item.slot, item.key.clone());
            let next = (flair.equipped[slot as usize].as_deref() != Some(key.as_str())).then_some(key);
            flair.equip(game, slot, next);
        }
    }
    game.audio.play_button_click();
}
