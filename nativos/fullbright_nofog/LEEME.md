# Fullbright & NoFog (`fullbright_nofog`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Toggle full brightness and the world's fog with a key (G / N), /fullbright or /nofog.

**Descripción del catálogo:** A native Horizon mod: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Uses only Horizon's native hook points (hooks 80, for `render_fog`); tested on Nightly 2026-11-01 (c9d474d) and horizon-mixins 0d6b103.

- **Fullbright**: press G (or `/fullbright`, `/fb`) to toggle full brightness (gamma 1500%). Your brightness slider and `options.txt` are never changed; turning it off gives your own brightness back.
- **NoFog**: press N (or `/nofog`) to remove the world's fog. `/nofog fluids` also clears water, lava and powder snow fog.
- `/fullb…

- Autores: Nirokx · Licencia: MIT · Fuente: https://github.com/Nirokx/horizon-fullbright-nofog
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: fullbright, gamma, brightness, fog, no-fog

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_render_fog`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: g, n · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
