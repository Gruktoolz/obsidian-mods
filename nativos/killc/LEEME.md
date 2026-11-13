# KillC (`killc`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** /killc, /killc <player> and /suicide, with an optional random respawn near (0, 0).

**Descripción del catálogo:** A Horizon port of [adasjusk's KillC](https://github.com/adasjusk/KillC), the kill command plugin for Paper, Folia, Velocity and Fabric.

- `/killc` and `/suicide` kill you.
- `/killc <player>` kills another player (operator level 2).
- After a death in the Overworld, players respawn at a random surface spot within `randomSpawnRadius` of (0, 0).
- Config: `config/killc.json` (`enabled`, `useRandomSpawn`, `randomSpawnRadius`), the same file as the Fabric build.

Vanilla keeps `/kill`. Runs where the world is hosted: single player and worlds opened to LAN (Horizon's dedicated server cannot build native mods yet). Native mods run with the player's account permissions and are not sandboxed.

Sour…

- Autores: adasjusk, Vakarux12 · Licencia: CC-BY-SA-4.0 · Fuente: https://github.com/Vakarux12/KillC/tree/main/horizon
- Tipo: mixin (parchea el código del juego) · versión 5.1.0 · pide la revisión de Horizon d91cab68de
- Etiquetas: commands, kill, respawn, random-spawn, port

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_perform_command`
- `horizon_client::game::native_hooks::Game<'_>::native_player_died`
- `horizon_client::game::native_hooks::Game<'_>::native_register_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_server_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
