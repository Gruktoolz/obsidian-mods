# FreeCam (`freecam_yannis`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Free camera, F6 follow view and nine tripod positions for Horizon.

**Descripción del catálogo:** ## FreeCam by Yannis_

Free flight, a high and wide follow view, and nine tripod positions — now native to Horizon.

### Controls

- **F4:** toggle free camera. Move with your movement keys, Jump/Sneak and Sprint. Mouse look is independent of the player.
- **F6:** follow your player while continuing to play normally. The crosshair is hidden while following. Toggle again to restore your previous camera.
- **Hold F4 + 1–9:** save or return to a tripod for the current world.
- **Options → Mods → FreeCam:** rebind keys and adjust camera speed, follow distance and height. Default follow distance: 28 blocks; height: 18 blocks.

### Original Modrinth project

This is the native Rust edition of [Fre…

- Autores: Yannis_ · Licencia: MIT · Fuente: https://github.com/yan6678yan-design/freecam-horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: camera, freecam, follow-camera, screenshots, client

## Cómo funciona
Se engancha en 15 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_block_highlight`
- `horizon_client::game::native_hooks::Game<'_>::native_camera_pose`
- `horizon_client::game::native_hooks::Game<'_>::native_client_disconnecting`
- `horizon_client::game::native_hooks::Game<'_>::native_client_level_unload`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_interaction_key`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_movement_input`
- `horizon_client::game::native_hooks::Game<'_>::native_render_hand`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::perspective::Game<'_>::own_body_in_view`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: free → f4; follow → f6
- Teclas fijas en el código: f4, f6 · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
