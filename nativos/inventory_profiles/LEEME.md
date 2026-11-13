# Inventory Profiles (`inventory_profiles`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** R-key & UI sorting (Category, A-Z, Count, ID), Auto-Refill depleted stacks, Tool Break Protection, and Quick-Stack to chests.

**Descripción del catálogo:** A 1:1 native Rust port of Inventory Profiles Next & Inventory Sorter for Horizon. Press R or click the UI sort buttons to merge partial stacks and sort chests or your inventory using 4 selectable modes: Category (Creative Tab), Alphabetical (A-Z), Stack Count (High-Low), or Item ID. Includes automatic hand stack refilling when placing blocks or consuming items, Tool Break Protection that swaps out low-durability tools before they break, and G-key Quick-Stacking to move matching items into open containers.

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/inventory-profiles
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: inventory, sorting, auto-refill, quick-stack, port

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::tweaks::Game<'_>::sort_slots`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: g, r · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
