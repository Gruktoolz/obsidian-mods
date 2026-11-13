# Fly Speed (`fly_speed`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Allows you to change your flight speed

**Descripción del catálogo:** Fly Speed

A native Horizon mod: /flyspeed changes how fast you fly, from chat or from Options > Mods > Fly Speed > Settings.
Commands
Command 	What it does
/flyspeed 	Says the current multiplier
/flyspeed 2 	Flies twice as fast
/flyspeed 1.5 	Any multiplier from 0.1 to 20
/flyspeed +1 / /flyspeed -0.5 	Steps from the current one
/flyspeed reset 	Back to the game's own speed (1x)
/flyspeed-help 	Shows the command help
/flyspeed-reset 	Back to the game's own speed (1x)

Horizon chat autocomplete lists /flyspeed, /flyspeed-help and /flyspeed-reset when the mod is enabled. Numeric and relative arguments are entered after /flyspeed as shown above.

The settings page has one button that steps thr…

- Autores: ImmortalDev · Licencia: All Rights Reserved · Fuente: https://github.com/ImmortalWhisper/Fly-Speed
- Tipo: mixin (parchea el código del juego) · versión 0.2.0 · pide la revisión de Horizon 9d979caa48
- Etiquetas: flying, speed, creative, commands

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
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
