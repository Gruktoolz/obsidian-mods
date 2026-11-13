# Stuck? (`stuck`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** /stuck and /escape move you to a safe spot nearby, off the Nether roof too.

**Descripción del catálogo:** A Horizon port of [adasjusk's Stuck?](https://github.com/adasjusk/Stuck), the unstuck plugin for Spigot, Paper, Folia and Fabric.

- `/stuck` and `/escape` search random columns near you for solid ground with two blocks of air above, and teleport you there.
- Skips lava, fire, magma, cactus and other hazards, and leaves and snow.
- A player on the Nether's bedrock roof is brought back under it.
- Cooldown between uses, 30 seconds by default.
- Config: `config/stuck.properties`, the same keys as the Fabric build.

Only loaded chunks are searched. Runs where the world is hosted: single player and worlds opened to LAN (Horizon's dedicated server cannot build native mods yet). Native mods run wi…

- Autores: adasjusk, Vakarux12 · Licencia: CC-BY-SA-4.0 · Fuente: https://github.com/Vakarux12/Stuck/tree/main/horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon d91cab68de
- Etiquetas: commands, teleport, unstuck, nether-roof, port

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_perform_command`
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
