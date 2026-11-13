# Zoom (`zoom`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Hold a key to zoom in; the mouse wheel zooms further while you hold it.

**Descripción del catálogo:** Horizon's own zoom, formerly an extra in Options > Horizon Extras.

- Hold the key (Z by default) to zoom in at the starting zoom; let go and the view comes back.
- While you hold it, the mouse wheel zooms further in or out (up to 10x) and the hotbar stays put.
- Smooth zoom eases in and out like the spyglass; slower turning keeps aiming steady.
- Settings: Options > Mods > Zoom > Settings (the key; starting zoom; smooth zoom; slower turning).

It hears its own key and the mouse wheel while you hold it, and zooms the camera: nothing else.

Now a native mod (2.0.0): built into the game by the launcher, on the Horizon you play, like any native mod
(install it on the launcher's Mods page, then …

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/zoom
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: zoom, camera, spyglass

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- `horizon_client::game::native_hooks::Game<'_>::native_zoom`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: z · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
