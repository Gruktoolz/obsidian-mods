# MouseTweaks (`mousetweaks`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::inventory_drag_move'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Mouse Tweaks from Java: right-drag to spread, left-drag to gather or quick-move, the wheel moves items one by one.

**Descripción del catálogo:** A port of [Mouse Tweaks](https://github.com/YaLTeR/MouseTweaks) by Ivan Molodetskikh (YaLTeR) to Horizon, as a native mod, with all seven of the original's options.

- **RMB drag**: hold the right button with a stack and drag over slots to drop one item into each.
- **LMB drag**: with an item on the cursor, drag to gather the same items; with **Shift**, quick-move every slot you pass.
- **LMB drag with an empty hand + Shift**: quick-move every slot you pass.
- **Wheel tweak**: the wheel over a slot moves items one at a time between your inventory and the container, crafting results included. Search order, scroll direction (normal, inverted, position-aware) and item scaling as in the original…

- Autores: DeportedToHell · Licencia: BSD-3-Clause · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/mousetweaks
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: inventory, mouse, drag, scroll, port

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
- `horizon_client::game::Game<'_>::inventory_drag_move`
- `horizon_client::game::Game<'_>::inventory_mouse_release`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
