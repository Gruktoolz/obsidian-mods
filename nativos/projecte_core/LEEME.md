# ProjectE for Horizon (`projecte_core`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Unofficial port of ProjectE: EMC, transmutation tables, collectors, condensers, dark and red matter tools and armour.

**Descripción del catálogo:** **This mod has two parts, and you need both.** Install this one, the native part, and [ProjectE for Horizon (assets)](https://playhorizon.dev/mods/projecte), then press Build.

An **unofficial** port of [ProjectE](https://github.com/sinkillerj/ProjectE) to Horizon. It isn't made or supported by the ProjectE team, so please report its bugs here, not to them.

Break items down into EMC and turn that EMC into new items:

- **Transmutation Table and Tablet**: burn an item to learn it and gain its EMC, then make more of anything you've learned. Your EMC and knowledge are saved per player.
- **Philosopher's Stone**: changes blocks in the world, with charge and modes, and works as a crafting tool.
…

- Autores: Arthur Black · Licencia: MIT · Fuente: https://github.com/d3xc0/projecte-horizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 66f3981f52
- Etiquetas: emc, transmutation, alchemy, tools

## Cómo funciona
Se engancha en 22 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_container`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_ticks`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_update_tag`
- `horizon_client::game::native_hooks::Game<'_>::native_component_types`
- `horizon_client::game::native_hooks::Game<'_>::native_destroy_block`
- `horizon_client::game::native_hooks::Game<'_>::native_inventory_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_item_bar`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_left_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_clicked`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_closed`
- `horizon_client::game::native_hooks::Game<'_>::native_net_from_client`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: c, g, r, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
