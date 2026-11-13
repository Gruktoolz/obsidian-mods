# Light Overlay (`light_overlay`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** 1:1 Light Overlay & MiniHUD (F7): visualizes mob-spawnable blocks with 3D Yellow (night spawn) and Red (always/cave spawn) crosses (X) or 3D light level numbers

**Descripción del catálogo:** A 1:1 native Rust port of Light Overlay and MiniHUD's F7 Light Level Overlay for Horizon. Press F7 during gameplay to scan nearby spawnable block surfaces and render depth-tested 3D markers directly on block tops. Red markers indicate blocks where hostile mobs can spawn at any time (caves/indoor darkness), Yellow markers indicate blocks where hostile mobs can spawn at night, and optional Green markers show safe blocks. Supports Crosses (X), 3D Numbers (0-15), or Both, with configurable scan radius (8 to 20 blocks) and Modern (== 0) or Legacy (<= 7) spawn thresholds.

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/light-overlay
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: light-level, mob-spawning, overlay, minihud, port

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::fishing::Game<'_>::fishing_line`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: f7 · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
