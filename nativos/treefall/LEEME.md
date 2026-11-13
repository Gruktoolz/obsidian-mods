# TreeFall (`treefall`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::break_block'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Cut one log of a natural tree with an axe and the whole tree comes down.

**Descripción del catálogo:** **A native Horizon mod** (built into the game by the launcher; native mods are not sandboxed, so read the source before you build it).

Cut one log of a natural tree with an axe and the whole tree comes down. Every log is broken through the game's ordinary mining path, so drops, statistics, hunger and axe wear are exactly as if you had chopped each log yourself.

- Sneak to cut a single log; log cabins (logs without natural leaves) are never felled.
- **Settings page** (Options > Mods > TreeFall): which tools fell (axe / any tool / anything), sneaking, largest tree, leaves needed, cut time (as one log, or longer for bigger trees), tool wear (every log / once / a factor / none), keep durabili…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/treefall
- Tipo: mixin (parchea el código del juego) · versión 0.3.0 · pide la revisión de Horizon 142388e1c1
- Etiquetas: trees, tree-felling, axe, lumberjack

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::Game<'_>::break_block`
- `horizon_client::game::Game<'_>::pop_resource`
- `horizon_client::game::hand::Game<'_>::destroy_progress`
- `horizon_client::game::lan_actions::Game<'_>::guest_command`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
