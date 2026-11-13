# Waypoints (`waypoints`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Place waypoints with N, manage them with M, and see their name and distance in the world.

**Descripción del catálogo:** Waypoints for Horizon.

- **N**: create a waypoint where you stand: name, X/Y/Z, dimension, colour, visible or hidden. Enter saves, Tab moves between fields, Esc cancels.
- **M**: the waypoint list, filtered by All / Overworld / Nether / End. Each row shows its colour, name, coordinates, dimension and distance, with Hide/Show, Edit and Delete buttons.
- **In the world**: each visible waypoint in your current dimension shows its name, its distance and a marker in its colour.
- **When you die**: a red `Death 1`, `Death 2`... waypoint marks the block you died on. The newest 5 stay, and the oldest goes when you die again; rename one to keep it for good.
- **Options > Mods > Waypoints**: rebind b…

- Autores: Nirokx · Licencia: MIT · Fuente: https://github.com/Nirokx/horizon-waypoints
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: waypoints, navigation, markers, hud

## Cómo funciona
Se engancha en 11 puntos del juego:
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


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: f1, m, n · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
