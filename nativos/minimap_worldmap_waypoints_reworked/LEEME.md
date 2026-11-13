# Minimap, Worldmap and waypoints reworked (`minimap_worldmap_waypoints_reworked`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::<Game<'_> as ApplicationHandler>::exiting'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** An adaptive minimap and world map with named waypoints, portals, world borders and a biome readout.

**Descripción del catálogo:** An adaptive minimap and a full world map for Horizon, with a shared in-memory terrain cache.

- Minimap in four corners, square or round, 64–256 GUI pixels, 0.1–4 blocks per pixel, and optional player rotation.
- Player arrow on both maps shows the viewing direction and accounts for minimap rotation.
- English, Russian, French and Spanish interfaces follow the game language, including regional variants.
- World map opened by a rebindable key (M by default); drag to pan and scroll to zoom under the cursor, with eased camera motion.
- Independent dimension selection for the minimap and the world map (Player / Overworld / Nether / End), without teleporting the player.
- Hover shows the pointed-…

- Autores: daniliammo · Licencia: GPL-3.0-only · Fuente: https://github.com/daniliammo/Minimap-Worldmap-and-waypoints-reworked
- Tipo: mixin (parchea el código del juego) · versión 0.4.1 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: minimap, map, world-map, waypoints, navigation, hud, portals, biome

## Cómo funciona
Se engancha en 19 puntos del juego:
- `horizon_client::game::<Game<'_> as ApplicationHandler>::exiting`
- `horizon_client::game::Game<'_>::forget_atlas_slots`
- `horizon_client::game::native_hooks::Game<'_>::native_client_disconnecting`
- `horizon_client::game::native_hooks::Game<'_>::native_client_player_network_logging_in`
- `horizon_client::game::native_hooks::Game<'_>::native_client_resources_reloaded`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_dragged_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: e, m, n · Ratón extra: key.mouse.left, key.mouse.middle, key.mouse.right

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
