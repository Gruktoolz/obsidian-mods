# Waymap (`waymap`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Persistent client-side waypoints with 3D world markers, minimap dots, a native editor, and automatic death markers for Horizon.

**Descripción del catálogo:** Waymap brings complete waypoint management into the game itself. Instead of relying only on chat commands or external tools, it provides a native Horizon screen for creating, finding and editing waypoints, while keeping the existing /wp commands and minimap behaviour. Waypoints are stored locally per world or server, survive restarts, and never require server-side installation. Deaths are recorded automatically so the player can always find the place where they died.
Features
- World markers — 3D waypoints projected from the player's camera, hidden when outside the view
- Minimap markers — coloured dots at the correct map position, including rotated maps
- Native waypoint screen — list and e…

- Autores: Aron · Licencia: MIT · Fuente: https://github.com/Lauch2/horizon-waymap-mod
- Tipo: mixin (parchea el código del juego) · versión 0.2.1 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: waypoints, minimap, navigation, death-points, hud

## Cómo funciona
Se engancha en 14 puntos del juego:
- `horizon_client::game::extras::Game<'_>::update_minimap`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_render_post`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client_lib::extras_hud::draw_minimap`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: b, m, n, pagedown, pageup · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
