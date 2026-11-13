# Motion Blur (`motion_blur`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Blurs what moves: mobs, players, items and vehicles along their own motion, with optional camera blur.

**Descripción del catálogo:** Moving mobs, players, items, minecarts and boats blur along their own motion. The camera's motion (turning, walking) can blur too, and is off by default.

Each frame the mod works out every pixel's motion since the last frame (the world's from the depth buffer and the camera, entities from last frame's vertices), finds the largest motion near each tile, and gathers along it with depth-aware weights (McGuire et al., "A Reconstruction Filter for Plausible Motion Blur", 2012).

Settings are in Options → Mods → Motion Blur: strength for moving things and the camera, shutter angle or a fixed exposure, the longest blur, quality, blurring yourself in third person, and a view that shows each pixel's…

- Autores: levzzz5154 · Licencia: MIT · Fuente: https://github.com/levzzz5154/horizon-motion-blur
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: motion-blur, post-processing, graphics, visual

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client_render::gfx::native_render::native_render_needs`
- `horizon_client_render::gfx::native_render::native_world_drawn`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
