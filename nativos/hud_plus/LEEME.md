# HUD+ (`hud_plus`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** One fully configurable HUD with a drag-and-drop editor and four views per module.

**Descripción del catálogo:** HUD+ is one configurable HUD for Horizon, with an in-game drag-and-drop layout editor.

Every module has four views: **Minimal**, **Compact**, **Standard** and **Full**. `Standard` follows the module's own switches; the other three are presets. A profile picks a view for all modules: Minimal gives Minimal, PvP gives Compact, the rest give Standard.

- **AppleSkin** — the original mod's look: golden saturation outlines over the hunger icons and the food/saturation a held item would restore flashing on the same bar. Minimal draws the outlines only, Compact adds food numbers, Full adds saturation, exhaustion, regeneration and the item preview.
- **Effects** — names, amplifiers, remaining time, …

- Autores: oxi-does-dev · Licencia: MIT · Fuente: https://github.com/oxi-does-dev/hud-plus
- Tipo: mixin (parchea el código del juego) · versión 4 · pide la revisión de Horizon f42ef42ef6
- Etiquetas: hud, ui

## Cómo funciona
Se engancha en 17 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_post`
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_released_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_dragged_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_render_post`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: b, f6, f7, h, j, k, m, n, o, p, r, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
