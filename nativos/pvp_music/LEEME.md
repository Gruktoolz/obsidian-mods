# PVP Music (`pvp_music`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Battle music during combat: fades, death fades and the phased two-half track switch, the original's behavior kept.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to the Horizon tree this was built against.

- Mutual-combat detection: attacks on and by players and mobs are remembered for 5 s; a mob we hit that then hits us starts a mob battle.
- A landed hit on a player starts a player battle; explosion and bad-respawn-point hurts pull in combat within 12 blocks; a wither within 30 blocks starts a fight.
- The battle track fades in and out, death fades run at the configured multiplier, and the phased two-half musi…

- Autores: sparticle62ops · Licencia: CC0-1.0 · Fuente: https://github.com/Sparticle62ops/horizon-pvp-music
- Tipo: mixin (parchea el código del juego) · versión 1.2.0 · pide la revisión de Horizon 3fb4c3138e
- Etiquetas: combat, music, audio, qol

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_music`
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
