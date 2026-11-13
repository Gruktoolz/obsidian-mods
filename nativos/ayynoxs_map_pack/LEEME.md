# ayynoxs map pack (`ayynoxs_map_pack`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Minimap and full-screen world map with waypoints, cave layers and opt-in map sharing between players.

**Descripción del catálogo:** A **minimap** and a **full-screen world map** for Horizon, written from scratch as a native mod. Inspired by the well-known Java map mods, but no code or graphics are taken from them.

## ayynoxs minimap
- Built on Horizon's own minimap renderer, with coordinates and facing direction below it
- Square or round, north-up or rotating, 4 sizes, zoom with `I` / `O`
- **Waypoints:** `B` opens a window for the name, `Backspace` removes the one you aim at. They show on the minimap, on the world map and in the world with their distance in metres.
- Automatic death point
- Chat commands: `/wp add|del|rename|share|sync|list|clear|help`

## ayynoxs world map
- Full-screen map on `M`: drag or WASD to pa…

- Autores: DenoxyYT (Ayynox) · Licencia: MIT · Fuente: https://github.com/DenoxyYT/ayynoxs-map-pack
- Tipo: mixin (parchea el código del juego) · versión 0.4.2 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: minimap, world, map, waypoints, multiplayer

## Cómo funciona
Se engancha en 23 puntos del juego:
- `horizon_client::game::advancements_screen::Game<'_>::advancements_drag`
- `horizon_client::game::advancements_screen::Game<'_>::advancements_mouse`
- `horizon_client::game::advancements_screen::Game<'_>::advancements_scroll`
- `horizon_client::game::advancements_screen::Game<'_>::close_advancements`
- `horizon_client::game::advancements_screen::Game<'_>::draw_advancements`
- `horizon_client::game::extras::Game<'_>::extras_connecting`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_minimap`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_net_from_client`
- `horizon_client::game::native_hooks::Game<'_>::native_net_from_server`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: a, b, d, equal, i, m, minus, o, s, w · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
