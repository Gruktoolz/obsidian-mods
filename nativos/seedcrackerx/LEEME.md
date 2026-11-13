# SeedCrackerX (`seedcrackerx`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** SeedCrackerX for Horizon: finds the world seed of a server from the structures you walk past, outlined through walls.

**Descripción del catálogo:** A port to Rust of [SeedCrackerX](https://github.com/19MisterX98/SeedcrackerX) 2.16.2 for Horizon, written from the Java source together with the parts of the SeedFinding libraries it calls. Walk near structures (igloos, temples, monuments, shipwrecks, end cities, buried treasure, outposts, trial chambers, desert wells, end gateways, emerald ore, dungeons) and they are outlined through walls; with enough of them the world seed is printed in chat. Commands are `/seedcracker ...` as in the Java (`render outlines`, `finder`, `data`, `cracker`, `version`, `database`, `gui`), and the settings are on the mod's page in Options > Mods. PARITY.md in the source lists every Java file and what became of …

- Autores: luhnox (this port); SeedCrackerX by 19MisterX98 and the SeedCrackerX contributors; the SeedFinding libraries by KaptainWutax and Neil and Matthew Bolan · Licencia: MIT · Fuente: https://github.com/luhnox/horizon-seedcrackerx
- Tipo: mixin (parchea el código del juego) · versión 0.2.3 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: —

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::debug_renderers::Game<'_>::debug_renderer_gizmos`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
