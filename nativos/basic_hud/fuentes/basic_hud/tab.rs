// Enhanced Tab list renderer: numeric colored ping, own player highlight,
// player count top-line, background opacity override, optional ping bars,
// sorting by vanilla/name/ping, and editor preview with labeled samples.
use std::collections::HashSet;

use super::Game;
use crate::game::basic_hud as m;
use crate::game::basic_hud_i18n as i18n;
use horizon_client::hud::HudBatch;
use horizon_client::hud_layout::GuiDraw;
use horizon_client::scoreboard as sb;
use horizon_client::tab_list::{self, TabInputs, TabPlayer};
use horizon_client::text::Text;

/// Draws the enhanced Tab player list overlay or editor preview.
/// Returns true if drawing was handled, or false to fall through to stock rendering.
pub(super) fn draw(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    config: &m::Config,
    preview: bool,
) -> bool {
    if !preview && !config.tab.enabled {
        return false;
    }

    // 1. In active custom draw (in-game), pump skin downloads so faces and heads stay current.
    if !preview {
        let (ready, drawn) = {
            let mut downloads = game.skin_downloads.borrow_mut();
            (downloads.take_ready(), downloads.take_drawn())
        };
        game.skin_slots.next_frame(drawn);
        for (location, rgba) in ready {
            game.store_skin(location, rgba);
        }
    }

    // 2. Obtain players, online_mode, and upside-down identities.
    let (mut players, online_mode, upside_down) = if preview {
        sample_preview_players(config.language(&game.settings.language))
    } else {
        match game.remote_tab_players() {
            Some(remote) => remote,
            None => local_tab_players(game),
        }
    };

    // 3. In game, check visibility against key, singleplayer, and objectives.
    if !preview {
        let key = game.screen == crate::game::Screen::Playing
            && game.keys.contains(&winit::keyboard::KeyCode::Tab);
        let has_objective = game.score.board.display_objective(sb::SLOT_LIST).is_some();
        let shown = tab_list::visible(
            key,
            game.has_singleplayer_server(),
            players.len(),
            has_objective,
        );
        game.tab_list.set_visible(shown);
        if !shown {
            return true;
        }
    }

    let own_name = game.own_profile_name();
    let is_own = |p: &TabPlayer| -> bool {
        if preview {
            p.uuid == 1
        } else {
            p.name == own_name
        }
    };

    let total_count = players.len();
    apply_sorting(&mut players, game, config.tab.sort, &is_own);
    // Only the stock maximum of80 displayed entries needs extra components.
    players = tab_list::sorted_players(&players, &game.score.board).into_iter().cloned().collect();

    // 4. Update face skins for displayed players (live game only).
    if !preview {
        let faces: HashSet<u128> = tab_list::sorted_players(&players, &game.score.board)
            .iter()
            .map(|p| p.uuid)
            .collect();
        for p in players.iter_mut().filter(|p| faces.contains(&p.uuid)) {
            p.skin = game.face_skin(p.uuid, &p.skin);
        }
    }

    // 5. Compose component JSON for display_name:
    // Existing display_name or board.players_team.formatted_name_comp(&jsonname),
    // then append colored numeric latency and own marker.
    for p in &mut players {
        let own = is_own(p);
        let base_comp = match &p.display_name {
            Some(existing) => existing.clone(),
            None => {
                let name_val = serde_json::json!({ "text": &p.name });
                match game.score.board.players_team(&p.name) {
                    Some(team) => team.formatted_name_comp(&name_val),
                    None => name_val,
                }
            }
        };

        let mut extra = vec![base_comp];

        if config.tab.numeric_ping {
            let (ping_str, ping_color) = if own && game.has_singleplayer_server() {
                (format!(" {}", i18n::tr(config.language(&game.settings.language), "local")), "green")
            } else if p.latency < 0 {
                (" -".to_string(), "gray")
            } else {
                let color = if p.latency < 80 {
                    "green"
                } else if p.latency < 150 {
                    "yellow"
                } else {
                    "red"
                };
                (format!(" {}ms", p.latency), color)
            };
            extra.push(serde_json::json!({
                "text": ping_str,
                "color": ping_color,
            }));
        }

        if own && config.tab.highlight_self {
            extra.push(serde_json::json!({
                "text": " *",
                "color": "gold",
            }));
        }

        if extra.len() > 1 {
            p.display_name = Some(serde_json::json!({
                "text": "",
                "extra": extra,
            }));
        }
    }

    // 6. Sorting via temporary list_order values preserving spectator last (no mutation to network state).
    // 7. Player count top-line preserving server header.
    // A sample preview never changes the real list's visibility/heart animation.
    let mut sample_list = tab_list::TabList::default();
    if preview {
        sample_list.header = game.tab_list.header.clone();
        sample_list.footer = game.tab_list.footer.clone();
        sample_list.set_visible(true);
    }
    let list = if preview { &mut sample_list } else { &mut game.tab_list };
    let original_header = list.header.clone();
    if config.tab.show_count {
        let count_text = i18n::fmt(config.language(&game.settings.language), "Players: {0}", &[&total_count.to_string()]);
        let mut header = Text::literal(count_text);
        if let Some(h) = &original_header {
            header.push(Text::literal("\n"));
            header.push(h.clone());
        }
        list.header = Some(header);
    }

    // 8. Scale layout: virtual screen width for layout centering.
    let scale = if config.tab.scale.is_finite() {
        config.tab.scale.clamp(0.5, 1.5)
    } else {
        1.0
    };
    let virtual_width = if scale > 0.01 {
        (batch.width as f32 / scale).round() as i32
    } else {
        batch.width as i32
    };

    let opacity = config.tab.opacity.min(100);
    let bg_f64 = f64::from(opacity) / 100.0;
    let background = tab_list::row_background(true, bg_f64);

    let board = &game.score.board;
    let objective = board.display_objective(sb::SLOT_LIST);
    let up = |id: u128| upside_down.contains(&id);
    let inputs = TabInputs {
        players: &players,
        board,
        objective,
        screen_width: virtual_width,
        online_mode,
        gui_ticks: game.ticks as i64,
        background,
        upside_down: &up,
    };

    let mut draws = list.layout(&inputs, &game.gui.atlas.font);

    // Restore original header after layout calculation
    list.header = original_header;

    // 9. Opacity override on background fills
    let alpha = ((opacity as u32 * 255 + 50) / 100).min(255) as u8;
    for draw in &mut draws {
        if let GuiDraw::Fill { color, .. } = draw {
            if (*color & 0x00FF_FFFF) == 0 {
                *color = (u32::from(alpha) << 24) | (*color & 0x00FF_FFFF);
            }
        }
    }

    // 10. Optional ping bars: filter GuiDraw::Sprite name icon/ping*
    if !config.tab.ping_bars {
        draws.retain(|d| {
            if let GuiDraw::Sprite { sprite, .. } = d {
                !sprite.starts_with("icon/ping")
            } else {
                true
            }
        });
    }

    // 11. Paint and scale vertices using batch.scale_since
    let start = batch.vertices.len();
    horizon_client::hud_layout::paint(batch, &game.gui.atlas, &draws);
    if (scale - 1.0).abs() > 1e-4 {
        batch.scale_since(start, 0.0, 0.0, scale, scale);
    }

    true
}

