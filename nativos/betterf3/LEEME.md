# BetterF3 (`betterf3`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** A port of BetterF3: a customizable, colour-coded, human-readable debug screen (F3) with modules and animations.

**Descripción del catálogo:** A port of [BetterF3](https://github.com/TreyRuffy/BetterF3) by cominixo and TreyRuffy to Horizon, as a native mod. It replaces the debug screen (F3) with BetterF3's customizable, colour-coded, human-readable one.

- **Modules** in two columns, each in its own colours (Minecraft, FPS, Graphics, Server, Coordinates, Chunks, Location, Entities, Sounds, Help, System, Misc, Targeted), separators, and BetterF3's extra lines (velocity, slime chunk, days played, time, coloured FPS and memory).
- Every module and every line on or off, colours, left or right column, order; add and delete modules.
- Line backgrounds, shadow, font scale, the **sliding animation**; hide the boss bar and sidebar while F3 …

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/betterf3
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: f3, debug, hud, customization, port

## Cómo funciona
Se engancha en 16 puntos del juego:
- `hook:frame`
- `hook:gui_layer_pre`
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
- `horizon_client::game::debug_keys::Game<'_>::draw_debug_overlay`
- …

## Qué necesita
- Datos que trae: data/icon.png, data/lang/en_us.json, data/lang/ru_ru.json
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: c, f, m, v, x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
