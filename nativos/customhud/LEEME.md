# Beautiful Day Counter (`customhud`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::<Game<'_> as ApplicationHandler>::window_event'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** A lightweight day counter, clock and day/night cycle bar for the HUD, with an in-game settings screen.

**Descripción del catálogo:** Beautiful Day Counter for Horizon, a native mod.

Shows the current survival day, an in-game clock and a color-shifting bar that follows the day/night cycle, without changing gameplay. It is a port of the author's own Fabric/NeoForge mod [CustomHUD](https://github.com/sonoFrangu/CustomHUD), with the same HUD and the same settings screen.

- Day counter with the game mode icon; a golden apple or totem, a rainbow counter and a sound on milestone days (50, 100, 365, every 100).
- 12-hour or 24-hour clock; the clock icon turns into a bed when you can sleep.
- Day/night bar with four color themes (Classic, Pastel, Colorblind, Mono), night marks, countdown to night or day, moon phase.
- Top or bot…

- Autores: SCARABOCCHIO100 · Licencia: MIT · Fuente: https://github.com/sonoFrangu/CustomHUD-Horizon
- Tipo: mixin (parchea el código del juego) · versión 1.7.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: day-counter, clock, hud, day-night, moon-phase, port

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::<Game<'_> as ApplicationHandler>::window_event`
- `horizon_client::game::dragon::Game<'_>::draw_boss_bars`
- `horizon_client::game::extras::Game<'_>::draw_extras`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::server_links::Game<'_>::draw_dialog_warning`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: h · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
