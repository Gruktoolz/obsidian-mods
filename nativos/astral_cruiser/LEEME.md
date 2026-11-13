# Astral Cruiser (`astral_cruiser`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::terrain_distance'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** A huge pilotable starship with a walkable interior, lasers, dash and live portals between dimensions.

**Descripción del catálogo:** A huge starship you fly yourself, with a large walkable interior that moves with it.

- **Flight:** WASD with the ship following your view, and F5 views including the ship's own.
- **Weapons:** turrets, nose guns and two beams.
- **Travel:** coordinate dash and jump. Long dashes show only a small strip of far terrain around the ship.
- **Portals:** a portal shows the real destination dimension live, and you cross it aboard or on foot.
- **Start:** `/astral spawn`, then `/astral board`. N opens the control panel; keys are in Options > Mods > Settings.

**What it does as a native mod.** It is compiled into the game with no sandbox. It changes the game's rendering, ticking and chunk loading for…

- Autores: repositorykreml1n · Licencia: MIT · Fuente: https://github.com/repositorykreml1n/astral-cruiser
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 68ac6f647e
- Etiquetas: starship, vehicle, flying, portals, weapons, dimensions

## Cómo funciona
Se engancha en 23 puntos del juego:
- `horizon_client::game::Game<'_>::prepare_lod`
- `horizon_client::game::Game<'_>::rain_height`
- `horizon_client::game::Game<'_>::terrain_distance`
- `horizon_client::game::entity_culling::Game<'_>::view_scale`
- `horizon_client::game::ghasts::Game<'_>::player_entity_collisions`
- `horizon_client::game::lan_vehicles::Game<'_>::guest_vehicle_tick`
- `horizon_client::game::levels::Game<'_>::levels_to_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: 1, 2, 3, 4, b, h, n, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
