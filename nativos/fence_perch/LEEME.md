# Fence Perch (`fence_perch`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Jump onto fences, walls and closed gates from a standstill, stand on top, step off.

**Descripción del catálogo:** Jump onto fences, walls and closed fence gates from a standstill, stand on top, and step off either side.

### Features
- Walk up to a fence, wall or closed fence gate and jump while stepping forward: you land on top, no run-up needed.
- Stand on it, walk along a connected line, then step off on either side.
- Gates stay closed, so animals stay in the pen while you climb in and out.

### How it works
For the player only, supported barriers collide like a one-block-high block instead of one and a half. Your jump is not stronger: a 1.5-block ledge stays out of reach and low ceilings still block you. On top of a barrier you stand half a block lower than on a full post. Animals, mobs, open gates…

- Autores: AyanoKaito · Licencia: MIT · Fuente: https://github.com/AyanoKaito/horizon-fence-perch
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: fences, movement, parkour, farming

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client_lib::lan::hub::Hub::collide`
- `horizon_client_lib::motion::<MotionWorld<'_,'_,'_> as EntityMoveWorld>::collide`
- `horizon_client_lib::motion::<MotionWorld<'_,'_,'_> as EntityMoveWorld>::set_on_ground_with_movement`
- `horizon_client_lib::motion::MotionWorld<'_,'_,'_>::no_player_collision`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
