# Movement Status (`movement_status`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Better toggle Sprint/Sneak display functionality

**Descripción del catálogo:** # Movement Status

A lightweight native HUD for Horizon that shows your current movement state in the corner of the screen.

It reads the live player state (sprinting, sneaking, flying, elytra gliding) and the physical vs. logical key state so it can correctly distinguish between a key that is being held and a key that is toggled via vanilla’s Toggle Sprint / Toggle Crouch options.

## Features

- Live movement status in the bottom-left corner:
  - `Sprint [Held]` / `Sprint [Toggled]`
  - `Sneak [Held]` / `Sneak [Toggled]`
  - `Flying [Creative]` / `Flying [Elytra]`
- Multiple states can be shown at once (sprint + sneak, etc.)
- Optional semi-transparent background box
- Respects vanilla Tog…

- Autores: Unidex101 · Licencia: MIT · Fuente: https://github.com/UniDex101/movement-status/tree/main
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: —

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
