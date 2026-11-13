# Better Dynamic Lights (`dynamiclights`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Horizon's built-in dynamic lights, with more: glowing creatures, projectiles and TNT shine too, water puts out fire, a switch for every light.

**Descripción del catálogo:** **Horizon's dynamic lights, taken further.** Horizon already has built-in dynamic lights (Options > Horizon Extras > Dynamic lights, off by default): items in your hands, dropped items and burning mobs light up the world, with one on/off switch. Better Dynamic Lights uses the same light renderer and takes the place of that switch while it is on, with much more.

| | Built into Horizon | Better Dynamic Lights |
|---|---|---|
| Held items (yours, other players', mobs') and dropped items | ✅ | ✅ + armor stands |
| Burning mobs and items | ✅ | ✅ + burning arrows and tridents |
| Glowing creatures: blazes, magma cubes, glow squids, allays, vexes | — | ✅ |
| Projectiles: fireballs, wither skulls, …

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/betterdynamiclights
- Tipo: mixin (parchea el código del juego) · versión 1.0.3 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: lighting, dynamic-lights, torch, visual, better-dynamic-lights

## Cómo funciona
Se engancha en 15 puntos del juego:
- `hook:frame`
- `hook:key`
- `hook:mods`
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
- `horizon_client::game::dynamic_lights::Game<'_>::gather_dynamic_lights`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: key.mouse.left, key.mouse.middle, key.mouse.right

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
