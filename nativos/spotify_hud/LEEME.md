# Spotify HUD (`spotify_hud`)

**Estado:** no compila en Android: usa arboard (portapapeles de escritorio)

**Qué hace:** What Spotify is playing on your HUD, with play, pause, skip, shuffle, repeat and volume.

**Descripción del catálogo:** Shows the song Spotify is playing in a box on the HUD: cover, title, artists and progress. Open chat and point at the box for buttons: play/pause, next, previous, shuffle, repeat, volume, the song's link, settings and moving the box.

## Connect Spotify (one time)

1. Go to https://developer.spotify.com/dashboard and press **Create app**.
2. Add the redirect URI `http://127.0.0.1:21851/spotify/callback`, tick **Web API**, save.
3. In a world: `/music clientid <Client ID>`.
4. Then `/music login` and allow it in the browser (within two minutes).

The buttons need **Spotify Premium** (Spotify refuses them from free accounts) and a song playing on some device.

## Using it

- `/music` opens the…

- Autores: tomasmendes01 · Licencia: MIT · Fuente: https://github.com/tomasmendes01/spotify-hud-horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 20186e4044
- Etiquetas: spotify, music, hud, media-controls

## Cómo funciona
Se engancha en 14 puntos del juego:
- `hook:command`
- `hook:commands`
- `hook:frame`
- `hook:hud`
- `hook:key`
- `hook:mods`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `hook:settings`
- `hook:settings_click`
- `hook:tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
