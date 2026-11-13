# Wayfarer Stones (`wayfarer_stones_native`)

**Estado:** no compila en Android: content_root con firma antigua

**Qué hace:** Place stones around your world, activate them, and travel between the ones you know.

**Descripción del catálogo:** **This mod has two parts, and you need both.** Install this one and [Wayfarer Stones (assets)](https://playhorizon.dev/mods/wayfarer_stones), then press Build.

- **Place** a stone, or find one in a village. Right-click it once to add it to your list.
- **Travel** from any stone you know: pick a destination on the screen, or search for it. You land beside the stone, on solid ground.
- **Pay** in experience levels: 1, plus 1 for each 500 blocks, up to 5. Travel between dimensions costs 3.
- **Carry** a Wayfarer's Scroll to travel from anywhere. Each travel uses one up.
- **Stones are safe:** only the owner or an operator can break one, and pistons won't move it.

Every world has its own setti…

- Autores: bioneisme · Licencia: MIT · Fuente: https://github.com/Bioneisme/wayfarer-stones
- Tipo: mixin (parchea el código del juego) · versión 0.6.3 · pide la revisión de Horizon 67dad0d1a5
- Etiquetas: teleport, waystones, fast-travel, villages

## Cómo funciona
Se engancha en 25 puntos del juego:
- `horizon_client::game::mods::content_root`
- `horizon_client::game::native_hooks::Game<'_>::native_blocks`
- `horizon_client::game::native_hooks::Game<'_>::native_destroy_block`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_items`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_net_from_client`
- `horizon_client::game::native_hooks::Game<'_>::native_net_from_server`
- `horizon_client::game::native_hooks::Game<'_>::native_perform_command`
- `horizon_client::game::native_hooks::Game<'_>::native_place_block`
- `horizon_client::game::native_hooks::Game<'_>::native_player_logged_in`
- `horizon_client::game::native_hooks::Game<'_>::native_register_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_right_click_block`
- `horizon_client::game::native_hooks::Game<'_>::native_screen_char_typed_pre`
- …

## Qué necesita
- Datos que trae: data/wayfarer_stones-content.hmod
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: e · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
