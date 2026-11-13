# Voltz (`voltz_core`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Machines, power, pipes, a nuclear reactor, missiles, turrets and force fields: the classic Voltz tech pack remade for Horizon.

**Descripción del catálogo:** **This mod has two parts, and you need both.** Install this one, the native part, and [Voltz (assets)](https://playhorizon.dev/mods/voltz), then press Build.

Voltz is a big tech and explosives pack for Horizon, remade from the spirit of the classic Voltz modpack. It adds working machines, power, automation and weapons, each set in its own creative tab:

- **Power**: wire, coal, heat, wind, solar and geothermal generators, batteries and energy storage.
- **Processing**: electric furnace, crusher, compressor, extractor, enrichment chamber, infuser, alloy smelter, SAG mill, smeltery, coke oven, blast furnace, centrifuge and more, with survival recipes.
- **Automation**: item pipes, quarry, min…

- Autores: Arthur Black · Licencia: MIT · Fuente: https://github.com/d3xc0/voltz-horizon
- Tipo: mixin (parchea el código del juego) · versión 0.3.0 · pide la revisión de Horizon 66f3981f52
- Etiquetas: machines, energy, explosives, automation, reactor

## Cómo funciona
Se engancha en 13 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_container`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_tick`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_ticks`
- `horizon_client::game::native_hooks::Game<'_>::native_block_entity_update_tag`
- `horizon_client::game::native_hooks::Game<'_>::native_chunk_load`
- `horizon_client::game::native_hooks::Game<'_>::native_level_tick_post`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_clicked`
- `horizon_client::game::native_hooks::Game<'_>::native_menu_closed`
- `horizon_client::game::native_hooks::Game<'_>::native_right_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_server_tick`
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
