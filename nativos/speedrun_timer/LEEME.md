# Speedrun Timer (`speedrun_timer`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Speedrun Timer Is a mod made to count the time it took you to beat the Game

**Descripción del catálogo:** An RTA speedrun timer in a corner of the screen, `HH:MM:SS.mmm` with live milliseconds, as a native mod for
Horizon.

- **Starts** on your first move after a world loads, and **stops** when you take the exit portal back from the End.
  Dying is not a finish: the run keeps counting.
- **Resets** on its own when you leave a world, ready for the next run.
- **Saves** every finished run, and your personal best, in the mod's data folder; its settings page has a button to
  open it.
- `/speedrun`, `/speedrun start`, `/speedrun stop` and `/speedrun reset`.

Pick its corner in Options > Mods > Speedrun Timer > Settings. It hides with F1 and keeps clear of F3.

- Autores: Demerobit · Licencia: MIT · Fuente: https://github.com/Demerobit/Speedrun-Timer-Horizon-Rust-Minecraft
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: speedrun, timer, rta, hud

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
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
