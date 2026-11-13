# Flair (`flair`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::<Game<'_> as ApplicationHandler>::window_event'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Custom accessory mod

**Descripción del catálogo:** A cosmetics catalog for Horizon. Open it from the title screen or the pause menu and wear HD capes that move like cloth, animated bandanas with Japanese script, hats, wings, shoulder pets and auras with colour themes. Turn your player in a live preview, hide your armor, and add your own capes and Blockbench cosmetics with animations. Only you see them.

- Autores: _idom_ · Licencia: MIT · Fuente: https://github.com/umbx-dot/horizon-flair
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon 66f3981f52
- Etiquetas: cosmetics, capes, hats, wings, pets, blockbench

## Cómo funciona
Se engancha en 8 puntos del juego:
- `horizon_client::game::<Game<'_> as ApplicationHandler>::window_event`
- `horizon_client::game::Game<'_>::entity_geometry`
- `horizon_client::game::debug_keys::Game<'_>::draw_debug_overlay`
- `horizon_client::game::fishing::Game<'_>::fishing_line`
- `horizon_client::game::mob_blocks::Game<'_>::head_item_geometry`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client_lib::mobs::render::render_with`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
