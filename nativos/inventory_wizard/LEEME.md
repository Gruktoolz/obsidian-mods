# InventoryWizard (`inventory_wizard`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** InventoryWizard's one-click sorting as a native mod for Horizon

**Descripción del catálogo:** ## Features

- **Smart sorting** by category (stone, earth, wood, ores, blocks, tools, weapons, armor, food, redstone,
  transportation, decoration), from item tags and components, so other mods' items are sorted too
- **Survival-friendly hotbar**: weapons, tools, food, building blocks, utility items, then the rest
- **Stacks merged**, but enchanted or named items are never combined
- **Three modes**: Default, Alphabetical and Stack-based
- **Sort buttons** by the inventory and container titles, plus a mode button (it shows the mode's letter)
- **Locked slots**: **Alt+Click** a hotbar or inventory slot to lock it, and sorting leaves it alone
- **Keys**: **R** (or **Shift+Right Click**) sorts…

- Autores: VanishingTacos · Licencia: MIT · Fuente: https://github.com/VanishingTacos/InventoryWizard-Horizon
- Tipo: mixin (parchea el código del juego) · versión 2.3.3 · pide la revisión de Horizon f99f8feaaa
- Etiquetas: inventory, sorting, port

## Cómo funciona
Se engancha en 11 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::tweaks::Game<'_>::draw_sort_buttons`
- `horizon_client::game::tweaks::Game<'_>::sort_key_press`
- `horizon_client::game::tweaks::Game<'_>::sort_mouse_press`
- `horizon_client::game::tweaks::Game<'_>::sort_press`
- `horizon_client::game::tweaks::Game<'_>::sort_slots`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: r · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
