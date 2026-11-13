# Fast Items for Horizon (`fast_items`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Optimize item rendering by transforming item entities to 2D

**Descripción del catálogo:** Flat, camera-facing dropped items for Horizon, as a native mod inspired by Noryea's
[Fast Items](https://modrinth.com/mod/fast-items) (independently written; no code from it).

- **Flat items** (potions, tools...): the sprite's edges and hidden back face are dropped, so each layer is one flat
  quad. Multi-layer textures, tint, lighting and glint stay.
- **Block items** keep their 3D model and turn to face the camera, as in Lunar Client and Badlion.
- Item bobbing, stacks, pickup animation, culling and glow outlines work as before.
- Optionally turns off dropped items' blob shadows.

Only dropped items change: inventory icons, held items, item frames and servers stay as they are. Settings ar…

- Autores: Lyvt (dc: _.lyvt) · Licencia: MIT · Fuente: https://github.com/Lyvt-Dev/FastItems-for-Horizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 998b1d8301
- Etiquetas: items, rendering, performance, 2d-items, port

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::Game<'_>::entity_geometry`
- `horizon_client::game::item_draw::Game<'_>::dropped_item_geometry`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
