use super::Game;
use super::umbx_flair_auras as auras;
use super::umbx_flair_back as back;
use super::umbx_flair_bandanas as bandanas;
use super::umbx_flair_capes as capes;
use super::umbx_flair_catalog::{self as catalog, Layout};
use super::umbx_flair_critters as critters;
use super::umbx_flair_custom as custom;
use super::umbx_flair_hats as hats;
use super::umbx_flair_hd_capes as hd_capes;
use super::umbx_flair_items::{Item, SLOTS, Slot, THEMES};
use super::umbx_flair_pets as pets;
use super::umbx_flair_render::{self as render, Shape};
use super::umbx_flair_screen as screen;
use super::umbx_flair_text as text;
use std::sync::{Mutex, MutexGuard};
use std::time::Instant;

pub(super) const FRONT_YAW: f32 = 200.0;
const BACK_YAW: f32 = 20.0;
const HIDE_ARMOR: &str = "hide_armor";
const THEME: &str = "theme.aura";
const SWING_RATE: f32 = 5.0;
const LIFT_PER_SPEED: f32 = 5.0;
const MAX_LIFT: f32 = 1.1;
const FALL_LIFT: f32 = 2.0;
const MAX_FALL_LIFT: f32 = 0.5;
const SIDE_PER_SPEED: f32 = 3.0;
const MAX_SIDE: f32 = 0.4;

pub(super) struct Entry {
    pub(super) item: Item,
    pub(super) shape: Option<Shape>,
}

pub(super) struct State {
    pub(super) open: bool,
    pub(super) category: usize,
    pub(super) scroll: f32,
    pub(super) entries: Vec<Entry>,
    pub(super) equipped: [Option<String>; 5],
    pub(super) first_person: [bool; 5],
    pub(super) hide_armor: bool,
    pub(super) theme: usize,
    pub(super) preview_yaw: f32,
    pub(super) dragging: Option<f32>,
    pub(super) swing: [f32; 2],
    swung: Option<Instant>,
    builtin: usize,
    ready: bool,
    uploaded: Vec<(String, usize)>,
    original: Option<String>,
    started: Option<Instant>,
}

static STATE: Mutex<State> = Mutex::new(State {
    open: false,
    category: 0,
    scroll: 0.0,
    entries: Vec::new(),
    equipped: [None, None, None, None, None],
    first_person: [false; 5],
    hide_armor: false,
    theme: 0,
    preview_yaw: FRONT_YAW,
    dragging: None,
    swing: [0.0; 2],
    swung: None,
    builtin: 0,
    ready: false,
    uploaded: Vec::new(),
    original: None,
    started: None,
});

pub(super) fn state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(super) fn texture(item: &Item, layer: usize) -> String {
    format!("flair/{}#{layer}", item.key)
}

fn entry(item: Item) -> Entry {
    Entry { shape: (!item.bones.is_empty()).then(|| render::shape(&item)), item }
}

impl State {
    pub(super) fn millis(&self) -> u128 {
        self.started.map_or(0, |s| s.elapsed().as_millis())
    }

    pub(super) fn seconds(&self) -> f32 {
        self.millis() as f32 / 1000.0
    }

    pub(super) fn shown(&self) -> Vec<usize> {
        (0..self.entries.len()).filter(|i| self.entries[*i].item.slot == SLOTS[self.category]).collect()
    }

    pub(super) fn worn(&self, slot: Slot) -> Option<usize> {
        let key = self.equipped[slot as usize].as_deref()?;
        self.entries.iter().position(|e| e.item.slot == slot && e.item.key == key)
    }

    pub(super) fn cape_shown(&self) -> bool {
        self.worn(Slot::Cape).is_some() && self.worn(Slot::Back).is_none()
    }

    pub(super) fn wearing(&self) -> Vec<(Slot, usize)> {
        SLOTS.iter().filter_map(|s| self.worn(*s).map(|i| (*s, i))).collect()
    }

    pub(super) fn equip(&mut self, game: &Game<'_>, slot: Slot, key: Option<String>) {
        game.native_set_setting(catalog::ID, slot.key(), key.as_deref().unwrap_or(""));
        if key.is_some() {
            self.preview_yaw = if matches!(slot, Slot::Cape | Slot::Back) { BACK_YAW } else { FRONT_YAW };
        }
        self.equipped[slot as usize] = key;
    }

    pub(super) fn toggle_first_person(&mut self, game: &Game<'_>, slot: Slot) {
        let on = !self.first_person[slot as usize];
        self.first_person[slot as usize] = on;
        game.native_set_setting(catalog::ID, &first_person_key(slot), &on.to_string());
    }

    pub(super) fn toggle_hide_armor(&mut self, game: &Game<'_>) {
        self.hide_armor = !self.hide_armor;
        game.native_set_setting(catalog::ID, HIDE_ARMOR, &self.hide_armor.to_string());
    }

    pub(super) fn next_theme(&mut self, game: &Game<'_>) {
        self.theme = (self.theme + 1) % THEMES.len();
        game.native_set_setting(catalog::ID, THEME, THEMES[self.theme].0);
    }

