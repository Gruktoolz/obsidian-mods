# Freecam+ (`freecam_plus`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::<Game<'_> as ApplicationHandler>::window_event'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Fly the camera anywhere while your player stays perfectly still.

**Descripción del catálogo:** Press U and your camera leaves your body. Fly it with WASD, Space and Shift, hold Ctrl to go faster and use the mouse wheel to change the speed. Your player does not move or turn, so the server sees you standing still. Through walls and speed can be changed in Options, Mods, Freecam.

Since this is a native mod it will not work in v0.2.97 as there are no native hooks possible. This works on all nightly releases and LIKELY on version upwards from v0.2.97.

- Autores: _idom_ · Licencia: MIT · Fuente: https://github.com/umbx-dot/horizon-freecam
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon 66f3981f52
- Etiquetas: freecam, camera, spectator

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::<Game<'_> as ApplicationHandler>::window_event`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- `horizon_client::game::perspective::Game<'_>::camera_view`
- `horizon_client::game::perspective::Game<'_>::turn_player`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: u · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
