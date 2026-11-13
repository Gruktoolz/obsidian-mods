# Tectonic-Horizon (`tectonic_horizon`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Unofficial native port of Tectonic: mountain ranges, underground rivers and configurable terrain.

**Descripción del catálogo:** Unofficial native Rust port of Tectonic 3.0.1 by Apollo (Apollounknowndev), revision 0761b561062de01b701d2571401f625c31059a9a under CC0-1.0. Not made or endorsed by Apollo or the Horizon project.

Mountain ranges, terrain density recipes, underground rivers and configurable terrain for Horizon 26.3. Source-only native mod: install in the launcher, then Build; runs with the game's account permissions. No Minecraft code or assets are included.

Enable under Options > Mods before opening a world. The mod's Settings page edits terrain and biome configuration. Config: config/tectonic-horizon.json in the game home. Invalid configuration logs a diagnostic and uses defaults. Restart after editing; n…

- Autores: eltanschauung, Apollo (original Tectonic) · Licencia: CC0-1.0 · Fuente: https://github.com/eltanschauung/tectonic-horizon
- Tipo: mixin (parchea el código del juego) · versión 0.3.4 · pide la revisión de Horizon 8dbbde4350
- Etiquetas: terrain, mountains, rivers, world-generation, port

## Cómo funciona
Se engancha en 10 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client_lib::vanillagen::shared_data_with`
- `horizon_core::random::hash::seed_from_hash`
- `horizon_world::worldgen::biome_temperature::BiomeTemperatureSettings::height_adjusted`
- `horizon_world::worldgen::generator_decoration::Decoration::decorate`
- `horizon_world::worldgen::overrides::WorldgenOverrides::apply`
- `horizon_world::worldgen::overrides::WorldgenOverrides::level_height`
- `horizon_world::worldgen::structure::legacy::make_bounding_box`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
