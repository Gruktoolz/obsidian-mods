# Recipes (`recipes`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Every item beside your inventory, with its recipes and uses.

**Descripción del catálogo:** Horizon's own recipe viewer.

- Every item sits in a panel beside your inventory and container screens, with a search box: words in the name or the id, `@minecraft` for a namespace, `#logs` for a tag.
- R over an item (in the panel, in any slot, or in a recipe) shows its recipes, and U shows its uses. A click does the same: a left click for recipes, a right click for uses. Both keys can be rebound in Controls.
- Recipes have a tab for each station: crafting table, furnace, blast furnace, smoker, campfire, stonecutter, smithing table and brewing stand. Back returns through the pages you opened.
- + beside a crafting recipe you know fills the crafting grid, as the recipe book does.
- In creati…

- Autores: Horizon · Licencia: MIT · Fuente: https://github.com/horizon-rust/horizon-mods/tree/main/native/recipes
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 4baa571566
- Etiquetas: recipes, item-browser, recipe-viewer, search

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_panel_event`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: r, u · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
