# Smooth Swapping (`smoothswapping`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::draw_inventory_slots'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** A port of Smooth Swapping: items fly from slot to slot instead of jumping, along an animation curve you can edit.

**Descripción del catálogo:** A port of [Smooth Swapping](https://github.com/Schauweg/Smooth-Swapping) by Schauweg and Riflusso to Horizon, as a native mod.

- Items moved in an inventory **fly from slot to slot** instead of jumping: shift-clicks, hotbar keys, sorting, crafting results and stacks taken into the cursor.
- The flight follows the original's **animation curve** at the original's speed.
- **Smooth Swapping's own config screen**, as in Java (Mod Menu's Configure, or Options > Mods > Smooth Swapping > Configure...): Toggle Mod, the speed slider, the curve editor and the test inventory.
- Drawing only: it works on servers too.

Built for Horizon Nightly 2026-11-07 (`2ebf00a`). Native mods are compiled into the g…

- Autores: DeportedToHell · Licencia: LGPL-3.0 · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/smoothswapping
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: inventory, animation, ui, port

## Cómo funciona
Se engancha en 16 puntos del juego:
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
- `horizon_client::game::Game<'_>::draw_inventory_slots`
- `horizon_client::game::Game<'_>::draw_slot_item`
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
