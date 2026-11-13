# Living Camera (`living_camera`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A camera that moves with you: it leans into strafes, sways on turns, dips on landing and bobs with your steps

**Descripción del catálogo:** A camera that moves with you. It leans into strafes, tips as you jump and fall, sways on fast turns, dips when you land, bobs with your steps and breathes while you stand still. Everything runs on springs, so the camera eases into each motion and settles back with a little weight instead of snapping.

- **Strafe and forward lean**: the camera rolls into sideways movement and tips slightly while running.
- **Turn sway**: fast mouse turns tilt the view.
- **Jump and fall tilt**: the view lifts going up and dips coming down.
- **Landing dip**: a hard landing bumps the camera down, then it springs back.
- **Step bob**: a soft bounce in step with your walking, on top of vanilla's.
- **Idle breath…

- Autores: selfmonoraque5 · Licencia: MIT · Fuente: https://github.com/selfmonoraque5/living-camera
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: camera, immersion, client-side, animation, fov, view-bobbing

## Cómo funciona
Se engancha en 10 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_camera_shake`
- `horizon_client::game::native_hooks::Game<'_>::native_compute_fov_modifier`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_health_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_render_hand_pose`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
