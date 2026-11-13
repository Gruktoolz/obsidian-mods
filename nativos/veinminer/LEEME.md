# Veinminer (`veinminer`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::break_block'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Mine connected ores and matching trees using Horizon's native player-mining behavior.

**Descripción del catálogo:** An unofficial Rust port by levzzz5154. Mines connected veins, including diagonal neighbours,
through Horizon's player-mining code, preserving loot, XP, tool wear and server block events.
Includes per-player toggles, configurable block/tool groups, a block limit and cooldown.

Native source mod: install it on the launcher's Mods page, press Build there,
and confirm the native-code warning. A supported launcher downloads its own Rust toolchain
and builds a separate game using Horizon Mixins. Requires Git and Horizon's platform build
dependencies, and runs with your account's permissions.

Each version is pinned to one Horizon Nightly and Horizon Mixins revision, named in its
changelog; the lau…

- Autores: levzzz5154 · Licencia: MIT · Fuente: https://github.com/levzzz5154/horizon-veinminer
- Tipo: mixin (parchea el código del juego) · versión 0.3.2 · pide la revisión de Horizon d91cab68de
- Etiquetas: veinminer, mining, ores, trees, server-side, port

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::Game<'_>::break_block`
- `horizon_client::game::commands::Game<'_>::run_command`
- `horizon_client::game::lan_actions::Game<'_>::guest_command`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
