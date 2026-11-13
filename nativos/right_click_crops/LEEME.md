# Right-Click Crops (`right_click_crops`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Right-click harvest for 16 plant families, with replanting and individual settings.

**Descripción del catálogo:** Harvest mature plants with a right-click, replant crops, and keep your garden growing.

### Features
- Supports 16 plant families.
- Replants crops using one planting item from the harvest, or from your inventory.
- Keeps the base of tall plants and the stems of pumpkins and melons.
- Lets you enable or disable each plant separately in **Options > Mods > Right-Click Crops > Settings**.

### Supported plants
Wheat, carrots, potatoes, beetroot, nether wart, cocoa, torchflower, pitcher crop, sugar cane, cactus, bamboo, kelp, twisting vines, weeping vines, pumpkin and melon.

### Good to know
- Harvesting works in survival, one targeted plant at a time.
- Without a suitable planting item, the cr…

- Autores: AyanoKaito · Licencia: MIT · Fuente: https://github.com/AyanoKaito/horizon-right-click-crops
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon 66f3981f52
- Etiquetas: farming, crops, harvest, replant

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_right_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_use_item`
- `horizon_client::game::native_hooks::Game<'_>::native_use_item_on`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
