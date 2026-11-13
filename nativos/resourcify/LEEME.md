# ResourcifyHorizon (`resourcify`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Resourcify for Horizon

**Descripción del catálogo:** Find and install resource packs from Modrinth without leaving the game. By tariksabanuz.

A **+** button appears in the top right of the Resource Packs screen. It opens a browser with:

- a search box that searches while you type, a sort button (Relevance, Downloads, Newest, Updated) and a switch between "for this game version" and "all versions";
- a list of packs with pictures, authors and download counts, loaded as you scroll;
- a detail pane with the versions of the selected pack and an **Install** button.

Installing downloads the file from Modrinth's own server (`cdn.modrinth.com` only), checks it against the SHA-1 hash Modrinth provides, and saves it in `resourcepacks/`. Nothing is tu…

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/ResourcifyHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.2 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: resource-packs, modrinth, browser, download

## Cómo funciona
Se engancha en 8 puntos del juego:
- `hook:frame`
- `hook:mods`
- `horizon_client::game::options::Game<'_>::draw_options`
- `horizon_client::game::options::Game<'_>::options_drag`
- `horizon_client::game::options::Game<'_>::options_key`
- `horizon_client::game::options::Game<'_>::options_mouse`
- `horizon_client::game::options::Game<'_>::options_scroll`
- `horizon_client::game::options::Game<'_>::options_shortcut`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
