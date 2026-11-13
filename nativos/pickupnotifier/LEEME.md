# Pick Up Notifier (`pickupnotifier`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A port of Pick Up Notifier: the items, experience and arrows you pick up, listed in a corner of the screen.

**Descripción del catálogo:** A port of [Pick Up Notifier](https://github.com/Fuzss/pickupnotifier) by Fuzs to Horizon, as a native mod. Items, experience and arrows you pick up show in a list in a corner of the screen; old entries move out and fade, and picking up more of an item adds to its entry.

- Items, experience orbs (with their value) and arrows, each on or off; off in creative if you like; a list of items never shown.
- Entries combined always, never, or except named items; how long they stay, how they move out (sideways, up/down, both or not at all) and fade.
- The item's sprite, its name in its rarity colour or a colour of your choice, the amount as a sprite count, text or both, and how many you now have in y…

- Autores: DeportedToHell · Licencia: MPL-2.0 · Fuente: https://github.com/DeportedToHell/MCRE-Mods/tree/main/pickupnotifier
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: pickup, hud, items, notifications, port

## Cómo funciona
Se engancha en 18 puntos del juego:
- `hook:frame`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_key_released_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `horizon_client::game::native_hooks::Game<'_>::native_client_packet`
- `horizon_client::game::native_hooks::Game<'_>::native_client_tick_post`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_post`
- …

## Qué necesita
- Datos que trae: data/icon.png
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- glue.rs: En este Horizon `orb.value` es i32 (en el que se hizo el mod era u32): E0308 `if` and `else` have incompatible types.

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
