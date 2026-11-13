// Client data adapter for Basic HUD 0.2: effects, consumables, durability,
// totem pops, telemetry (coords, FPS, ping, speed, CPS, clock), cooldowns,
// and verified remote/LAN player health.
use std::cell::RefCell;
use std::time::{Duration, Instant, SystemTime};

use super::Game;
use crate::game::{
    basic_hud as m,
    basic_hud_i18n as i18n,
    basic_hud_logic as logic,
    basic_hud_players as players,
};
use horizon_client::{items::ItemStack, lang};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Icon {
    None,
    Effect(String),
    Item(ItemStack),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Row {
    pub icon: Icon,
    pub text: String,
    pub color: u32,
    pub value: String,
    pub sample: bool,
}

#[derive(Default)]
struct SpeedState {
    last_dim: &'static str,
    meter: logic::SpeedMeter,
    readout: f64,
    sampled: Option<Instant>,
}

#[derive(Clone, Debug)]
struct PingCache {
    last_sample: Option<Instant>,
    status: logic::PingStatus,
}

#[derive(Default)]
struct FpsCache {
    sampled: Option<Instant>,
    values: (u32, u32, u32),
}

thread_local! {
    static HEALTH_CACHE: RefCell<logic::HealthStore> = RefCell::new(logic::HealthStore::new());
    static SPEED_STATE: RefCell<SpeedState> = RefCell::new(SpeedState::default());
    static PING_CACHE: RefCell<PingCache> = RefCell::new(PingCache {
        last_sample: None,
        status: logic::PingStatus::Unknown,
    });
    static FPS_CACHE: RefCell<FpsCache> = RefCell::new(FpsCache::default());
}

fn fps_values(game: &Game<'_>) -> (u32, u32, u32) {
    FPS_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if cache.sampled.is_none_or(|at| at.elapsed() >= Duration::from_millis(250)) {
            cache.values = game.native_frame_rate();
            cache.sampled = Some(Instant::now());
        }
        cache.values
    })
}

fn speed_value() -> f64 {
    SPEED_STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.sampled.is_none_or(|at| at.elapsed() >= Duration::from_millis(250)) {
            state.readout = state.meter.value();
            state.sampled = Some(Instant::now());
        }
        state.readout
    })
}

pub(super) fn durability(game: &Game<'_>, stack: &ItemStack) -> Option<(u32, u32)> {
    let base = game.res.items().get(&stack.item).map_or(0, |item| item.max_damage);
    let max = match stack.extra.as_ref().and_then(|e| e.other("minecraft:max_damage")) {
        Some(horizon_world::nbt::Tag::Int(value)) => (*value).max(0) as u32,
        _ => base,
    };
    (max > 0).then_some((max.saturating_sub(stack.damage), max))
}

/// Reset all transient/sampled client data (on world leave, disconnect, or mod disabled).
pub(super) fn clear_all() {
    crate::game::basic_hud_pickup_data::clear();
    HEALTH_CACHE.with(|c| c.borrow_mut().clear());
    SPEED_STATE.with(|s| *s.borrow_mut() = SpeedState::default());
    PING_CACHE.with(|p| {
        *p.borrow_mut() = PingCache {
            last_sample: None,
            status: logic::PingStatus::Unknown,
        };
    });
}

/// Remove a single player/entity identity from the health cache when deceased or despawned.
pub(super) fn remove_entity(id: u32) {
    HEALTH_CACHE.with(|c| c.borrow_mut().remove(id));
}

/// Prune any entity IDs no longer present in confirmed player lists, verifying matching identity.
pub(super) fn cleanup_stale_entities(game: &Game<'_>) {
    let mut confirmed: Vec<(u32, String)> = game.remote_player_names();
    if let Some(host) = &game.lan {
        for (conn, model) in &host.models {
            if let Some(g) = host.hub.guest(*conn) {
                confirmed.push((model.id, g.name.clone()));
            }
        }
    }
    HEALTH_CACHE.with(|c| {
        c.borrow_mut().cleanup_stale(&confirmed);
    });
}

