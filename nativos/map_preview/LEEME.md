# Map Preview (`map_preview`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** See a map's picture in its tooltip and in its inventory slot, so you can tell your maps apart

**Descripción del catálogo:** See what's on a map without holding it.

- **Tooltip preview**: hover a filled map and its picture shows in the tooltip. Always, or only while holding Shift, in three sizes.
- **Slot preview**: every map in your hotbar, inventory and chests shows a small copy of its picture instead of the plain map icon, with or without a paper edge.
- Maps update live as you explore.

All in Options > Mods > Map Preview, with a reset button.

Client side only: it works in single player and on any server. A preview needs the map's picture on your client: the server sends it for maps you carry, as vanilla does, so a map you've never carried may show no preview until you pick it up once.

Horizon has no hook p…

- Autores: selfmonoraque5 · Licencia: MIT · Fuente: https://github.com/selfmonoraque5/map-preview
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: maps, tooltip, inventory, preview, client-side

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_item_decorations`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tooltip_component`
- `horizon_client::game::native_hooks::Game<'_>::native_tooltip_component_draw`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: a, right.shift · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
