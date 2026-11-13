# Classic Zoom (`classic_zoom`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Hold V to zoom 3x, scroll for more, release to snap back. Instant, no settings, inspired by WI Zoom.

**Descripción del catálogo:** ## Controls

| Action | Result |
|---|---|
| Hold **V** | Zoom in 3x |
| Scroll while holding V | Zoom from 1x to 10x, x1.1 per wheel step |
| Release V | Back to normal view at once |

While V is held, the mouse wheel zooms instead of switching hotbar slots, and turning slows down in step with the zoom so aiming stays steady. Rebind the key on the mod's page in Options > Mods.

## How it differs from Horizon's own Zoom

| | Horizon Zoom | Classic Zoom |
|---|---|---|
| Default key | Z | V |
| Starting zoom | 4x (configurable) | 3x |
| Range | 1.25x to 10x | 1x to 10x |
| Wheel step | x1.25 | x1.1, finer |
| Transition | Smooth by default | Instant |
| Settings page | Yes | None |

Both can …

- Autores: Surge · Licencia: GPL-3.0-or-later · Fuente: https://github.com/SU4G3/classic-zoom
- Tipo: mixin (parchea el código del juego) · versión 2.0.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: zoom, camera

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- `horizon_client::game::native_hooks::Game<'_>::native_zoom`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
