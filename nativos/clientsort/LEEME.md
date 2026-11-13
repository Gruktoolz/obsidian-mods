# ClientSort (`clientsort`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Inventory sorting from ClientSort: Sort, Restock and Transfer buttons, middle-click sort, Shift/Ctrl/Alt for other orders.

**Descripción del catálogo:** A port of [ClientSort](https://github.com/TerminalMC/ClientSort) by TerminalMC to Horizon, as a native mod.

- **Trigger buttons** beside each inventory, as in the original: **Sort**, **Restock** (moves the items the other inventory already holds) and **Transfer** (moves as much as fits).
- **Middle click** over an inventory sorts it.
- Sort order: the creative inventory's order by default; hold **Shift** for quantity, **Ctrl** for name (A-Z), **Alt** for item ID.
- The container, the main inventory and the hotbar are sorted on their own.
- Keys for every operation, including **Fill Stacks**, in Options > Mods > ClientSort.
- Settings: buttons on/off, default order, click speed, hotbar as it…

- Autores: DeportedToHell · Licencia: Apache-2.0 AND LGPL-3.0-or-later · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/clientsort
- Tipo: mixin (parchea el código del juego) · versión 1.2.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: inventory, sorting, buttons, port

## Cómo funciona
Se engancha en 17 puntos del juego:
- `hook:frame`
- `hook:key`
- `hook:mods`
- `hook:mouse_button_pre`
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
- Teclas fijas en el código: c, v, x · Ratón extra: key.mouse.4, key.mouse.left, key.mouse.middle, key.mouse.right

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
