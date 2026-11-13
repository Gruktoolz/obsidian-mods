# FPS Counter (`fps_overlay`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Frames per second in a corner of the screen, with the minimum and the 1% low.

**Descripción del catálogo:** Horizon's own FPS counter, formerly an extra in Options > Horizon Extras.

- Frames in the last second, in a small box in the corner you pick.
- Optionally the slowest frame and the 1% low of the last 5 seconds.
- Settings: Options > Mods > FPS Counter > Settings (corner; minimum and 1% low).

It reads the game's own frame counter and draws on the HUD: nothing else.

Now a native mod (2.0.0): built into the game by the launcher, on the Horizon you play, like any native mod
(install it on the launcher's Mods page, then press Build there). Your settings carry over from 1.0.0. Turn it off in
Options > Mods: it rests at once, and the next build leaves it out. Its source is
[horizon-mods/native/f…

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/fps-overlay
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: fps, hud, performance, frame-time

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
