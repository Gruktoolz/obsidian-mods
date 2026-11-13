# Cross Version (`crossversion`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Allows version hopping offline.

**Descripción del catálogo:** Join servers that speak an older Minecraft protocol from Horizon (which speaks only 26.3's), as a native
mod.

When you join, it asks the server which version it speaks. If it isn't 26.3, it starts a
[ViaProxy](https://github.com/ViaVersion/ViaProxy) on a loopback port and sends the join through it: the proxy speaks
26.3 to Horizon and the server's own version to the server. A join to a 26.3 server starts nothing.

- **Offline-mode servers only**: online-mode servers can't be joined.
- The server list shows such servers as joinable, with their own version, instead of "Incompatible version!".
- **Edit Server** and **Direct Connect** get a line to choose the version yourself (auto, 1.8 to 1.21…

- Autores: JustKarkat · Licencia: MIT · Fuente: https://github.com/JustKarkat/horizon-crossversion/
- Tipo: mixin (parchea el código del juego) · versión 0.8.0 · pide la revisión de Horizon 07b7ea30d0
- Etiquetas: multiplayer, viaproxy, protocol, old-servers

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::multiplayer::Screen::click`
- `horizon_client::multiplayer::Screen::draw_tooltip`
- `horizon_client::net::Session::connect`
- `horizon_client::net::Session::connect_transferred`
- `horizon_client::net::Session::disconnect`
- `horizon_client::net::ping`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
