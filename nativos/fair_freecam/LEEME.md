# Fair Freecam (`fair_freecam`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::hand_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Fly the camera freely while your player stays where it is, with an optional fair play mode.

**Descripción del catálogo:** Press **F4** to leave your body and fly the camera anywhere. Your player stays where it is and you see it from outside. Press **F4** again to snap back.

**Controls while flying:**
- **W A S D**: fly where you look
- **Space / Shift**: up / down
- **Ctrl**: 2.5x faster
- **Mouse wheel**: change the speed

**Settings** (Options > Mods > Fair Freecam):
- **Speed**: 2, 5, 10, 20, 40 or 80 blocks per second.
- **Fair play**: the camera can't fly through blocks, so no peeking into caves or bases through walls. Glass, fences and doors stop it, grass and flowers don't.
- **Allowed on servers**: turn it off to only use freecam in your own worlds.

While flying, your player doesn't move, attack, mine…

- Autores: Lutziano888 · Licencia: MIT · Fuente: https://github.com/Lutziano888/horizon-freecam
- Tipo: mixin (parchea el código del juego) · versión 1.2.0 · pide la revisión de Horizon aa27eca326
- Etiquetas: freecam, camera, spectator

## Cómo funciona
Se engancha en 13 puntos del juego:
- `horizon_client::game::Game<'_>::hand_geometry`
- `horizon_client::game::Game<'_>::key_pressed`
- `horizon_client::game::Game<'_>::play_mouse`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- `horizon_client::game::perspective::Game<'_>::camera_view`
- `horizon_client::game::perspective::Game<'_>::own_body_in_view`
- `horizon_client::game::perspective::Game<'_>::turn_player`
- `horizon_client::game::remote::Game<'_>::tick_remote_level`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: f4 · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
