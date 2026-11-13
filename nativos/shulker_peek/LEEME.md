# ShulkerPeek (`shulker_peek`)

**Estado:** no compila en Android: firma con 7 argumentos

**Qué hace:** A shulker box's contents as a 9 x 3 grid in its tooltip.

**Descripción del catálogo:** Hover a shulker box to see what's inside: its 27 slots appear as a 9 x 3 grid right in the tooltip, laid out exactly like the box's own screen, with item icons and stack counts.

- Works with the plain shulker box and all 16 dyed ones, whether empty, partly filled or full.
- Works on every screen with item tooltips: your inventory, chests and other containers, the creative inventory, the item panel, chat item links and trading.
- Keeps the tooltip's usual lines (name, lore, "64 x Diamond"...); the grid goes underneath.
- Respects hidden tooltips: a box whose `minecraft:tooltip_display` hides its contents stays hidden.
- Settings in Options > Mods > ShulkerPeek > Settings: show the grid alway…

- Autores: vstxx · Licencia: MIT · Fuente: https://github.com/vstxx/shulkerpeek
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 5e391eb673
- Etiquetas: shulker-box, tooltip, inventory, preview

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::stations::Game<'_>::draw_stack_tooltip`
- `horizon_client::lan::hub::Hub::stack`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
