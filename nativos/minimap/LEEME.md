# Minimap (`minimap`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A map of the world around you in a corner of the screen.

**Descripción del catálogo:** Horizon's own minimap, formerly an extra in Options > Horizon Extras.

- A map of the land around you, drawn like a map item: vanilla's map colours with their height shading and water depth, and your arrow.
- Underground, under a roof and in the Nether it maps the floor around your height.
- Settings: Options > Mods > Minimap > Settings (corner; size; square or round; zoom, 0.5 to 4 blocks per pixel; turn with the player).

It shows a map in a corner of the screen: nothing else.

Now a native mod (2.0.0): built into the game by the launcher, on the Horizon you play, like any native mod
(install it on the launcher's Mods page, then press Build there). Your settings carry over from 1.0.0. Turn…

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/minimap
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: minimap, map, navigation, hud

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_minimap`
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
