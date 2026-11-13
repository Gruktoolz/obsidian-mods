# Shulker Box Tooltip (`shulkerboxtooltip`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::slot_at'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** A port of ShulkerBoxTooltip: the contents of shulker boxes, chests, the ender chest and bundles right in their tooltip.

**Descripción del catálogo:** A port of [ShulkerBoxTooltip](https://github.com/MisterPeModder/ShulkerBoxTooltip) by MisterPeModder to Horizon, as a native mod: the contents of a container item right in its tooltip, with ShulkerBoxTooltip's own window texture, layout and colours.

- **Shift**: compact preview, equal items merged (1.2k, 45k…); **Alt+Shift**: full preview, every slot in place; **Ctrl**: lock the tooltip and hover the items inside it for their own tooltips.
- Every **shulker box** (tinted with its dye), chests, copper and trapped chests, barrels, furnaces, droppers, dispensers, hoppers, brewing stands, chiseled bookshelves, decorated pots, shelves, lecterns and any item holding contents.
- The **ender chest*…

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/shulkerboxtooltip
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: shulker, tooltip, inventory, preview, port

## Cómo funciona
Se engancha en 23 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:net_from_client`
- `hook:net_from_server`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_key_released_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `hook:server_tick`
- `hook:settings`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: c, v, x, z · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
