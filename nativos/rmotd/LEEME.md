# RMotd (`rmotd`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Rotating server list MOTDs, managed in game with /randommotd and /motd.

**Descripción del catálogo:** A Horizon port of [adasjusk's RMotd](https://github.com/adasjusk/RMotd), the rotating MOTD plugin.

- The MOTD in the server list changes every 10 seconds, through the list in `config/rmotd.json` (the Fabric build's file).
- `/randommotd` turns rotation on and off.
- `/motd create <name> <motd>` adds an MOTD, `/motd reload` reads the file again (operator level 2).
- MOTDs take `&` colour codes.

It sets the MOTD of a world opened to LAN. The server list ping shows the rotation, the LAN broadcast keeps the first one. Horizon has no MOTD hook point, so a support module in `horizon_client::lan` writes the private MOTD field of the LAN server. Its injections are all hook points, but that field i…

- Autores: adasjusk, Vakarux12 · Licencia: CC-BY-SA-4.0 · Fuente: https://github.com/Vakarux12/RMotd/tree/main/horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon d91cab68de
- Etiquetas: motd, server-list, lan, commands, port

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
