# Server Atlas (`server_browser`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::<Game<'_> as ApplicationHandler>::window_event'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Find and organize Minecraft servers with local notes, favorites, filters, and opt-in discovery.

**Descripción del catálogo:** Server Atlas is a native Rust multiplayer browser for Horizon and Minecraft Java 26.3. It adds search, favorites, recent servers, LAN, reusable filters, and local notes and groups in a Minecraft-style interface.

Online Discover is off until you accept the privacy screen. Modrinth and minecraftserve.rs receive listing requests with public filters. Discover contacts visible servers and a small margin for status checks. Saved addresses, search text, player names, and notes stay local. Feedback and server suggestions save local drafts. The toolbar Ko-fi button opens https://ko-fi.com/foryou41 in your browser.

Install the .hnmod in your instance, build it through the Horizon launcher, and open …

- Autores: shuab · Licencia: MIT · Fuente: https://github.com/Musti3214/Horizon-Server-Browser/tree/8a467654699ca83ebdf901498fe3dc73c46dbe69/mod
- Tipo: mixin (parchea el código del juego) · versión 2.2.1 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: server-list, servers, multiplayer, browser, favorites

## Cómo funciona
Se engancha en 25 puntos del juego:
- `horizon_client::game::<Game<'_> as ApplicationHandler>::window_event`
- `horizon_client::game::control::Game<'_>::control_command`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::pad_menu::Game<'_>::pad_osk_output`
- `horizon_client::game::pad_menu::Game<'_>::pad_typing`
- `horizon_client::game::startup_menu::StartupMenu::title_key`
- `horizon_client_lib::multiplayer::Screen::draw_tooltip`
- `horizon_client_lib::title::Title::click`
- `horizon_client_lib::title::Title::confirm_edit`
- `horizon_client_lib::title::Title::drag`
- `horizon_client_lib::title::Title::drag_field`
- `horizon_client_lib::title::Title::draw_with_items`
- `horizon_client_lib::title::Title::escape`
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
