# Coordinates HUD (`coords_hud`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Your coordinates and facing in a corner of the screen.

**Descripción del catálogo:** Your block position and the direction you face in the top-left corner of the screen.

- `/where` puts your coordinates and dimension in chat.
- `/coords` hides or shows the box, and the game keeps your choice (also Options > Mods > Coordinates HUD > Settings).

It reads the player's position, draws on the HUD and writes to chat: nothing else.

Now a native mod (2.0.0): built into the game by the launcher, on the Horizon you play, like any native mod
(install it on the launcher's Mods page, then press Build there). The time of
day line is gone, since Horizon's hook points don't give the game's clock yet. Turn it off in Options > Mods: it
rests at once, and the next build leaves it out. Its so…

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/coords-hud
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 4baa571566
- Etiquetas: coordinates, hud, position

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
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
