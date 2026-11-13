# Armor Status (`armor_status`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Displays equipped armor & held items with authentic Java durability colors.

**Descripción del catálogo:** An ArmorStatusHUD-style overlay for Horizon, as a native mod: your armor and the items in your hands, with
their durability, on the HUD.

- **All four armor slots** (helmet, chestplate, leggings, boots) plus the main hand and the off hand.
- **Durability colours** as in Java Edition: green above 50%, yellow from 25%, orange from 10%, red below 10%.
- **Stack counts** for held items that don't wear out (food, blocks, torches).

**Settings** (Options > Mods > Armor Status): position (bottom right by default; hotbar left or right, top left or
right, bottom left), durability as the remaining value, a percentage or a fraction, which hand items to show (both,
main hand, off hand or armor only), an…

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/armor-status
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: hud, armor, durability, inventory-hud

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