/// Sets health data observed from remote `SetEntityData` packets for confirmed players.
pub(super) fn observe_health(
    game: &Game<'_>,
    id: i32,
    values: &[(u8, i32, horizon_protocol::entity_data::Value)],
) {
    let entity_id = id as u32;
    // Find confirmed player name from server remote players or LAN guests
    let confirmed_name = game
        .remote_player_names()
        .into_iter()
        .find(|(pid, _)| *pid == entity_id)
        .map(|(_, name)| name)
        .or_else(|| {
            let host = game.lan.as_ref()?;
            for (conn, m) in &host.models {
                if m.id == entity_id {
                    return host.hub.guest(*conn).map(|g| g.name.clone());
                }
            }
            None
        });

    let Some(name) = confirmed_name else {
        return;
    };

    let mut new_hp = None;
    let mut new_abs = None;
    for (index, _, value) in values {
        match (*index, value) {
            (9, horizon_protocol::entity_data::Value::Float(hp)) => {
                if hp.is_finite() && *hp >= 0.0 {
                    new_hp = Some(*hp);
                }
            }
            (17, horizon_protocol::entity_data::Value::Float(abs)) => {
                if abs.is_finite() && *abs >= 0.0 {
                    new_abs = Some(*abs);
                }
            }
            _ => {}
        }
    }

    if new_hp.is_some() || new_abs.is_some() {
        HEALTH_CACHE.with(|c| {
            c.borrow_mut().observe(entity_id, &name, new_hp, new_abs);
        });
    }
}

/// Constructs a verified health snapshot for a confirmed player by inspecting
/// live model attributes (for finite positive max_health), actual LAN guest health,
/// and observed remote SetEntityData metadata.
fn get_player_snapshot(game: &Game<'_>, id: u32, confirmed_name: &str) -> Option<logic::HealthSnapshot> {
    let model = game.mobs.iter().find(|m| m.id == id);
    let mut max_hp = model.and_then(|m| {
        let max = m.attrs.max_health as f32;
        if max.is_finite() && max > 0.0 {
            Some(max)
        } else {
            None
        }
    });

    let mut guest_hp = None;
    let mut guest_abs = None;
    if let Some(host) = &game.lan {
        for (conn, m) in &host.models {
            if m.id == id {
                if let Some(g) = host.hub.guest(*conn) {
                    if g.health.is_finite() && g.health >= 0.0 {
                        guest_hp = Some(g.health);
                    }
                    let abs = g
                        .motion
                        .as_ref()
                        .map(|m| m.health.absorption())
                        .or(g.saved_absorption)
                        .filter(|a| a.is_finite() && *a >= 0.0);
                    if let Some(abs) = abs {
                        guest_abs = Some(abs);
                    }
                    if max_hp.is_none() {
                        let max = m.attrs.max_health as f32;
                        if max.is_finite() && max > 0.0 {
                            max_hp = Some(max);
                        }
                    }
                }
            }
        }
    }

    let observed = HEALTH_CACHE.with(|c| {
        c.borrow().get(id, Some(confirmed_name)).cloned()
    });

    let current_hp = guest_hp.or_else(|| observed.as_ref().and_then(|o| o.hp));
    let absorption = guest_abs
        .or_else(|| observed.as_ref().and_then(|o| o.absorption))
        .unwrap_or(0.0);

    let hp = current_hp?;
    logic::HealthSnapshot::new(confirmed_name, hp, max_hp, absorption)
}

/// Returns a formatted health string and color above player name tags if verified health is known.
pub(super) fn health_line(game: &Game<'_>, id: u32, name: &str) -> Option<(String, u32)> {
    let snapshot = get_player_snapshot(game, id, name)?;
    let color = logic::health_color(snapshot.hp, snapshot.max_hp);
    Some((logic::format_health_line(&snapshot), color))
}

fn targeted_player(game: &Game<'_>) -> Option<(u32, String)> {
    let mob_hit = game.mob_hit().filter(|(_, distance)| game.before_block(*distance));
    // Legacy in-frame hosts keep guest models separately from ordinary mobs.
    if let Some((conn, _)) = game.guest_hit().filter(|(_, d)| {
        game.before_block(*d) && !mob_hit.is_some_and(|(_, other)| other < *d)
    }) {
        let host = game.lan.as_ref()?;
        let model = host.models.get(&conn)?;
        return host.hub.guest(conn).map(|g| (model.id, g.name.clone()));
    }
    let (index, _) = mob_hit?;
    let model = game.mobs.get(index)?;
    game.remote_player_names().into_iter().find(|(id, _)| *id == model.id)
}

