# Mod Menu (`modmenu`)

**Estado:** no encaja: E_INVALID_BINDING: binding 'split' must be a target parameter or its explicit borrow/reborrow; no calls or locals

**Qué hace:** A port of TerraformersMC's Mod Menu: a Mods button on the title screen and pause menu opening Mod Menu's list of your mods.

**Descripción del catálogo:** A port of [Mod Menu](https://github.com/TerraformersMC/ModMenu) by Prospector and TerraformersMC to Horizon, as a native mod. It is Mod Menu's own code rewritten in Rust, with Mod Menu's layout, textures and translations.

- **A Mods button** on the title screen (a fourth icon beside Friends, Language and Accessibility, or below, beside or instead of Realms) and in the pause menu (a fifth icon, or a full-width button), and the mod count after the version: "Horizon 26.3 (Rust) (12 Mods)".
- **The mods screen**: the native mods built into your game, the content mods turned on, Minecraft and Horizon, each with its icon, version, summary and badges.
- **Details** of the selected mod: description…

- Autores: DeportedToHell · Licencia: MIT · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/modmenu
- Tipo: mixin (parchea el código del juego) · versión 1.1.2 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: modmenu, mods-list, ui, settings, port

## Cómo funciona
Se engancha en 21 puntos del juego:
- `hook:frame`
- `hook:key`
- `hook:mods`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_key_released_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `horizon_client::game::clipboard::Game<'_>::title_edit`
- `horizon_client::game::screens::Game<'_>::draw_pause_menu`
- `horizon_client::game::screens::Game<'_>::execute_pause_action`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: open_menu → unknown
- Teclas fijas en el código: c, v, x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
