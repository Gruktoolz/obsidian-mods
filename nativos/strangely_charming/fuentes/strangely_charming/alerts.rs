use super::{Game, HudBatch, rgba};
use horizon_protocol::packets::{configuration::CustomPayload, play::Clientbound};
use std::{
    cell::RefCell,
    collections::VecDeque,
    time::{Duration, Instant},
};

pub(super) const CHANNEL: &str = "strangely_charming:alert";
struct Alert {
    text: String,
    since: Option<Instant>,
}
thread_local! { static QUEUE: RefCell<VecDeque<Alert>> = RefCell::default(); }
pub(super) fn clear() {
    QUEUE.with_borrow_mut(VecDeque::clear);
}

impl Game<'_> {
    pub(super) fn sc_notice(&mut self, text: impl Into<String>) {
        self.sc_alert(text.into(), false);
    }
    pub(super) fn sc_alert(&mut self, text: String, chime: bool) {
        let mut data = vec![u8::from(chime)];
        data.extend_from_slice(text.as_bytes());
        if self.lan_acting_uuid.is_some() {
            let who = self.native_acting_id();
            self.native_send(
                who,
                Clientbound::CustomPayload(CustomPayload {
                    channel: CHANNEL.into(),
                    data,
                }),
            );
        } else {
            self.sc_receive_alert(&data);
        }
    }
    pub(super) fn sc_receive_alert(&mut self, data: &[u8]) {
        let Some((&sound, bytes)) = data.split_first() else {
            return;
        };
        if bytes.len() > 2048 {
            return;
        }
        let Ok(text) = std::str::from_utf8(bytes) else {
            return;
        };
        if sound == 1 {
            self.play_at_feet("minecraft:entity.player.levelup");
        }
        QUEUE.with_borrow_mut(|queue| {
            if queue.back().is_some_and(|a| a.text == text) {
                return;
            }
            if queue.len() >= 4 {
                queue.pop_front();
            }
            let alert = Alert {
                text: text.to_owned(),
                since: None,
            };
            if sound == 1 {
                queue.push_front(alert);
            } else {
                queue.push_back(alert);
            }
        });
    }
    pub(super) fn sc_draw_alert(&self, batch: &mut HudBatch) {
        QUEUE.with_borrow_mut(|queue| {
            let Some(alert) = queue.front_mut() else {
                return;
            };
            let since = alert.since.get_or_insert_with(Instant::now);
            let duration = Duration::from_secs(if alert.text.len() > 60 { 6 } else { 3 });
            if since.elapsed() >= duration {
                queue.pop_front();
                return;
            }
            let atlas = &self.gui.atlas;
            let lines = horizon_client::multiplayer::wrap(
                &alert.text,
                (batch.width - 20.).clamp(60., 370.),
                atlas,
            );
            let top = (batch.height - lines.len() as f32 * 10.) * 0.5;
            for (i, line) in lines.iter().enumerate() {
                let x = (batch.width - HudBatch::text_width(atlas, line)) * 0.5;
                batch.text(
                    atlas,
                    line,
                    x.round(),
                    top.round() + i as f32 * 10.,
                    rgba(255, 255, 85, 255),
                    true,
                );
            }
        });
    }
}
