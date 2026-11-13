# WorldEdit (`worldedit`)

**Estado:** no encaja: E_MODULE: module destination must be an indexed Rust source file

**Qué hace:** working port of java world edit

**Descripción del catálogo:** The WorldEdit editor as a native mod for Horizon: select a region and fill, replace, shape, copy and paste
it with `//` commands, and load or save schematics. It writes blocks through the game's own code, with no command
length limit, in single player and on a LAN world you host or join.

- **Select**: `//wand` gives a wooden axe; left click and right click set position 1 and 2 (or `//pos1`, `//pos2`).
- **Edit**: `//set`, `//replace`, `//walls`, `//outline`, `//hollow`, `//overlay`, `//sphere`, `//cyl`,
  `//pyramid`, `//line`, `//copy`, `//cut`, `//paste`, `//move`, `//expand`, `//contract`, `//shift`, `//count`.
- **Patterns**: one block or a weighted mix, such as `//set 70%stone,30%grave…

- Autores: _cykoo_ · Licencia: MIT · Fuente: https://github.com/XnotCykoX/WorldEdit-Horizon
- Tipo: mixin (parchea el código del juego) · versión 1.2.1 · pide la revisión de Horizon 83f15fa276
- Etiquetas: worldedit, building, schematics, editing, commands, port

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::chat::ChatScreen::update_command_info`
- `horizon_client::game::Game<'_>::key_pressed`
- `horizon_client::game::Game<'_>::open_world`
- `horizon_client::game::Game<'_>::play_mouse`
- `horizon_client::game::chat::Game<'_>::handle_chat_input`
- `horizon_client::game::commands::Game<'_>::run_command`
- `horizon_client::game::lan_actions::Game<'_>::guest_command`
- `horizon_client::game::screens::Game<'_>::draw_pause_menu`
- `horizon_client::game::screens::Game<'_>::pause_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
