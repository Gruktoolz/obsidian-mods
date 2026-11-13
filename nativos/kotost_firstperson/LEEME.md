# Kotost First Person (`kotost_firstperson`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::cosmetics::fx::Game<'_>::cosmetics_fx_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Your own body, legs and arms in first person, items in your 3D hands, the game's hands for eating.

**Descripción del catálogo:** - **Your own body in first person**: look down and see your body, legs and both arms in 3D, in your own skin, instead of the flat floating hands. The body leans back as you look down.
- **Items in your 3D hands**, main hand and off hand.
- **Eating and drinking**: the game's own hands bring the food before your face while you eat, or the 3D arm does (`eat_hand = 0`).
- **One shadow**: with a shader pack, the shadow pass keeps the head where it really is.
- **Settings** in `config/kotost-firstperson.txt` (re-read while you play). Kotost Body 0.3's first-person lines are carried over on the first run.

Works alone. With Kotost Bust your body has its bust and figure; whatever a mod draws on the…

- Autores: Kotost Games · Licencia: MIT · Fuente: https://github.com/Leks2000/kotost-firstperson
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: first-person, player-model, body, immersion

## Cómo funciona
Se engancha en 10 puntos del juego:
- `hook:render_hand`
- `horizon_client::game::cosmetics::covered`
- `horizon_client::game::cosmetics::fx::Game<'_>::cosmetics_fx_geometry`
- `horizon_client::game::mob_blocks::Game<'_>::head_item_geometry`
- `horizon_client::game::mob_blocks::Game<'_>::held_item_geometry`
- `horizon_client::game::perspective::Game<'_>::player_model_geometry`
- `horizon_client::game::perspective::Game<'_>::player_model_shown`
- `horizon_client_lib::mobs::anim::bob`
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