fn local_tab_players(game: &Game<'_>) -> (Vec<TabPlayer>, bool, Vec<u128>) {
    let uuid = game.level.player_uuid.map_or(0, |u| {
        u.iter().fold(0u128, |a, &w| (a << 32) | u128::from(w as u32))
    });
    let mut me = TabPlayer::new(uuid, game.own_profile_name());
    me.game_mode = game.mode_id();
    me.skin = "entity/player/wide/steve".into();
    let guests = game.lan.as_ref().map_or(&[][..], |h| &h.hub.guests);
    let mut players = vec![me];
    players.extend(guests.iter().filter(|g| g.placed).map(|g| {
        let mut player = TabPlayer::new(g.uuid, &g.name);
        player.game_mode = g.game_mode;
        player.latency = g.latency.load(std::sync::atomic::Ordering::Relaxed);
        player
    }));
    let upside_down = players
        .iter()
        .filter(|p| p.name == "Dinnerbone" || p.name == "Grumm")
        .map(|p| p.uuid)
        .collect();
    (players, true, upside_down)
}

fn sample_preview_players(language: &str) -> (Vec<TabPlayer>, bool, Vec<u128>) {
    let marker = i18n::tr(language, "[Sample]");
    let samples = vec![
        (1u128, format!("{marker} You"), 0, 24),
        (2u128, format!("{marker} Alex"), 0, 68),
        (3u128, format!("{marker} Steve"), 0, 145),
        (4u128, format!("{marker} Ghost"), 3, -1),
        (5u128, format!("{marker} Guest"), 0, 320),
    ];
    let players = samples
        .into_iter()
        .map(|(uuid, name, mode, lat)| {
            let mut p = TabPlayer::new(uuid, name);
            p.game_mode = mode;
            p.latency = lat;
            p.skin = tab_list::default_skin(uuid).to_owned();
            p
        })
        .collect();
    (players, true, Vec::new())
}

fn apply_sorting(
    players: &mut [TabPlayer],
    game: &Game<'_>,
    sort_mode: u8,
    is_own: &impl Fn(&TabPlayer) -> bool,
) {
    match sort_mode {
        1 => {
            // Sort by name (case-insensitive), preserving spectator last
            let mut indices: Vec<usize> = (0..players.len()).collect();
            indices.sort_by(|&i, &j| {
                let a = &players[i];
                let b = &players[j];
                (a.game_mode == 3)
                    .cmp(&(b.game_mode == 3))
                    .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
            for (rank, &idx) in indices.iter().enumerate() {
                players[idx].list_order = -(rank as i32);
            }
        }
        2 => {
            // Sort by ping (lowest latency first), preserving spectator last
            let ping_key = |p: &TabPlayer| -> i32 {
                if is_own(p) && game.has_singleplayer_server() {
                    0
                } else if p.latency < 0 {
                    i32::MAX
                } else {
                    p.latency
                }
            };
            let mut indices: Vec<usize> = (0..players.len()).collect();
            indices.sort_by(|&i, &j| {
                let a = &players[i];
                let b = &players[j];
                (a.game_mode == 3)
                    .cmp(&(b.game_mode == 3))
                    .then_with(|| ping_key(a).cmp(&ping_key(b)))
                    .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
            });
            for (rank, &idx) in indices.iter().enumerate() {
                players[idx].list_order = -(rank as i32);
            }
        }
        _ => {
            // Vanilla order (0): keep stock list_order
        }
    }
}
