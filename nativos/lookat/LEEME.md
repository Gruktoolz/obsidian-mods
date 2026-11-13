# LookAt (`lookat`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** The block or mob under your crosshair as a vanilla tooltip: icon, name in your language, health, the tool it needs and where it comes from

**Descripción del catálogo:** The block or entity under your crosshair at the top of the screen, drawn as a vanilla tooltip: its item's icon, its name in your language and the namespace it comes from.

- **Blocks**: up to 1024 blocks away, across distant terrain.
- **Mobs and players**: name and health as hearts or numbers; boats, minecarts, armor stands, paintings and item frames (with what they hold).
- **Harvest tool**: ✔ or ✘ with the tool a block needs.

**Settings** (Options → Mods → LookAt): range, position (top, top left, top right), size, background opacity, item icon, where it comes from, position and distance, entities, health, harvest tool. Boxes keep clear of F3, boss bars, the minimap and effect icons.

**N…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/lookat
- Tipo: mixin (parchea el código del juego) · versión 0.2.1 · pide la revisión de Horizon 142388e1c1
- Etiquetas: tooltip, hud, block-info, waila

## Cómo funciona
Se engancha en 4 puntos del juego:
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
