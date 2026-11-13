# Held Item Info (`held_item_info`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Selected-item durability, count and enchantment names above the hotbar.

**Descripción del catálogo:** Minimal client-only selected-item text HUD for Horizon 26.3. Displays remaining/maximum durability, stack count and up to six translated enchantment names; long strings are shortened to GUI width. Original MIT implementation inspired by A5b84/held-item-info; no LGPL source was read or included. Potion details, layout options and fading are not implemented. Existing Armor Status covers equipped-item icons/durability; this mod focuses on selected-item text and enchantment names. See the source for validation and exact engine/mixin pins.

- Autores: AltMugen · Licencia: MIT · Fuente: https://github.com/AltMugen/horizon-held-item-info
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 3db53df3bd
- Etiquetas: hud, durability, enchantments, held-item

## Cómo funciona
Se engancha en 2 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_post`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
