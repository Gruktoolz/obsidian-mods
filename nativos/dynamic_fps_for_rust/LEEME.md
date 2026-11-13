# Dynamic FPS for rust (`dynamic_fps_for_rust`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::handle_window_event'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Reduce resource usage while Minecraft is in the background, idle, or on battery

**Descripción del catálogo:** Dynamic FPS for Horizon: it lowers the frame rate, and optionally the volume, while the game window is in the background, hovered but not focused, minimised or idle, or while Horizon's battery mode is on.

- Caps by default: 30 FPS hovered, 10 unfocused (after half a second), 1 minimised, 20 after a minute idle, 30 on battery; no cap while you play.
- At 1 FPS the game still runs its input, network and ticks up to 20 times a second.
- Volume lowering is off by default.
- Settings: `config/dynamic-fps-for-rust.json` in the game folder, written with the defaults on first start. There is no settings screen yet.

An independent implementation after the public description of Dynamic FPS; it holds…

- Autores: Lyvt (dc: _.lyvt) · Licencia: MIT · Fuente: https://github.com/Lyvt-Dev/dynamic-fps-for-rust
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 998b1d8301
- Etiquetas: fps, performance, battery, background, idle

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::Game<'_>::handle_window_event`
- `horizon_client::game::Game<'_>::render`
- `horizon_client::game::music::Game<'_>::tick_sound_engine`
- `horizon_client::game::options::Game<'_>::framerate_limit`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
