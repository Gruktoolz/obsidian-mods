# Spider-Man (`spider_man`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Web swinging, zips, wall running and parkour, tuned against the console games' feel.

**Descripción del catálogo:** Swing through a city on webs. Hold **R** and jump: a web catches the nearest anchor ahead and you swing on it, let go
to fly on, catch the next one.

**Controls** (change them on the mod's page in Options > Mods):
- **R**: hold to swing; on the ground, a parkour run, faster with **W**; on a wall, a wall run
- **Z**: hold to zip to where you look
- **Space** on a web: pump for speed; let go to jump off
- **A / D** while swinging: lean to the side
- **F5**: third person, where the poses, the web lines and the chase camera show

**Settings** (Options > Mods > Spider-Man): how long a web flies before it bites. `/swingmode on|off|delay|view|debug`
in chat does the same and more.

Works in your ow…

- Autores: wyddy7 · Licencia: MIT · Fuente: https://github.com/wyddy7/minecraft-spider-man-rust
- Tipo: mixin (parchea el código del juego) · versión 0.2.0 · pide la revisión de Horizon 294448014f
- Etiquetas: spider-man, web-swinging, parkour, wall-running, movement

## Cómo funciona
Se engancha en 13 puntos del juego:
- `horizon_client::game::leads::Game<'_>::leash_geometry`
- `horizon_client::game::native_hooks::Game<'_>::native_camera_shake`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::perspective::Game<'_>::camera_view`
- `horizon_client::game::perspective::Game<'_>::player_model_geometry`
- `horizon_client::game::remote::Game<'_>::remote_send_state`
- `horizon_client::game::remote::Game<'_>::tick_remote`
- `horizon_client_render::camera::view_matrix`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: r, z · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
