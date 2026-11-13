# Chop! (`chop`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Cut one log and the whole tree tips over, crashes down and breaks apart, with soft vanilla sounds and particles.

**Descripción del catálogo:** Cut one log with an axe: the whole tree tips over like a real falling tree, crashes to the ground, and breaks apart from the base to the top in little pops. Everything is made of vanilla sounds and particles: no resource pack.

- **The log shakes while you cut it**, then pops when it gives way.
- **A physical fall**: the trunk drops onto the stump, creaks, tips over with gravity and stops where it meets the ground. Leaves in the way are crushed.
- **Falls in front of you**, within an adjustable angle, toward the clearest side.
- **A cute break-up** from the base to the top, with clouds, sparkles and hearts.
- **Replants a sapling** of the tree's kind (from its loot, or always).
- **Chain rea…

- Autores: Mayonesu0, Claude · Licencia: LicenseRef-Chop-1.0 · Fuente: https://github.com/Mayonesu0/chop
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon c9d474dd91
- Etiquetas: trees, tree-felling, axe, lumberjack, animation, server-side

## Cómo funciona
Se engancha en 11 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_destroy_block`
- `horizon_client::game::native_hooks::Game<'_>::native_left_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_level_tick_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_perform_command`
- `horizon_client::game::native_hooks::Game<'_>::native_place_block`
- `horizon_client::game::native_hooks::Game<'_>::native_player_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_register_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_right_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_server_stopping`
- `horizon_client::game::native_hooks::Game<'_>::native_server_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_world_lifecycle`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
