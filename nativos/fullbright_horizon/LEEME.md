# Fullbright (`fullbright_horizon`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Toggle maximum 1000% gamma cave and night brightness with smooth fade via keybind G.

**Descripción del catálogo:** Full brightness in caves and at night, as a native mod for Horizon.

- Press **G** (rebindable in Options > Mods > Fullbright) to turn maximum brightness (1000% gamma) on or off.
- A smooth 250 ms fade, and a notice on the HUD saying which way it went.
- Client-side only: it changes your lightmap, nothing sent to a server.

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/fullbright
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: fullbright, gamma, brightness

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: g · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
