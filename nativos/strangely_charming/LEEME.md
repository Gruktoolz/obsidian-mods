# Strangely Charming (`strangely_charming`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::first_person_arm'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** An exploration and quality-of-life mod

**Descripción del catálogo:** A collection of crafting, exploration and village features for Horizon. Each feature defaults to enabled and can be turned off in config/strangely_charming.json. Modpack makers are encouraged to prune features they feel are overpowered or conflict with other mods.

See github for full description and features list: https://github.com/rewenx/strangely-charming/blob/main/README.md"

- Autores: Evander · Licencia: MIT · Fuente: https://github.com/rewenx/strangely-charming
- Tipo: mixin (parchea el código del juego) · versión 0.2.11 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: travel, exploration, magic

## Cómo funciona
Se engancha en 102 puntos del juego:
- `horizon_client::game::Game<'_>::creative_tab_open`
- `horizon_client::game::Game<'_>::draw_inventory_slots`
- `horizon_client::game::Game<'_>::first_person_arm`
- `horizon_client::game::Game<'_>::play_mouse`
- `horizon_client::game::Game<'_>::portal_shape`
- `horizon_client::game::Game<'_>::tick_mobs`
- `horizon_client::game::Game<'_>::use_instrument`
- `horizon_client::game::block_entity_ticks::Game<'_>::tick_furnace`
- `horizon_client::game::consume::Game<'_>::random_teleport`
- `horizon_client::game::creatures::Game<'_>::credit_kill`
- `horizon_client::game::creatures::Game<'_>::mob_died`
- `horizon_client::game::creatures::Game<'_>::projectile_hit`
- `horizon_client::game::dialogs::Game<'_>::run_dialog_action`
- `horizon_client::game::hand::Game<'_>::destroy_progress`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: scrollkeepers → j; portable_crafting → v; inventory_tidying → y; portable_crafting → v; inventory_tidying → y; fullbright → g
- Teclas fijas en el código: g, j, v, y · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
