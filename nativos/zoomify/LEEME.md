# Zoomify (`zoomify`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A port of Zoomify: a zoom key with smooth transitions, scroll zoom, a secondary zoom and Zoomify's own settings screen.

**Descripción del catálogo:** A port of [Zoomify](https://github.com/isXander/Zoomify) by isXander to Horizon, as a native mod.

- **Zoom** (C, hold or toggle) with Zoomify's transitions, times and initial zoom; **scroll zoom** with the mouse wheel while zooming; the **secondary zoom** (F6), slow and cinematic, that hides the HUD.
- Relative sensitivity, the cinematic camera while zooming, the hand's field of view and the **spyglass** rules (combine, override, only while holding or carrying it; its overlay and sounds).
- **Zoomify's own config screen**, as in Java (YACL): Mod Menu's Configure button, Options > Mods > Zoomify > Configure..., or `/zoomify`. Tabs, collapsible groups, sliders and reset buttons, the descripti…

- Autores: DeportedToHell · Licencia: LGPL-3.0-or-later · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/zoomify
- Tipo: mixin (parchea el código del juego) · versión 1.1.1 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: zoom, camera, spyglass, port

## Cómo funciona
Se engancha en 23 puntos del juego:
- `hook:command`
- `hook:commands`
- `hook:compute_fov_modifier`
- `hook:frame`
- `hook:gui_layer_pre`
- `hook:key`
- `hook:mods`
- `hook:mouse_scrolling`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_key_released_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- …

## Qué necesita
- Datos que trae: data/demo/third-person-hud.webp, data/demo/third-person-view.webp, data/demo/zoom-hand.webp, data/demo/zoom-world.webp, data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: c, f6, v, x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
