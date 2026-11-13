# Just Enough Items (`justenoughitems`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** JEI for Horizon: every item beside your inventory, JEI's search, recipes and uses, the "+" button, bookmarks and cheat mode.

**Descripción del catálogo:** A port of [Just Enough Items (JEI)](https://github.com/mezz/JustEnoughItems) by mezz to Horizon, as a native mod. It draws JEI's own interface, with JEI's sprites and layout, over the game's container screens.

- **Ingredient list** beside every container screen and the creative inventory: every item, enchanted books, potions and other variants included, in pages (mouse wheel or arrows).
- **JEI's search**: words (names and tooltips), `"phrases"`, `-exclusions`, `a | b`, `@mod`, `#tooltip`, `$tag`, `%creative tab`, `&id`.
- **Recipes and uses**: left click or **R** for how an item is made, right click or **U** for what it is used for — on the list, the bookmarks, a recipe's slots or any slot…

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/justenoughitems
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: jei, recipes, item-list, recipe-viewer, search, port

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
- Teclas fijas en el código: a, c, f, o, r, u, v, x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
