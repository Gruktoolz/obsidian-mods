# I'm Fast (`imfast`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Removes Minecraft's server-side player and elytra movement limits.

**Descripción del catálogo:** A port of [I'm Fast](https://github.com/Gabriel-Hiss/Im-Fast) by Gabriel Hiss for
Horizon: it lifts the server-side movement checks that pull a player back for
"moved too quickly".

**Implemented so far**

- `player_movement_check` and `elytra_movement_check` are switched off once per
  open world, so long jumps and fast elytra flight are not refused.
- Verified with a real LAN guest: a 200-block move is kept while the rules are
  off and refused when they are back on.

**Not yet ported**

- The two "moved wrongly" thresholds (player and vehicle) and the vehicle speed
  limit: Horizon has no rule for them, and the hooks that could patch them are
  not stamped yet.

Original copyright and lic…

- Autores: Gabriel Hiss, AdLf0208 · Licencia: MIT · Fuente: https://github.com/AdLf0208/imfast
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 1d63e0a5a3
- Etiquetas: port, movement, server, elytra

## Cómo funciona
Se engancha en 1 puntos del juego:
- `hook:server_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
