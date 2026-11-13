# Toggle Sprint (`toggle_sprint`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** 1:1 Better Sprinting & Lunar ToggleSprint HUD: toggle sprint and toggle sneak with [Sprinting (Toggled)] / [Key Held] / [Flying] status HUD and configurable Cre

**Descripción del catálogo:** A 1:1 native Rust port of Better Sprinting and the Lunar/Badlion ToggleSprint HUD for Horizon. Allows toggling continuous sprint (V or vanilla Sprint key) and sneak (C) without holding keys down, while automatically pausing toggled sneak inside inventory and chest screens so Shift-clicking items works normally. Renders the classic movement status HUD ([Sprinting (Toggled)], [Sprinting (Key Held)], [Sneaking (Toggled)], [Sneaking (Key Held)], [Flying (2.0x Boost)], [Swimming], [Gliding (Elytra)], [Riding]) and includes a configurable Creative Fly Speed Boost multiplier (1.0x to 4.0x).

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/toggle-sprint
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: toggle-sprint, toggle-sneak, hud, movement, pvp

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: c, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
