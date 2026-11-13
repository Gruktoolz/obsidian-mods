# PhysicsModHorizon (`horizon_physics`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::attack'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Physics for Horizon, inspired by PhysicsMod: ragdolls, waves, physical particles and capes.

**Descripción del catálogo:** By tariksabanuz

Physics for Horizon, inspired by PhysicsMod by Haubna. It is a clean-room version for Horizon: it contains none of PhysicsMod's code or assets and is not made or endorsed by Haubna. Everything is built as a native Horizon mod, so no other mods or loaders are needed.

Features

- **Realistic water**: waves that never repeat, built from several swells with wave groups, wind gusts, sun glints, foam on the crests and along the shore, and rain rings on the surface. Boats and the camera follow the same waves you see.
- **Ragdolls**: mobs fall and tumble with their bodies instead of just disappearing.
- **Physical particles**: block cracks, sprint dust and broken or eaten items bec…

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/PhysicsModHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.20.7 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: physics, ragdolls, water, waves, particles, capes

## Cómo funciona
Se engancha en 40 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client::game::Game<'_>::attack`
- `horizon_client::game::Game<'_>::break_block`
- `horizon_client::game::Game<'_>::crack_particle`
- `horizon_client::game::Game<'_>::destroy_particles`
- `horizon_client::game::boats::Game<'_>::boat_geometry`
- `horizon_client::game::client_ambient::Game<'_>::sprint_dust`
- `horizon_client::game::end::Game<'_>::end_geometry`
- `horizon_client::game::fishing::Game<'_>::fishing_geometry`
- `horizon_client::game::fishing::Game<'_>::fishing_line`
- `horizon_client::game::item_draw::Game<'_>::dropped_item_geometry`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
