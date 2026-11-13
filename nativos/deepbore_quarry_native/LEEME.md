# Deepbore Quarry (`deepbore_quarry_native`)

**Estado:** no compila en Android: content_root con firma antigua

**Qué hace:** A machine that digs a square pit down to bedrock on furnace fuel and fills a chest, under a moving gantry with a drill.

**Descripción del catálogo:** **This mod has two parts, and you need both.** Install this one, the native part, and [Deepbore Quarry (assets)](https://playhorizon.dev/mods/deepbore_quarry), then press Build. If one is missing, the game says so in chat.

- **Place** a quarry. The pit lies beyond it, the way you are looking, and goes from the quarry's own level down to bedrock.
- **Put a chest** against another side of it, and fuel in its slot: anything a furnace burns. One coal digs eight blocks.
- **Watch** the gantry: a drill runs over the pit on a moving bridge, row by row. The screen says what the quarry is doing, and why when it stands still.
- **Choose** four upgrades: Area up to 17×17, Speed up to a block in 0.25 s…

- Autores: bioneisme · Licencia: MIT · Fuente: https://github.com/Bioneisme/deepbore-quarry
- Tipo: mixin (parchea el código del juego) · versión 0.1.2 · pide la revisión de Horizon 67dad0d1a5
- Etiquetas: quarry, mining, machine, automation, digging

## Cómo funciona
Se engancha en 25 puntos del juego:
- `horizon_client::game::mods::content_root`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_container`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_render_bounding_box`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_submit`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_ticks`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_update_tag`
- `horizon_client::game::native_hooks::Game<'_>::native_blocks`
- `horizon_client::game::native_hooks::Game<'_>::native_destroy_block`
- `horizon_client::game::native_hooks::Game<'_>::native_items`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_button`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_quick_move`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- …

## Qué necesita
- Datos que trae: data/deepbore_quarry-content.hmod
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