    pub(super) fn load(&mut self, game: &Game<'_>) {
        if !self.ready {
            let font = text::font();
            let write = |words: &str| text::write(font.as_ref(), words);
            let items = [capes::items(), hd_capes::items(), bandanas::items(&write), hats::items(), back::items(), pets::items(), critters::items(), auras::items()];
            self.entries = items.into_iter().flatten().map(entry).collect();
            self.builtin = self.entries.len();
            self.equipped = SLOTS.map(|slot| game.native_setting(catalog::ID, slot.key()).filter(|key| !key.is_empty()));
            self.first_person = SLOTS.map(|slot| game.native_setting(catalog::ID, &first_person_key(slot)).as_deref() == Some("true"));
            self.hide_armor = game.native_setting(catalog::ID, HIDE_ARMOR).as_deref() == Some("true");
            let theme = game.native_setting(catalog::ID, THEME);
            self.theme = THEMES.iter().position(|(name, _)| Some(*name) == theme.as_deref()).unwrap_or(0);
            self.ready = true;
        }
        self.entries.truncate(self.builtin);
        self.entries.extend(custom::items(&game.native_data_dir(catalog::ID)).into_iter().map(entry));
        self.uploaded.clear();
    }

    fn swing(&mut self, game: &Game<'_>) {
        let dt = self.swung.replace(Instant::now()).map_or(0.0, |at| at.elapsed().as_secs_f32()).min(0.1);
        let Some(m) = game.player_model.as_ref() else { return };
        let yaw = m.body_yaw.to_radians();
        let (fx, fz) = (-yaw.sin(), yaw.cos());
        let (vx, vy, vz) = ((m.position.x - m.previous.x) as f32, (m.position.y - m.previous.y) as f32, (m.position.z - m.previous.z) as f32);
        let lift = (vx * fx + vz * fz) * LIFT_PER_SPEED;
        let target = [lift.clamp(0.0, MAX_LIFT) + (-vy * FALL_LIFT).clamp(0.0, MAX_FALL_LIFT), ((vx * fz - vz * fx) * SIDE_PER_SPEED).clamp(-MAX_SIDE, MAX_SIDE)];
        let k = 1.0 - (-dt * SWING_RATE).exp();
        for (now, goal) in self.swing.iter_mut().zip(target) {
            *now += (goal - *now) * k;
        }
    }
}

pub(super) fn toggles(slot: Slot) -> bool {
    matches!(slot, Slot::Pet | Slot::Aura)
}

fn first_person_key(slot: Slot) -> String {
    format!("first_person.{}", slot.key())
}

pub(super) fn listed(mods: &mut Vec<super::native_hooks::NativeMod>) {
    mods.push(super::native_hooks::NativeMod {
        id: catalog::ID.into(),
        title: format!("Flair {}", catalog::VERSION),
        description: catalog::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

pub(super) fn frame(game: &mut Game<'_>) {
    let local = game.player_model.as_ref().map(|m| m.id);
    if !game.native_on(catalog::ID) {
        horizon_client::mobs::render::umbx_flair_armor::set(false, local);
        return;
    }
    let mut flair = state();
    flair.started.get_or_insert_with(Instant::now);
    if !flair.ready {
        flair.load(game);
    }
    horizon_client::mobs::render::umbx_flair_armor::set(flair.hide_armor, local);
    flair.swing(game);
    let ms = flair.millis();
    let mut wanted: Vec<usize> = SLOTS.iter().filter_map(|s| flair.worn(*s)).collect();
    if flair.open {
        let (w, h) = game.gui_size();
        let shown = flair.shown();
        wanted.extend(Layout::new(w, h, screen::previewing(game)).visible(shown.len(), flair.scroll).map(|i| shown[i]));
    }
    wanted.retain(|i| flair.entries[*i].item.textured());
    for i in wanted {
        let item = &flair.entries[i].item;
        let frame = item.frame(ms);
        let names: Vec<String> = (0..item.layer_count()).map(|layer| texture(item, layer)).collect();
        let stale = names.iter().any(|name| !flair.uploaded.contains(&(name.clone(), frame)) || !game.gui.atlas.sprites.contains_key(name));
        if stale {
            let canvases = item.canvases(ms).into_owned();
            for (name, canvas) in names.iter().zip(canvases) {
                game.store_skin(name.clone(), canvas.0);
            }
            flair.uploaded.retain(|(name, _)| !names.contains(name));
            flair.uploaded.extend(names.iter().map(|name| (name.clone(), frame)));
        }
        let mut downloads = game.skin_downloads.borrow_mut();
        for name in &names {
            downloads.drawn(name);
        }
    }
}

pub(super) fn wear(game: &mut Game<'_>, camera: glam::DVec3, alpha: f64, out: &mut Vec<horizon_client::mesher::Vertex>) {
    if !game.native_on(catalog::ID) {
        return;
    }
    cape(game);
    render::first_person(game, camera, alpha, out);
}

fn cape(game: &mut Game<'_>) {
    let mut flair = state();
    let covered = flair.worn(Slot::Cape).is_some() || flair.worn(Slot::Back).is_some();
    let Some(model) = game.player_model.as_mut() else { return };
    if model.cape.is_some() {
        flair.original = model.cape.clone();
    }
    model.cape = if covered { None } else { flair.original.clone() };
}
