# The Basic HUD (`basic_hud`)

**Estado:** no encaja: E_INVALID_BINDING: binding 'own' must be a target parameter or its explicit borrow/reborrow; no calls or locals

**Qué hace:** A visual HUD editor with 14 customizable widgets, recent pickups, effect timers, durability, coordinates, FPS and a customizable Tab list.

**Descripción del catálogo:** A client-side visual HUD for Horizon. Move, resize and style fourteen independent widgets directly over the game.

Includes effects and timers, consumable counts, durability, observed totem uses, available player health, item cooldowns, coordinates, FPS, ping, speed, CPS, world clock, mini armour and recent item pickups.

Recent pickups aggregates matching items and refreshes their expiry: 4 torches + 2 torches becomes x6. Five seconds by default, adjustable from 1 to 60 seconds. Includes icon + count mode.

Customize each widget's position, scale, colours, opacity, padding and shadows. Includes a customizable Tab list and ten interface languages. A short tutorial appears on the first settin…

- Autores: repositorykreml1n · Licencia: MIT · Fuente: https://github.com/repositorykreml1n/the-basic-hud
- Tipo: mixin (parchea el código del juego) · versión 0.5.0 · pide la revisión de Horizon 68ac6f647e
- Etiquetas: hud, hud-editor, widgets, coordinates, fps, status-effects, durability, tab-list

## Cómo funciona
Se engancha en 23 puntos del juego:
- `horizon_client::game::names::Game<'_>::name_tag_geometry`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_pickup_refused`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::options::Game<'_>::draw_options`
- `horizon_client::game::options::Game<'_>::options_drag`
- `horizon_client::game::options::Game<'_>::options_key`
- `horizon_client::game::options::Game<'_>::options_mouse`
- `horizon_client::game::options::Game<'_>::options_scroll`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: layout_editor → right.shift
- Teclas fijas en el código: right.shift · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
