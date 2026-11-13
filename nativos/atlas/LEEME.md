# Atlas (`atlas`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** A clone of Xaero's Minimap and World Map: a sharp minimap, a world map of everywhere you've been, and waypoints.

**Descripción del catálogo:** ![Atlas in the game](https://raw.githubusercontent.com/MrCloudy2/horizon-atlas/main/docs/images/hero.jpg)

Atlas is meant to be a close copy of Xaero's Minimap and Xaero's World Map for Horizon: the same look, the same keys and the same settings. The minimap, the world map and the waypoints share one map of the world, kept on disk per world, server and dimension.

- **Minimap**: drawn at your screen's resolution, turns with you, with entity dots that fade with height. Hold Tab for mob faces, player faces and item icons.
- **World map (M)**: drag and zoom. B makes a waypoint under the pointer.
- **Map modes**: Auto (the surface in the open, the cave around you under a roof), Cave and Layer.
-…

- Autores: MrCloudy2 · Licencia: MIT · Fuente: https://github.com/MrCloudy2/horizon-atlas
- Tipo: mixin (parchea el código del juego) · versión 0.2.0 · pide la revisión de Horizon f319701e8c
- Etiquetas: minimap, world-map, waypoints, map, radar, cave-map, navigation, hud, xaero

## Cómo funciona
Se engancha en 16 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_camera_pose_now`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_released_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_dragged_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_render_post`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: b, equal, f1, h, keypad.add, keypad.subtract, left.bracket, m, minus, n, right.bracket, t, u, y, z · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
