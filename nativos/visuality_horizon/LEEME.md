# Visuality (`visuality_horizon`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::remote::Game<'_>::remove_entity'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Small pixel particles: hit fragments, slime blobs, creeper sparks, glints, soul wisps and rain ripples.

**Descripción del catálogo:** An original client-side Rust implementation of the effects of PinkGoosik's
[Visuality](https://github.com/PinkGoosik/visuality) (MIT), with its own art made in Blender: small pixel particles.

- **Hit fragments**: bone chips from skeletons, strays and skeleton horses, dark chips from wither skeletons,
  feathers in each chicken variant's colours, emerald chips from villagers.
- **Slime blobs**: chunky blobs in three sizes when a slime lands, in a colour you choose (magma cubes are left
  alone).
- **Sparks and glints**: charged creeper sparks, amethyst sparkles on full clusters, armor glints on gold and
  diamond armor, ore glints on uncovered faces only.
- **Soul wisps** over soul sand and …

- Autores: BenjiSyS · Licencia: MIT · Fuente: https://github.com/BenjiSyS/visuality-horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.2 · pide la revisión de Horizon c9d474dd91
- Etiquetas: particles, effects, visual, port

## Cómo funciona
Se engancha en 14 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:render_needs`
- `hook:settings`
- `hook:settings_click`
- `hook:tick`
- `hook:world_drawn`
- `horizon_client::animate_tick::block_animate_tick`
- `horizon_client::game::remote::Game<'_>::enter_remote_dimension`
- `horizon_client::game::remote::Game<'_>::handle_packet`
- `horizon_client::game::remote::Game<'_>::leave_server`
- `horizon_client::game::remote::Game<'_>::remove_entity`
- `horizon_client::game::remote::Game<'_>::spawn_entity`
- `horizon_client::mobs::cube::client_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
