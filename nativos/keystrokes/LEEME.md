# Keystrokes (`keystrokes`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Your movement keys, mouse buttons and space bar in a corner, lit while held, with clicks per second.

**Descripción del catálogo:** Horizon's own keystrokes overlay, formerly an extra in Options > Horizon Extras.

- Forward, left, back and right as you bound them, the two mouse buttons and the space bar, lit while held.
- Clicks per second under each mouse button (can be turned off).
- Settings: Options > Mods > Keystrokes > Settings (corner; clicks per second).

It reads which keys you hold and your clicks, and draws on the HUD: nothing else.

Now a native mod (2.0.0): built into the game by the launcher, on the Horizon you play, like any native mod
(install it on the launcher's Mods page, then press Build there). Your settings carry over from 1.0.0. Turn it off in
Options > Mods: it rests at once, and the next build le…

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/keystrokes
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: keystrokes, hud, cps, pvp

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
