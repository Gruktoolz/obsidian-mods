# Trails (`trails`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Paw prints, particles, lines or ribbons follow you as you move. Client-side, only you see them.

**Descripción del catálogo:** **Trails** leaves a trail behind you as you move. Only you see it: nothing is sent to the server or to other players.

- **Cat paws**: alternating paw prints on the blocks you walk on. They follow the local light and vanish when the block under them is removed or changes height.
- **Particle**: sparkles, dust, orbs or hearts drifting behind you.
- **Line**: a thin line along your path.
- **Ribbon**: a flat ribbon along your path.

Trails fade after a few seconds, or stay in persistent mode (bounded in memory, never saved into the world). Changing mode, dimension or world clears them.

**Controls**
- **K** turns trails on and off (rebind it in Options > Mods > Trails).
- **Options > Mods > Tr…

- Autores: fbi · Licencia: MIT · Fuente: https://github.com/fbikat/trails-horizon/
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: trails, particles, paw-prints, cosmetic, client-side

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_world_primitives`
- `horizon_client::game::native_hooks::Game<'_>::native_world_submit`


## Qué necesita
- Datos que trae: data/atlas.rgba
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: k · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
