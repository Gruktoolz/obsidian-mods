# ContinuityHorizon (`connected_textures`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Continuity for Horizon

**Descripción del catálogo:** Continuity-style connected textures for Horizon, as a native mod: blocks of the same kind join up into one
surface instead of showing a border on every block. It works with the game's own textures; no resource pack needed.

- **Glass and stained glass** (all 16 colours and tinted glass): the frame between two blocks of the same glass goes
  away, so a window is one pane with one frame round it.
- **Glass panes**: the same, between panes of the same colour.
- **Bookshelves, cut and chiseled sandstone** (red too): joined left and right into one long shelf or band.
- **Sandstone and red sandstone**: a wall or pillar loses the band between its blocks; the top row keeps it.

Options > Mods > Conn…

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/ContinuityHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.7 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: connected-textures, glass, visual, port

## Cómo funciona
Se engancha en 6 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client_render::gfx::reuse::On<'_>::create_shader_module`
- `horizon_client_render::mesher::Mesher<'_>::mesh_reusing`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
