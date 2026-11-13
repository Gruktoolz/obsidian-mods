# Mod Menu (`mod_menu`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** ModMenu For Horizon

**Descripción del catálogo:** A Mods button on the title screen, in the icon row after Accessibility, opening a screen laid out like the
Java [Mod Menu](https://modrinth.com/mod/modmenu)'s: every mod you have installed, with its icon, name, version and
description. A clean-room native mod for Horizon, with no Mod Menu code.

- **Search** by name, id or description, and **filter** by all, on, off, native or content mods.
- **Pick a mod** to see its version, type, description and any problems, turn it on or off, or open its own settings
  page.
- **Open Mods Folder** and **Done** at the bottom, as in the Java mod.
- The keyboard works too: type to search, arrows and Page Up/Down to pick a mod, Escape to leave.

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/ModMenuHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.3.5 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: mod-menu, mods-list, settings, ui, port

## Cómo funciona
Se engancha en 12 puntos del juego:
- `hook:frame`
- `hook:mods`
- `horizon_client::game::options::Game<'_>::draw_options`
- `horizon_client::game::options::Game<'_>::options_drag`
- `horizon_client::game::options::Game<'_>::options_key`
- `horizon_client::game::options::Game<'_>::options_mouse`
- `horizon_client::game::options::Game<'_>::options_scroll`
- `horizon_client::game::options::Game<'_>::options_shortcut`
- `horizon_client_lib::title::Title::buttons`
- `horizon_client_lib::title::Title::click`
- `horizon_client_lib::title::Title::draw_with_items`
- `horizon_client_render::gfx::reuse::On<'_>::create_shader_module`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
