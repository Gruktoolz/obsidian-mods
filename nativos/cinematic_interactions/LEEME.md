# Cinematic Interactions (`cinematic_interactions`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::hand_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Using a crafting table, furnace, anvil, enchanting table and more moves the camera in close for a cinematic, hands-on scene.

**Descripción del catálogo:** By tariksabanuz. Cinematic Interactions for Horizon, a port of Southware's Cinematic Interactions for Fabric: when you use a station,
the camera glides in and your hands work it in a close-up scene instead of opening a plain menu.

- Crafting table, crafter, stonecutter, loom and cartography table, furnace, smoker, blast furnace and campfire.
- Anvil with the hammer swing, grindstone, smithing table, enchanting table with its rune circles, brewing stand, beacon, lectern, jukebox and chiseled bookshelf.
- Right-click the next station beside you to move straight over to it.
- Cinematic Interaction Settings... in Options: turn stations off, tune the camera speed, lean, zoom, black bars and scre…

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/CinematicInteractionsHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.12.2 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: animation, interaction, menu, cinematic, port

## Cómo funciona
Se engancha en 32 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client::game::Game<'_>::clicked_outside`
- `horizon_client::game::Game<'_>::close_container`
- `horizon_client::game::Game<'_>::draw_inventory`
- `horizon_client::game::Game<'_>::hand_geometry`
- `horizon_client::game::Game<'_>::hud_layers`
- `horizon_client::game::Game<'_>::inventory_press`
- `horizon_client::game::Game<'_>::key_pressed`
- `horizon_client::game::Game<'_>::open_menu`
- `horizon_client::game::Game<'_>::slot_at`
- `horizon_client::game::campfire::Game<'_>::campfire_geometry`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
