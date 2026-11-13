# Horizonmatica (`horizonmatica`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** Litematica for Horizon: schematics as ghost blocks, a verifier, material lists and a printer that builds the missing blocks for you.

**Descripción del catálogo:** Horizonmatica is a re-creation of **Litematica** (with its **printer**) for Horizon. Load a schematic, see it as ghost blocks in your world, check your build against it, get a material list, and let the printer place the missing blocks.

## Features
- **Formats:** `.litematic` (versions 1–7, several regions), `.schem` (Sponge v1–v3: WorldEdit, FAWE, Amulet, Axiom), `.schematic` (MCEdit / WorldEdit up to 1.12) and vanilla structure `.nbt`. Older files are upgraded with Horizon's data fixer, the same way old worlds are.
- **Overlay:** missing blocks as translucent real block models (chests, banners, heads and other block entities included), optionally visible through walls.
- **Verifier:** wro…

- Autores: keksi2k · Licencia: MIT · Fuente: https://github.com/keksi2k/horizonmatica
- Tipo: mixin (parchea el código del juego) · versión 0.3.2 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: schematics, litematica, building, printer, material-list, port

## Cómo funciona
Se engancha en 22 puntos del juego:
- `hook:command`
- `hook:commands`
- `hook:frame`
- `hook:hud`
- `hook:key`
- `hook:mods`
- `hook:render_needs`
- `hook:settings`
- `hook:settings_click`
- `hook:tick`
- `hook:wheel`
- `hook:world_drawn`
- `horizon_client::game::chat::Game<'_>::chat_tree_for`
- `horizon_client::game::debug_options::Game<'_>::close_debug_options`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: j, k, m, n, r, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
