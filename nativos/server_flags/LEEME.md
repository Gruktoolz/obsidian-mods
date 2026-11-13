# Server Flags (`server_flags`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A flag beside each server in the Multiplayer list for the country it's hosted in

**Descripción del catálogo:** A small flag beside each server in your Multiplayer list for the country it's hosted in. Hover the flag for the country's name, and pick servers close to you for a better ping at a glance.

- **Pixel-art flags** after each server's name, or on its icon when the name is long.
- **67 drawn flags**, the countries servers are most often hosted in; any other country shows its two-letter code.
- **Country name on hover**, like "Germany (DE)".
- Servers on your own network are never looked up.

Settings in Options > Mods > Server Flags: lookups on/off, flag position, hover tooltip, codes for countries without a flag, reset.

- A grey **?** marks a server whose location can't be shown; hover it for …

- Autores: selfmonoraque5 · Licencia: MIT · Fuente: https://github.com/selfmonoraque5/server-flags
- Tipo: mixin (parchea el código del juego) · versión 1.0.1 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: servers, multiplayer, flags, country, client-side

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::multiplayer::Screen::draw_tooltip`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
