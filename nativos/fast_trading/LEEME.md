# Fast Trading (`fast_trading`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A speed-trade button on the merchant screen repeats the selected trade as fast as the settings allow.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launchers Mods page, then press Build there). Native mods are not sandboxed and run with your accounts rights, so read the source before you build it. Pinned to the Horizon tree this was built against.

- Speed trade button on the merchant screen repeats the selected trade quickly.
- Settings control how fast it repeats; a command opens the controls.

Port of [FastTrading](https://github.com/bendy1234/FastTrading) by bendy1234 (MIT); credited upstream, original license preserved. Feature gaps vs upstream are documented in the port README and tracked on horizon-rust/horizon.


- Autores: sparticle62ops · Licencia: MIT · Fuente: https://github.com/Sparticle62ops/horizon-fast-trading
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 3fb4c3138e
- Etiquetas: trading, villagers, gui, qol

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_input_key`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_mouse_button_pressed_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_render_post`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
