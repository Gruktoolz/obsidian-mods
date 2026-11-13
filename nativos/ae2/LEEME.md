# Applied Energistics 2 (`ae2`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::clicked_outside'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Unofficial port of Applied Energistics 2 15.4.12: ME networks, autocrafting, spatial storage and more.

**Descripción del catálogo:** An unofficial port of Applied Energistics 2 15.4.12 to Horizon, not made or endorsed by its authors. Please report problems to this port, not to them.

Rewritten in Rust: ME networks, storage cells, autocrafting, the cable bus with facades, P2P tunnels, spatial storage, the guide and the Fluix Researcher.

**One file, no separate content mod.**

### Optional integrations
Each is compiled in only when its mod is in the same build, so adding or removing JEI or Jade takes effect at the next build.
- **With JEI:** AE2's recipes and information pages, "+" transfer into terminals, and dragging items onto config slots.
- **With Jade:** network state, channels, P2P links, charge, crafting and stored…

- Autores: AlgorithmX2, TeamAppliedEnergistics, Joeypc6 · Licencia: LGPL-3.0-only AND CC-BY-NC-SA-3.0 · Fuente: https://github.com/Joeypc6/AE2-Horizon
- Tipo: mixin (parchea el código del juego) · versión 15.4.17 · pide la revisión de Horizon 6560641ba5
- Etiquetas: ae2, applied-energistics, storage, autocrafting, automation

## Cómo funciona
Se engancha en 170 puntos del juego:
- `hook:analog_output`
- `hook:attack_entity`
- `hook:block_entity`
- `hook:block_entity_container`
- `hook:block_entity_energy`
- `hook:block_entity_fluids`
- `hook:block_entity_render_bounding_box`
- `hook:block_entity_submit`
- `hook:block_entity_tick`
- `hook:block_entity_ticks`
- `hook:block_entity_update_tag`
- `hook:block_highlight`
- `hook:block_signal`
- `hook:chunk_load`
- …

## Qué necesita
- Datos que trae: data/ae2-content.hmod
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: category → unknown; recipe → r; uses → u; recipe → r; uses → u
- Teclas fijas en el código: ae2.ghost, e, r, u, w · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