/// Per-frame telemetry updates: tracks smoothed horizontal speed in BPS and
/// samples own ping once per second to avoid costly skin texture updates.
pub(super) fn frame(game: &mut Game<'_>, elapsed: Duration) {
    if !game.native_on(m::ID) || !game.native_window() {
        clear_all();
        return;
    }
    let Some(player) = game.native_player() else {
        clear_all();
        return;
    };
    crate::game::basic_hud_pickup_data::frame(game);
    if !game.native_playing() {
        SPEED_STATE.with(|s| {
            let mut state = s.borrow_mut();
            state.meter.reset();
            state.sampled = None;
            state.readout = 0.0;
        });
        return;
    }

    let dt = elapsed.as_secs_f64();
    let (speed_enabled, ping_enabled) = m::config(|| game.native_setting(m::ID, "config"), |c| {
        (c.enabled(m::Widget::Speed), c.enabled(m::Widget::Ping))
    });
    let mut dim_changed = false;
    SPEED_STATE.with(|s| {
        let mut state = s.borrow_mut();
        if state.last_dim != player.dimension {
            state.last_dim = player.dimension;
            state.meter.reset();
            state.sampled = None;
            state.readout = 0.0;
            dim_changed = true;
            return;
        }
        if speed_enabled {
            state.meter.sample(player.x, player.z, dt);
        } else {
            state.meter.reset();
            state.sampled = None;
            state.readout = 0.0;
        }
    });

    if dim_changed {
        // Spawn packets can already have supplied fresh HP before this frame.
        // Keep only identities present in the current level rather than losing those updates.
        cleanup_stale_entities(game);
        PING_CACHE.with(|p| {
            *p.borrow_mut() = PingCache {
                last_sample: None,
                status: logic::PingStatus::Unknown,
            };
        });
        return;
    }

    // A single-player client also has a Remote connection to its own server.
    // Its internal scheduling latency is not a multiplayer/network ping.
    let local = game.has_singleplayer_server();
    if local || !ping_enabled {
        PING_CACHE.with(|p| {
            let mut cache = p.borrow_mut();
            cache.status = logic::classify_ping(local, None);
            cache.last_sample = None;
        });
        return;
    }

    // Remote ping sampling once per second, only while its widget is enabled.
    let should_sample = PING_CACHE.with(|p| {
        p.borrow().last_sample.map_or(true, |t| t.elapsed() >= Duration::from_secs(1))
    });

    if should_sample {
        let latency = if let Some((tab_players, _, _)) = game.remote_tab_players() {
            let own_name = game.own_profile_name();
            if let Some(own) = tab_players.iter().find(|p| p.name == own_name) {
                Some(own.latency)
            } else {
                None
            }
        } else {
            None
        };
        let status = logic::classify_ping(false, latency);

        PING_CACHE.with(|p| {
            *p.borrow_mut() = PingCache {
                last_sample: Some(Instant::now()),
                status,
            };
        });
    }
}

