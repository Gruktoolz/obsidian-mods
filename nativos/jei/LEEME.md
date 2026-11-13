# JEI for Horizon (`jei`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** An item and recipe viewer for Horizon: every item beside container screens, with search, recipes, uses, tooltips and bookmarks

**Descripción del catálogo:** An item and recipe viewer for [Horizon](https://playhorizon.dev), the Rust reimplementation of Minecraft: Java Edition.

Horizon JEI is inspired by [JustEnoughItems (JEI)](https://github.com/mezz/JustEnoughItems). JEI serves as the reference specification, while this project is a completely new Rust implementation built around Horizon's native hook points.

**Version 0.2.1 — Overlay and bookmarks**

### Features

- **Recipe Navigation:** Browse recipe history with **Back** and click recipe slots to open the corresponding item.
- **Recipe Placement:** Use the **+** button to place a recipe into the open crafting grid, provided it is available in the recipe book.
- **Item Giving:** Give items …

- Autores: enderliker · Licencia: MIT · Fuente: https://github.com/enderliker/horizon-jei
- Tipo: mixin (parchea el código del juego) · versión 0.2.1 · pide la revisión de Horizon 6560641ba5
- Etiquetas: recipes, item-browser, recipe-viewer, search, port

## Cómo funciona
Se engancha en 11 puntos del juego:
- `hook:frame`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_render_post`
- `hook:tooltip`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_panel_event`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: recipe → r; uses → u
- Teclas fijas en el código: a, r, u · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
