# MoreGamma (`fullbright`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Brightness past 100%: set your own gamma with /gamma, or toggle full brightness with /fullbright.

**Descripción del catálogo:** A native Horizon mod: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Uses only Horizon's native hook points (hooks 1); tested on Horizon 41672b1 and horizon-mixins 0d6b103.

See in the dark without torches or night vision. MoreGamma lets you set the brightness (gamma) far past the video settings' 100% "Bright" limit, with a chat command, and switch full brightness on and off.

- `/gamma <value>`: your own gamma. `1` is vanilla's "Bright", `15` is fullbright, up to `100`; `/gamma 1500%` works too.
- `/gamma` shows the gamma in effect…

- Autores: bezi5kt · Licencia: MIT · Fuente: https://github.com/bezi5kt/gamma-rust
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 41672b1221
- Etiquetas: fullbright, gamma, brightness, night-vision

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
