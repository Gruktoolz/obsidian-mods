# Gyro (`gyro`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::apply_mouse_look'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Full 6DOF flight physics and immersive gyro camera tilt for Minecraft Horizon, inspired by Do a Barrel Roll.

**Descripción del catálogo:** **Fly an elytra like an aircraft.** Gyro swaps Horizon's locked horizon for full six-degree-of-freedom control: bank into a roll, pull through it to carve a turn, push the nose past vertical and loop. Inspired by the feel of the Java mod *Do a Barrel Roll*, written from scratch on Horizon's native hook points.

| While gliding | What it does |
|---|---|
| Mouse left / right | Roll — a full 360° with no snap-back |
| Mouse up / down | Pitch through your current bank, so a wing-down pull carves a turn |
| `A` / `D` | Rudder yaw for fine heading corrections |

In third person your avatar, elytra and held item stay pinned to the camera while the world, sky and entities roll around you. When you …

- Autores: CGRLabs · Licencia: MIT · Fuente: https://github.com/CGRLabs/gyro
- Tipo: mixin (parchea el código del juego) · versión 1.4.1 · pide la revisión de Horizon ba6344a59c
- Etiquetas: elytra, flight, barrel-roll, camera

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::Game<'_>::apply_mouse_look`
- `horizon_client::game::fov::Game<'_>::spinning_matrix`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::mobs::render::base_transform`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: a, d · Ratón extra: key.mouse.x, key.mouse.y

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
