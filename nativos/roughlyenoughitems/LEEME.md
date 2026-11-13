# Roughly Enough Items (`roughlyenoughitems`)

**Estado:** no compila en Android: firma con 7 argumentos

**Qué hace:** REI for Horizon: REI's item list, search, recipe screen, favorites, craftable filter and "+" button, in REI's own look.

**Descripción del catálogo:** A port of [Roughly Enough Items (REI)](https://github.com/shedaniel/RoughlyEnoughItems) by shedaniel to Horizon, as a native mod. It draws REI's own interface, with REI's textures and layout, over the game's container screens.

- **Entry list** beside every container screen and the creative inventory, in pages (arrows, mouse wheel, a click on "1/N" for the first page). Enchanted books, potions, tipped arrows, spawn eggs and music discs are folded into one cell each; **Alt+click** unfolds them.
- **REI's search**, coloured as you type: words, `"phrases"`, `-exclusions`, `a | b`, `@mod`, `#tooltip`, `$tag`, `*id`, `r/regex/`. The field turns red when nothing matches; Up and Down recall earlier…

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/roughlyenoughitems
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: rei, recipes, item-list, recipe-viewer, search, favorites, port

## Cómo funciona
Se engancha en 17 puntos del juego:
- `hook:frame`
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
- `hook:tooltip`
- `horizon_client::game::native_hooks::Game<'_>::native_tooltip_render_pre`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: a, c, o, r, right.alt, right.control, right.shift, u, v, x · Ratón extra: key.mouse.left, key.mouse.middle, key.mouse.right

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
