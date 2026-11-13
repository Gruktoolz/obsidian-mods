# Create for Horizon (`create`)

**Estado:** no encaja: E_UNSUPPORTED_MACRO: no source function 'horizon_client::game::load_resources'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Build rotating machines and small factories in Horizon with gears, belts, presses and fans.

**Descripción del catálogo:** An unofficial, experimental Rust port of [Create](https://github.com/Creators-of-Create/Create), made by Delete228. It is not made or endorsed by the Create team; their original code is MIT and is credited in the mod's NOTICE.

- Drive machines with water wheels, hand cranks or a Creative Motor.
- Route power through shafts, gears, gearboxes, clutches and encased chain drives.
- Move items on basic belts and depots. Press sheets, mill ingredients, mix in a Basin or process items with a fan.

Install it as a native mod in Horizon Launcher and build the instance. It uses simple built-in models and standard Minecraft textures out of the box.

**The original models and textures require the origi…

- Autores: Delete228 · Licencia: MIT · Fuente: https://github.com/Delete228/create-horizon/commit/236c87a889cd76f7e663d1d182c28dc1145f7b14
- Tipo: mixin (parchea el código del juego) · versión 0.7.0-alpha.2 · pide la revisión de Horizon 9c7f9b2215
- Etiquetas: machines, automation, rotation, gears, belts, factory, port

## Cómo funciona
Se engancha en 69 puntos del juego:
- `hook:analog_output`
- `hook:block_changed`
- `hook:block_entity`
- `hook:block_entity_container`
- `hook:block_entity_render_bounding_box`
- `hook:block_entity_submit`
- `hook:block_entity_tick`
- `hook:block_entity_ticks`
- `hook:block_entity_update_tag`
- `hook:can_survive`
- `hook:client_level_unload`
- `hook:client_resources_reloaded`
- `hook:data_map_types`
- `hook:data_pack_files`
- …

## Qué necesita
- Datos que trae: data/content.json
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