/// Collects row data for all 12 widgets. In active HUD (`preview == false`),
/// returns only genuine client data without placeholders or fake defaults.
/// In editor preview mode (`preview == true`), fills empty widgets with `[Example]` rows.
pub(super) fn collect(
    game: &mut Game<'_>,
    config: &m::Config,
    preview: bool,
    _ru: bool,
) -> Vec<(m::Widget, Vec<Row>)> {
    let language = config.language(&game.settings.language).to_owned();
    let lang = language.as_str();
    let mut out = Vec::with_capacity(m::WIDGETS.len());

    for widget in m::WIDGETS {
        if !preview && !config.enabled(widget) { continue; }
        let mut rows = Vec::new();

        match widget {
            m::Widget::Effects => {
                if config.enabled(m::Widget::Effects) && game.player.is_some() {
                    for e in &game.potions.effects.list {
                        if !config.shows_effect(e.id, e.ambient) {
                            continue;
                        }
                        let mut name = lang::get(&format!("effect.minecraft.{}", e.id));
                        if e.amplifier > 0 {
                            name = format!("{name} {}", lang::get(&format!("potion.potency.{}", e.amplifier)));
                        }
                        let text = format!("{name} {}", m::duration(e.duration));
                        let sprite = format!("mob_effect/{}", e.id.rsplit(':').next().unwrap_or(e.id));
                        rows.push(Row { sample: false,
                            icon: Icon::Effect(sprite),
                            text,
                            color: m::effect_colour(e.id, e.duration),
                            value: m::duration(e.duration),
                        });
                    }
                }
                if preview && rows.is_empty() {
                    let text = i18n::tr(lang, "[Example] Speed II 1:30").to_string();
                    rows.push(Row { sample: true,
                        icon: Icon::Effect("mob_effect/speed".to_string()),
                        value: "1:30".to_string(),
                        text,
                        color: m::GOOD,
                    });
                }
            }

            m::Widget::Items => {
                if config.enabled(m::Widget::Items) && game.player.is_some() {
                    let mut tally = m::Tally { rows: Vec::new() };
                    let carried = game.inventory.iter().take(36).flatten()
                        .chain(game.offhand.as_ref());
                    for s in carried {
                        let potion = s.item.ends_with("potion");
                        if potion
                            || [m::GOLDEN_APPLE, m::ENCHANTED_APPLE, m::TOTEM_ITEM, "minecraft:ender_pearl"]
                                .contains(&&*s.item)
                        {
                            let mut one = s.clone();
                            one.count = 1;
                            tally.add((one.item.clone(), one.potion().map(str::to_owned), one), s.count);
                        }
                    }
                    for ((_, _, stack), count) in tally.rows {
                        let name = game.stack_display_name(&stack).plain();
                        let text = format!("{} x{count}", name.trim_matches(['[', ']']));
                        rows.push(Row { sample: false,
                            icon: Icon::Item(stack),
                            value: m::icon_count(count),
                            text,
                            color: m::INK,
                        });
                    }
                }
                if preview && rows.is_empty() {
                    let sample = i18n::tr(lang, "Sample");
                    let pearl = ItemStack::new("minecraft:ender_pearl", 1);
                    let apple = ItemStack::new(m::GOLDEN_APPLE, 1);
                    let t1 = format!("[{sample}] {} x16", game.stack_display_name(&pearl).plain());
                    let t2 = format!("[{sample}] {} x8", game.stack_display_name(&apple).plain());
                    rows.push(Row { sample: true,
                        icon: Icon::Item(ItemStack::new("minecraft:ender_pearl", 16)),
                        value: "×16".to_string(),
                        text: t1,
                        color: m::INK,
                    });
                    rows.push(Row { sample: true,
                        icon: Icon::Item(ItemStack::new(m::GOLDEN_APPLE, 8)),
                        value: "×8".to_string(),
                        text: t2,
                        color: m::INK,
                    });
                }
            }

            m::Widget::Durability => {
                if config.enabled(m::Widget::Durability) && game.player.is_some() {
                    let worn = game.armor.iter().flatten();
                    let held = game.held().into_iter().chain(game.offhand.as_ref());
                    for s in worn.chain(held) {
                        if let Some((left, max)) = durability(game, s) {
                            rows.push(Row { sample: false,
                                icon: Icon::Item(s.clone()),
                                text: m::durability_text(left, max),
                                color: m::durability_colour(left, max),
                                value: m::durability_text(left, max),
                            });
                        }
                    }
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] 1561/1561").to_string();
                    rows.push(Row { sample: true,
                        icon: Icon::Item(ItemStack::new("minecraft:diamond_sword", 1)),
                        value: "1561/1561".to_string(),
                        text: t,
                        color: m::GOOD,
                    });
                }
            }

            m::Widget::Totems => {
                if config.enabled(m::Widget::Totems) && game.player.is_some() {
                    let own_name = game.own_profile_name();
                    let own_count = m::pops(|p| p.get(&own_name));
                    if own_count > 0 {
                        let label = i18n::tr(lang, m::POPPED);
                        rows.push(Row { sample: false,
                            icon: Icon::Item(ItemStack::new(m::TOTEM_ITEM, 1)),
                            text: format!("{label}: {own_count}"),
                            value: own_count.to_string(),
                            color: m::TOTEM,
                        });
                    }
                    for other in players::others(game) {
                        let count = m::pops(|p| p.get(&other.name));
                        if count > 0 {
                            rows.push(Row { sample: false,
                                icon: Icon::Item(ItemStack::new(m::TOTEM_ITEM, 1)),
                                text: format!("{}: {count}", other.name),
                                value: count.to_string(),
                                color: m::TOTEM,
                            });
                        }
                    }
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Totems used: 1").to_string();
                    rows.push(Row { sample: true, value: "1".to_owned(),
                        icon: Icon::Item(ItemStack::new(m::TOTEM_ITEM, 1)),
                        text: t,
                        color: m::TOTEM,
                    });
                }
            }

            m::Widget::Health => {
                if config.enabled(m::Widget::Health) && game.player.is_some() {
                    if let Some((mob_id, name)) = targeted_player(game) {
                        let eye = game.player.as_ref().map_or(glam::DVec3::ZERO, |p| {
                            glam::DVec3::new(p.position.x, p.position.y + p.dimensions.eye_height as f64, p.position.z)
                        });
                        let shown_tags = game.shown_name_tags(eye);
                        let expected_name = game.score.board.players_team(&name)
                            .map(|t| t.formatted_name(horizon_client::text::Text::literal(&name)).resolve_legacy().plain())
                            .unwrap_or_else(|| name.clone());
                        if shown_tags.iter().any(|t| t == &expected_name) {
                            if let Some(snapshot) = get_player_snapshot(game, mob_id, &name) {
                                rows.push(Row { sample: false, value: String::new(),
                                    icon: Icon::None,
                                    text: format!("{name}: {}", logic::format_health_line(&snapshot)),
                                    color: logic::health_color(snapshot.hp, snapshot.max_hp),
                                });
                            } else {
                                let unknown = i18n::tr(lang, "Health unknown");
                                rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text: format!("{name}: {unknown}"), color: m::WARNING });
                            }
                        }
                    }
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Steve: 20/20 HP").to_string();
                    rows.push(Row { sample: true, value: String::new(),
                        icon: Icon::None,
                        text: t,
                        color: m::GOOD,
                    });
                }
            }

            m::Widget::Cooldowns => {
                if config.enabled(m::Widget::Cooldowns) && game.player.is_some() {
                    let mut entries: Vec<_> = game.cooldowns.__horizon_mixins_basic_hud_cooldown_entries().iter().collect();
                    entries.sort_by(|(a, _), (b, _)| a.cmp(b));
                    let tick = *game.cooldowns.__horizon_mixins_basic_hud_cooldown_tick();

                    for (group, &(start, end)) in entries {
                        let Some((remaining, secs)) = logic::cooldown_remaining(start, end, tick) else {
                            continue;
                        };

                        let mut stack_opt = None;
                        let mut display_name = None;

                        let carried = game.inventory.iter().take(36).flatten()
                            .chain(game.offhand.as_ref())
                            .chain(game.held());
                        for s in carried {
                            let default = game.res.items().get(&s.item).and_then(|i| i.use_cooldown.clone());
                            let g = horizon_client::cooldowns::group(s, default.as_ref());
                            if g.as_ref() == group.as_ref() {
                                display_name = Some(game.stack_display_name(s).plain());
                                stack_opt = Some(s.clone());
                                break;
                            }
                        }

                        // Known item groups retain their name/icon after the last item was used.
                        if stack_opt.is_none() && game.res.items().get(group.as_ref()).is_some() {
                            let stack = ItemStack::new(group.as_ref(), 1);
                            display_name = Some(game.stack_display_name(&stack).plain());
                            stack_opt = Some(stack);
                        }

                        let name_str = display_name.unwrap_or_else(|| {
                            group.strip_prefix("minecraft:").unwrap_or(group).to_string()
                        });
                        let text = logic::format_cooldown(&name_str, secs);
                        let icon = stack_opt.map(Icon::Item).unwrap_or(Icon::None);
                        let color = logic::cooldown_color(remaining);

                        rows.push(Row { sample: false, value: format!("{secs:.1}s"), icon, text, color });
                    }
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Ender Pearl: 8.5s").to_string();
                    rows.push(Row { sample: true, value: "8.5s".to_owned(),
                        icon: Icon::Item(ItemStack::new("minecraft:ender_pearl", 1)),
                        text: t,
                        color: m::INK,
                    });
                }
            }

            m::Widget::Coordinates => {
                if config.enabled(m::Widget::Coordinates) {
                    if let Some(player) = game.native_player() {
                        let facing = i18n::tr(lang, logic::facing_from_yaw(player.yaw, false));
                        let dim = i18n::tr(lang, logic::format_dimension(player.dimension, false));
                        rows.push(Row { sample: false, value: String::new(),
                            icon: Icon::None,
                            text: format!("XYZ: {:.1} / {:.1} / {:.1}", player.x, player.y, player.z),
                            color: m::INK,
                        });
                        rows.push(Row { sample: false, value: String::new(),
                            icon: Icon::None,
                            text: format!("{dim} ({facing})"),
                            color: config.widget(m::Widget::Coordinates).accent,
                        });
                    }
                }
                if preview && rows.is_empty() {
                    let sample = i18n::tr(lang, "Sample");
                    let t1 = format!("[{sample}] XYZ: 120.5 / 64.0 / -250.3");
                    let t2 = format!("[{sample}] {} ({})", i18n::tr(lang, "Overworld"), i18n::tr(lang, "North (-Z)"));
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t1, color: m::INK });
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t2, color: config.widget(m::Widget::Coordinates).accent });
                }
            }

            m::Widget::Fps => {
                if config.enabled(m::Widget::Fps) {
                    let (fps, min_fps, low_fps) = fps_values(game);
                    let style = config.widget(m::Widget::Fps);
                    let text = logic::format_fps(fps, min_fps, low_fps, config.fps_label, config.fps_lows, config.fps_auto, style.scale, style.compact);
                    let color = if fps >= 60 {
                        m::GOOD
                    } else if fps >= 30 {
                        m::WARNING
                    } else {
                        m::BAD
                    };
                    rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text, color });
                }
                if preview && rows.is_empty() {
                    let style = config.widget(m::Widget::Fps);
                    let t = format!("[{}] {}", i18n::tr(lang, "Example"), logic::format_fps(144, 95, 82, config.fps_label, config.fps_lows, config.fps_auto, style.scale, style.compact));
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t, color: m::GOOD });
                }
            }

            m::Widget::Ping => {
                if config.enabled(m::Widget::Ping) && game.player.is_some() {
                    let status = PING_CACHE.with(|p| p.borrow().status);
                    let (_, color) = logic::format_ping(status, false);
                    let text = match status {
                        logic::PingStatus::Local => i18n::tr(lang, "Ping: local").to_owned(),
                        logic::PingStatus::Unknown => i18n::tr(lang, "Ping: unknown").to_owned(),
                        logic::PingStatus::Latency(ms) => format!("{}: {ms} {}", i18n::tr(lang, "Ping"), i18n::tr(lang, "ms")),
                    };
                    rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text, color });
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Ping: 35 ms").to_string();
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t, color: m::GOOD });
                }
            }

            m::Widget::Speed => {
                if config.enabled(m::Widget::Speed) && game.player.is_some() {
                    let bps = speed_value();
                    let text = format!("{}: {bps:.1} {}", i18n::tr(lang, "Speed"), i18n::tr(lang, "bps"));
                    rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text, color: m::INK });
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Speed: 5.60 bps").to_string();
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t, color: m::INK });
                }
            }

            m::Widget::Cps => {
                if config.enabled(m::Widget::Cps) {
                    let (lmb, rmb) = game.native_clicks();
                    rows.push(Row { sample: false, value: String::new(),
                        icon: Icon::None,
                        text: format!("CPS: {lmb} | {rmb}"),
                        color: m::INK,
                    });
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] CPS: 8 | 12").to_string();
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t, color: m::INK });
                }
            }

            m::Widget::Pickups => {
                rows = crate::game::basic_hud_pickup_data::rows(game, config, preview);
            }
            m::Widget::ArmorMini => {} // Item models and bars share a separate renderer/hitbox.
            m::Widget::Clock => {
                if config.enabled(m::Widget::Clock) {
                    if game.player.is_some() {
                        let (hours, minutes) = logic::clock_time_from_ticks(game.world.day_time);
                        let text = logic::format_clock(hours, minutes, false).replacen("Time", i18n::tr(lang, "Time"), 1);
                        rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text, color: m::INK });
                    } else {
                        let secs = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0);
                        let text = logic::format_utc_clock(secs);
                        rows.push(Row { sample: false, value: String::new(), icon: Icon::None, text, color: m::INK });
                    }
                }
                if preview && rows.is_empty() {
                    let t = i18n::tr(lang, "[Example] Time: 14:30").to_string();
                    rows.push(Row { sample: true, value: String::new(), icon: Icon::None, text: t, color: m::INK });
                }
            }
        }

        out.push((widget, rows));
    }

    out
}
