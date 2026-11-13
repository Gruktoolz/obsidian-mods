# World Map (`worldmap`)

**Estado:** no encaja: E_UNSUPPORTED_CONTROL_FLOW: calls beneath a reference may change temporary lifetimes

**Qué hace:** A full screen, self-writing world map with a minimap, waypoints (groups, import and export), day and night, cave layers and teleport.

**Descripción del catálogo:** A full screen map that writes itself as you explore, for Horizon, with a minimap in the game.

- **Self-writing map.** The terrain you have seen stays on the map after you leave, per world, dimension and cave layer, in the vanilla map colours, by day and by night (glowing blocks stay lit).
- **Smooth.** Drag with a glide, scroll to zoom towards the pointer; no flicker when zooming, light on big maps.
- **Minimap** that can turn with you, with the compass letters, waypoints and other players, and the floor at your head height in caves.
- **Cave layers.** Under a roof, in a cave or in the Nether the map shows the floor round you; layered or full, with a Top Y slider.
- **Waypoints.** Right-cli…

- Autores: Px228-Da-Da · Licencia: MIT · Fuente: https://github.com/Px228-Da-Da/horizon-worldmap
- Tipo: mixin (parchea el código del juego) · versión 0.7.1 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: map, worldmap, waypoints, navigation, caves, minimap

## Cómo funciona
Se engancha en 17 puntos del juego:
- `horizon_client::game::chat::Game<'_>::chat_tree_for`
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
- Teclas reasignables: open → m
- Teclas fijas en el código: a, b, c, d, e, equal, g, h, j, keypad.add, keypad.subtract, l, m, minus, n, page_down, page_up, s, t, w, z · Ratón extra: key.mouse.5

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
