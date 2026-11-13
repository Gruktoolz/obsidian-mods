# Freecam (`freecam`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::mapped_key'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Freecam mod ported for Horizon

**Descripción del catálogo:** Control your camera separately from your player: a rewrite for Horizon, as a native mod, of the
[Freecam](https://github.com/MinecraftFreecam/Freecam) mod by the Freecam authors (MIT; its licence notice is kept in
the source).

- **F4** (rebindable in Options > Mods) detaches the camera: fly it through walls, at any speed, while your player
  stays where it was. The movement keys fly it and Left Ctrl speeds it up.
- **Tripod cameras**: F4 + 1 to 9 opens one of nine saved camera spots, which keeps its place when closed.

**Settings** (Options > Mods > Freecam): default or creative-style flight, horizontal and vertical speed (0.25x to
8x), where the camera starts, show your player, return on d…

- Autores: hakergeniusz · Licencia: MIT · Fuente: https://github.com/hakergeniusz/freecam
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon af55a73486
- Etiquetas: freecam, camera, spectator, tripod, port

## Cómo funciona
Se engancha en 15 puntos del juego:
- `horizon_client::game::Game<'_>::aim`
- `horizon_client::game::Game<'_>::mapped_key`
- `horizon_client::game::Game<'_>::play_mouse`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::perspective::Game<'_>::camera_view`
- `horizon_client::game::perspective::Game<'_>::cycle_camera`
- `horizon_client::game::perspective::Game<'_>::own_body_in_view`
- `horizon_client::game::perspective::Game<'_>::turn_player`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: 1, 2, 3, 4, 5, 6, 7, 8, 9, a, d, f4, s, w · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
