# Smoothie Capes (`smoothie_capes`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** An Wavey Capes inspired mod for Horizon

**Descripción del catálogo:** Capes that move like cloth, as a native mod for Horizon inspired by Wavey Capes.

It replaces how the game draws capes: each cape becomes a strip of 17 linked segments simulated every frame, so it
trails out behind you when you run, swings to the side when you strafe or turn, lifts when you fall, and sways gently
when you stand still. It works on every cape the game draws.

Only how capes look changes; nothing is sent to a server. There are no settings: install it, build, and play.

- Autores: Nully · Licencia: MIT · Fuente: https://github.com/dumbnully/smoothie-capes
- Tipo: mixin (parchea el código del juego) · versión 0.5.1 · pide la revisión de Horizon 298f165557
- Etiquetas: capes, cloth, physics, animation, cosmetic

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::perspective::Game<'_>::player_elytra_geometry`
- `horizon_client_lib::mobs::render::cape`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
