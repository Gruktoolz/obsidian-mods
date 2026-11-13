# SmartF3 (`smartf3`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** A tidier, configurable F3 debug screen: the same information in modules you can colour, reorder and turn off.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

The F3 debug screen, grouped into 14 modules: FPS, server, coordinates, chunk, facing, world, biome, light, chart keys, system, rendering, and the targeted block, fluid and entity.

- Each module: on/off, label and value colours, left or right column, order.
- Text shadow, background (vanilla, dark, none), gap between modules, compact layout.
- The vanilla F3 is one click (or `/smartf3 off`) away; the F3 ch…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/smartf3
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 142388e1c1
- Etiquetas: f3, debug-screen, hud, betterf3

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::Game<'_>::hud`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
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
