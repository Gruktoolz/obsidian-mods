# Kotost Bust (`kotost_body`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::cosmetics::Game<'_>::cosmetics_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** A bust drawn from your own skin that bounces only when you move, an hourglass figure, chestplates in its shape.

**Descripción del catálogo:** - **A bust for your own player**, drawn by the mod and textured from your own skin: no cosmetics item, no resource pack, nothing on the server.
- **It moves only when you move**: a small spring bounce on each step (more when running) and on jumps and landings. Standing still, it stays still.
- **An hourglass figure**: a torso that narrows to the waist and widens to the hips, your own skin stretched over it.
- **Chestplates keep the figure**: a vanilla chestplate is drawn in the same shape, with cups that bounce with the bust.
- **Who gets it is up to you**: `config/kotost-body.txt` picks you only (default), a list of names, slim skins, everyone or nobody.

Client side only. Works alone or wi…

- Autores: Kotost Games · Licencia: MIT · Fuente: https://github.com/Leks2000/kotost-body
- Tipo: mixin (parchea el código del juego) · versión 0.4.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: cosmetic, skin, player-model, physics, figure

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::cosmetics::Game<'_>::cosmetics_geometry`
- `horizon_client::game::cosmetics::Game<'_>::cosmetics_model`
- `horizon_client::game::perspective::Game<'_>::player_elytra_geometry`
- `horizon_client_lib::mobs::anim::player_model_visibility`
- `horizon_client_lib::mobs::anim::setup`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
