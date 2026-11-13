# First-person Model (`firstpersonmodel`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** A port of First-person Model: see your own body in first person, with offsets for every pose and the original's settings.

**Descripción del catálogo:** A port of [First-person Model](https://github.com/tr7zw/FirstPersonModel) by tr7zw to Horizon, as a native mod. You see your own third-person body in first person.

- **Your body in first person**, head hidden, with your armour, cape, elytra, held items, parrots, arrows and fire.
- **Offsets for every pose**: standing, sneaking, sitting (boats, minecarts, mounts), crawling and swimming; the standing, sneaking and sitting offsets can be changed.
- **Vanilla Hands Mode** (Off, Always, Always Both Hands, Item In Hand) and **Dynamic Mode**; item lists that switch to vanilla hands or turn the mod off; hide any of the body's layers.
- **F6** turns it on and off.
- **The original's config screen** …

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/firstpersonmodel
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: first-person, player-model, camera, immersion, port

## Cómo funciona
Se engancha en 30 puntos del juego:
- `hook:frame`
- `hook:key`
- `hook:mods`
- `hook:render_hand`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_key_released_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `hook:settings`
- `hook:settings_click`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: a, c, f6, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
