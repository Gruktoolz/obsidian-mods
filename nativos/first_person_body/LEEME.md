# First Person Body (`first_person_body`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::cosmetics::Game<'_>::cosmetics_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** See your own body in first person: look down to see your legs and feet

**Descripción del catálogo:** See your own body in first person. Look down and your torso, legs and feet are there, moving as you walk, crouch and fight, instead of the floating hand alone.

- **Your own model in first person**, with your skin, armour and cape.
- **Head, helmet and head items hidden**, so the camera never looks out of your skull.
- **The body sits slightly behind the camera**, a little more when you look down or crouch, so you never see inside your chest.
- **Vanilla first-person hand** by default, or the model's own arms.
- Steps aside while swimming, crawling, gliding with an elytra, riptide spinning, sleeping and in spectator.

Settings in Options > Mods > First Person Body: on/off, arms, body offset,…

- Autores: selfmonoraque5 · Licencia: MIT · Fuente: https://github.com/selfmonoraque5/first-person-body
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: first-person, body, camera, immersion, client-side

## Cómo funciona
Se engancha en 15 puntos del juego:
- `horizon_client::game::cosmetics::Game<'_>::cosmetics_geometry`
- `horizon_client::game::mob_blocks::Game<'_>::head_item_geometry`
- `horizon_client::game::mob_blocks::Game<'_>::held_item_geometry`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_render_hand`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::perspective::Game<'_>::own_body_in_view`
- `horizon_client::game::perspective::Game<'_>::player_ears_geometry`
- `horizon_client::game::perspective::Game<'_>::player_model_geometry`
- `horizon_client::mobs::anim::player_model_visibility`
- `horizon_client::mobs::render::base_transform`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
