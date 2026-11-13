# Big Nukes (`big_nukes_native`)

**Estado:** no compila en Android: content_root con firma antigua

**Qué hace:** Nuclear bombs whose shockwave digs real craters and leaves fallout. Launch missiles, build a superbomb, survive in a bunker.

**Descripción del catálogo:** **One file, no separate content mod.** Install it and press Build: its textures, models, sounds, text and recipes come inside it.

- **Make** the bombs: mine uranium below y=16, enrich it in a centrifuge, and put the parts together on the assembly table.
- **Blast** with a demolition charge (radius 16), a tactical warhead (50) or an atomic bomb (250). The flash, the mushroom cloud and the ring of the shockwave are seen from far beyond your render distance, and the wave leaves a crater, ash and fires behind it.
- **Set them off** by fuse, by a keypad's code, or by a remote from any distance. Fire a missile from a silo at coordinates up to 4096 blocks away, or call a bomber onto a strike beaco…

- Autores: bioneisme · Licencia: MIT · Fuente: https://github.com/Bioneisme/big-nukes
- Tipo: mixin (parchea el código del juego) · versión 0.1.3 · pide la revisión de Horizon 6560641ba5
- Etiquetas: nuclear, bombs, explosions, radiation, missile, bunker

## Cómo funciona
Se engancha en 52 puntos del juego:
- `horizon_client::game::mods::content_root`
- `horizon_client::game::native_hooks::Game<'_>::native_apply_effect_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_container`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_ticks`
- `horizon_client::game::native_hooks::Game<'_>::native_blocks`
- `horizon_client::game::native_hooks::Game<'_>::native_camera_pose`
- `horizon_client::game::native_hooks::Game<'_>::native_camera_shake`
- `horizon_client::game::native_hooks::Game<'_>::native_chunk_load`
- `horizon_client::game::native_hooks::Game<'_>::native_chunk_unload`
- `horizon_client::game::native_hooks::Game<'_>::native_client_level_unload`
- `horizon_client::game::native_hooks::Game<'_>::native_component_types`
- `horizon_client::game::native_hooks::Game<'_>::native_consume_effect`
- …

## Qué necesita
- Datos que trae: data/big_nukes-content.hmod
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: 1, e, f, f1, f11, f2, f3, q · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
