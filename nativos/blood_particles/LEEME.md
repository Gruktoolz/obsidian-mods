# Blood Particles (`blood_particles`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Blood particles upon a mob hit

**Descripción del catálogo:** Blood particles upon a mob hit 🩸

The mod is customizable

made by Bogdan_Tokarev

- Autores: Bogdan_Tokarev · Licencia: MIT · Fuente: https://github.com/bogdan1tokarev1/Blood-Particles-Rust
- Tipo: mixin (parchea el código del juego) · versión 1.2.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: —

## Cómo funciona
Se engancha en 6 puntos del juego:
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
