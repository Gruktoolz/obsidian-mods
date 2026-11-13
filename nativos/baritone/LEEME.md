# Baritone (`baritone`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A Rust port of the Baritone pathfinder: #goto, #mine, #farm, #build, #explore, #follow, #elytra and the rest.

**Descripción del catálogo:** A port to Rust of Baritone 1.21.4 for Horizon, written from the Java source: the pathfinder (A* with walking, sprinting, parkour, bridging, pillaring, doors, ladders and water bucket landings), `#goto`, `#mine`, `#farm`, `#follow`, `#explore`, `#build` (schematics and shapes), `#tunnel`, `#elytra`, waypoints, and every setting and command of the Java mod. Type `#help` in chat. PARITY.md in the source lists every Java class and what became of it.

What it does not do: it draws the cached chunks semitransparent (`renderCachedChunks`) only if Horizon later offers a renderer for it. Like the Java mod it plays for you, so many multiplayer servers ban it: that choice is the player's.

For reviewer…

- Autores: luhnox (this port); Baritone by leijurv and the Baritone contributors · Licencia: LGPL-3.0-or-later (a derivative of Baritone, https://github.com/cabaletta/baritone) · Fuente: https://github.com/luhnox/horizon-baritone
- Tipo: trait (impl Hooks) · versión 1.1.52 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: pathfinding, automation, bot, mining, elytra, port

## Cómo funciona
Se engancha en 27 puntos del juego:
- `horizon_client::game::Game<'_>::sync_player_body`
- `horizon_client::game::Game<'_>::tick`
- `horizon_client::game::beacon::Game<'_>::beacon_geometry`
- `horizon_client::game::chat::Game<'_>::handle_chat_input`
- `horizon_client::game::control::Game<'_>::type_chars_into_screen`
- `horizon_client::game::fov::Game<'_>::tick_avatar`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::options::Game<'_>::draw_options`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
