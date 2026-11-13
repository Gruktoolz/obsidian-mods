# Distant Horizons for Horizon Client (`distant_horizons`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::terrain_distance'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** See far past your render distance: Distant Horizons' level-of-detail terrain, ported to Rust as a native mod.

**Descripción del catálogo:** An unofficial Rust port of [Distant Horizons](https://gitlab.com/distant-horizons-team/distant-horizons) by James Seibel and contributors: the world past your render distance, drawn at lower detail the farther it is.

- Far terrain built from the chunks you visit, and in single player from the world generator, out to 64–1024 chunks.
- Detail drops with distance by Distant Horizons' own rule, with its five quality presets.
- LODs are kept on disk per world, so they load at once next time.
- Drawn by Horizon's far-terrain renderer with block textures and biome tints.

Settings: Options > Mods > Distant Horizons. In chat: `/dh` for status, `/dh distance 512`, `/dh quality high`, `/dh debug on` …

- Autores: denizsahin7744, Deniz7744 · Licencia: LGPL-3.0-only · Fuente: https://github.com/denizsahin7744/distant-horizons-for-horizon-client
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon d009ebd140
- Etiquetas: lod, render-distance, far-terrain, port

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::Game<'_>::terrain_distance`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
