use std::sync::PoisonError;

use crate::game::Game;
use crate::game::official_elytratrails as m;

impl Game<'_> {
    pub(super) fn elytratrails_settings(&self) -> m::Settings {
        let mut cache = m::CACHE.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(settings) = *cache {
            return settings;
        }
        let settings = m::Settings::read(|key| self.native_setting(m::ID, key));
        *cache = Some(settings);
        settings
    }

    pub(super) fn elytratrails_page_buttons(&self) -> Vec<String> {
        let settings = self.elytratrails_settings();
        match m::page() {
            m::PAGE_OPTIMIZATION => m::optimization_labels(settings),
            _ => m::labels(settings),
        }
    }

    pub(super) fn elytratrails_page_click(&mut self, button: usize) {
        if m::page() == m::PAGE_OPTIMIZATION {
            self.elytratrails_optimization_click(button);
            return;
        }
        let settings = self.elytratrails_settings();
        if button == m::OPEN_OPTIMIZATION {
            m::set_page(m::PAGE_OPTIMIZATION);
            return;
        }
        if button == 4 {
            let next = m::next_duration(settings.duration).to_string();
            self.native_set_setting(m::ID, m::KEYS[4], &next);
            m::invalidate();
            return;
        }
        let (key, value) = match button {
            0 => (m::KEYS[0], m::flag(!settings.trails)),
            1 => (m::KEYS[1], m::flag(!settings.own)),
            2 => (m::KEYS[2], m::flag(!settings.others)),
            3 => (m::KEYS[3], m::cycle(&m::COLORS, settings.color)),
            5 => (m::KEYS[5], m::cycle(&m::WIDTHS, settings.width)),
            6 => (m::KEYS[6], m::cycle(&m::PATTERNS, settings.pattern)),
            7 => (m::KEYS[7], m::flag(!settings.pulse)),
            m::DAY_NIGHT_TOGGLE => (m::KEYS[14], m::flag(!settings.day_night)),
            m::FIRST_PERSON_TOGGLE => (m::KEYS[15], m::flag(!settings.first_person)),
            _ => return,
        };
        self.native_set_setting(m::ID, key, value);
        m::invalidate();
    }

    fn elytratrails_optimization_click(&mut self, button: usize) {
        let settings = self.elytratrails_settings();
        let (key, value) = match button {
            0 => (m::KEYS[8], m::flag(!settings.own_fixed)),
            1 => (m::KEYS[9], m::flag(!settings.others_fixed)),
            2 => (m::KEYS[10], if settings.half_detail { "full" } else { "half" }),
            3 => (m::KEYS[11], m::cycle(&m::RANGES, settings.range)),
            4 => (m::KEYS[12], m::flag(!settings.formation)),
            5 => (m::KEYS[13], m::flag(!settings.flash)),
            m::OPTIMIZATION_BACK => {
                m::set_page(m::PAGE_MAIN);
                return;
            }
            _ => return,
        };
        self.native_set_setting(m::ID, key, value);
        m::invalidate();
    }
}
