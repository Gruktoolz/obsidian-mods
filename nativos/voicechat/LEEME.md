# Horizon Voice Chat (`voicechat`)

**Estado:** no compila en Android: remote_entity_position no existe

**Qué hace:** Talk with Simple Voice Chat players: proximity voice, groups, push to talk and a menu (V).

**Descripción del catálogo:** An unofficial client for the [Simple Voice Chat](https://github.com/henkelmax/simple-voice-chat) protocol. On a server with the Simple Voice Chat mod or plugin, you can talk with players who use the Java mod on the regular launcher.

## What it does

- **Menu (V):** connection status, microphone and voice chat on/off, push to talk or voice activation, voice volume, microphone gain and activation level with a live meter, and every hotkey as it is bound now. The right side lists the server's groups with their members: join (with a password if needed), leave, or create a group.
- **Talking:** hold Caps Lock (push to talk) or use voice activation. Voices come from where players stand, getting qu…

- Autores: Nirokx · Licencia: MIT · Fuente: https://github.com/Nirokx/horizon-voice-chat
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: voice-chat, proximity-chat, simple-voice-chat, multiplayer, push-to-talk

## Cómo funciona
Se engancha en 15 puntos del juego:
- `hook:command`
- `hook:commands`
- `hook:frame`
- `hook:hud`
- `hook:key`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_key_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_scrolled_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_render_post`
- `horizon_client::game::remote::Game<'_>::handle_packet`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: caps.lock, j, k, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
