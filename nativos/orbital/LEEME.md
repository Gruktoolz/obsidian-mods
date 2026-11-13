# Orbital Laser (`orbital`)

**Estado:** no compila en Android: content_root con firma antigua

**Qué hace:** Fifteen orbital strikes, from a laser to a Planet Killer: a designator, a wheel, and craters that stay.

**Descripción del catálogo:** By Stink-O. Orbital Laser adds an Orbital Laser Designator. Aim it and hold right-click to call down one of fifteen orbital strikes (hold it and press R for the laser wheel, or type `/orbital wheel`), each with its own sky, sound and scar: the Orbital Laser, the Godly Laser, a Black Hole, the Meteor Storm, the Mothership, the Downfall, the Planet Killer, the Crystal Bloom, the Sinkhole, the Payload, the Nuke, the Rod from God, the Supercell, the Hurricane and the Tsunami. The wheel also sets your creative flying speed and the strike's approach, arrival and colours.

**Read before you fire**

- **Terrain damage is permanent.** A strike flattens and digs out the land, up to hundreds of blocks …

- Autores: Stink-O · Licencia: MIT · Fuente: https://github.com/Stink-O/orbital-laser
- Tipo: mixin (parchea el código del juego) · versión 0.5.10 · pide la revisión de Horizon e57b1d34bb
- Etiquetas: laser, explosions, destruction, shaders

## Cómo funciona
Se engancha en 23 puntos del juego:
- `hook:camera_shake`
- `hook:client_level_load`
- `hook:client_level_unload`
- `hook:command`
- `hook:frame`
- `hook:hud`
- `hook:key`
- `hook:level_tick_post`
- `hook:mods`
- `hook:net_from_client`
- `hook:net_from_server`
- `hook:perform_command`
- `hook:register_commands`
- `hook:render_needs`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: r · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
