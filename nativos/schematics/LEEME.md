# Schematics (`schematics`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Load schematics in every common format, see them as ghost blocks in the world, check what is missing, paste them and save areas.

**Descripción del catálogo:** Schematics for Horizon.

Reads `.litematic`, `.schem` (Sponge versions 1 to 3), `.schematic` (numeric ids of 1.12 and before), `.nbt`, `.mcstructure` (Bedrock, translated on a best-effort basis) and `.bp`. Put files in `schematics/` in the game folder.

- **M**: the menu: load a file, then move, turn, mirror or paste the placement.
- **In the world**: missing blocks show as see-through ghosts, wrong blocks red, wrong states orange, extra blocks magenta.
- **Materials**: what it takes, what is missing and what you carry.
- **Save area**: a stick's left and right clicks set two corners; save them as `.litematic`, `.schem` or `.nbt`.
- **Page Up / Page Down**: one layer at a time.
- **/schemati…

- Autores: Nirokx · Licencia: MIT · Fuente: https://github.com/Nirokx/horizon-schematics
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: schematics, building, litematic, schem, ghost-blocks, material-list

## Cómo funciona
Se engancha en 18 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_client_resources_reloaded`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_level_tick_post`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_mouse_button_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_mouse_scrolling`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: m · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
