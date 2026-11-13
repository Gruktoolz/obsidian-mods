# LivingMotion (`livingmotion`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Punchy moves ported into a hnmod.

**Descripción del catálogo:** Makes ur avatar animated in 3rd person. simmilar to punchy moves.

- Autores: bart · Licencia: MIT · Fuente: https://github.com/bartekchomik26-dotcom/Living-motion
- Tipo: mixin (parchea el código del juego) · versión 1.2.0 · pide la revisión de Horizon f319701e8c
- Etiquetas: punchy, animation, 3rdperson

## Cómo funciona
Se engancha en 10 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_render_hand_pose`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::text_displays::Game<'_>::text_display_geometry`
- `horizon_client_lib::mobs::anim::player_model_visibility`
- `horizon_client_lib::mobs::anim::setup`
- `horizon_client_lib::mobs::render::species_rotations`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: comma, i, period, y · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
