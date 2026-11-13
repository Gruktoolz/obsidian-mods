# Serverify (`serverify`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Find Minecraft servers and add them to your Multiplayer list, from a page in your web browser.

**Descripción del catálogo:** Serverify is a server browser. Press **F8** (or type `/serverify web`, or press the button on the mod's settings page) and your web browser opens a page with a search box, a category menu and cards showing each server's picture, address, player count and country. Press **Add** and the server goes to the top of your Multiplayer list. Then join it from Horizon's own Multiplayer screen.

It is the Horizon version of [Serverify](https://github.com/Louchatfroff/serverify), the server browser for Fabric, Quilt, Forge and NeoForge, by the same author.

## What it does

- Opens a server browser page in your web browser: search, 22 categories, an offline-mode filter and an **Add** button.
- Adds serv…

- Autores: Louchatfroff · Licencia: GPL-3.0-only · Fuente: https://github.com/Louchatfroff/serverify
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 6f8c7a0e5d
- Etiquetas: server-list, servers, multiplayer, browser

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: open → f8
- Teclas fijas en el código: f8 · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
