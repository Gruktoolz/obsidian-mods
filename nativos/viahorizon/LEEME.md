# ViaHorizon (`viahorizon`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::hand_geometry'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Connect to older Minecraft servers using Horizon 26.3.

**Descripción del catálogo:** Connect Horizon 26.3 to Minecraft Release servers from 1.7.2 through 26.3: an unofficial Rust port of
[ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus), as a native mod.

- In Options > Mods > ViaHorizon, choose **Auto** or the version your server runs.
- It translates the network packets between Horizon and the server; signing in stays Horizon's own.
- It saves its settings and checks the servers you pick.

A native source mod: it runs with the game's permissions. Upstream authors and derived protocol data are credited in
its third-party notices.

- Autores: hhhhsffd · Licencia: GPL-3.0-or-later · Fuente: https://github.com/hhhhsffd/viahorizon
- Tipo: mixin (parchea el código del juego) · versión 0.8.8-dev · pide la revisión de Horizon fdc87aecdb
- Etiquetas: multiplayer, protocol, old-servers, viafabricplus, port

## Cómo funciona
Se engancha en 37 puntos del juego:
- `horizon_client::game::Game<'_>::hand_geometry`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_use_animation`
- `horizon_client::game::native_hooks::Game<'_>::native_use_duration`
- `horizon_client::game::remote::Game<'_>::decorate_chat`
- `horizon_client::game::remote::Game<'_>::end_predictions`
- `horizon_client::game::remote::Game<'_>::held_until_released`
- `horizon_client::game::remote::Game<'_>::join_server`
- `horizon_client::game::remote::Game<'_>::remote_menu_ops`
- `horizon_client::game::remote::Game<'_>::remote_send_state`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
