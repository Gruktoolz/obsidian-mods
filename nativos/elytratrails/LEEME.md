# ElytraTrails (`elytratrails`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A Horizon mod that adds elytra trails with additional features.

**Descripción del catálogo:** A native mod for Horizon. The idea comes from [ElytraTrails](https://github.com/dbrighthd/elytratrails) by dbrighthd.

What it does

Lines stream from the outer tip of each wing while you glide and fade out over time. Only players get trails.

- Colors: ice, gold, rose, violet, white, rainbow, or speed (cold blue when slow, orange and thicker when fast).
- Patterns: solid, dashed or wavy. Built in only.
- Pulse: gently breathing brightness.
- Night: lines are brighter at night and dimmer in the day.
- Boost: a sudden speed jump, such as a firework, lights the whole trail up for two seconds.
- Formation: gliding players close to each other are joined by white lines between their wingtips.
- L…

- Autores: soufi · Licencia: MIT · Fuente: https://github.com/soufied/ElytraTrails-Horizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.2 · pide la revisión de Horizon 02c8607ee8
- Etiquetas: elytra, trails, flying, cosmetic, port

## Cómo funciona
Se engancha en 8 puntos del juego:
- `hook:mods`
- `hook:net_from_client`
- `hook:net_from_server`
- `hook:server_tick`
- `hook:settings`
- `hook:settings_click`
- `hook:tick`
- `horizon_client::game::native_visuals::Game<'_>::native_frame_visuals`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
