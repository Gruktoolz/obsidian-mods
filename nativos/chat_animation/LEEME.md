# ChatAnimationHorizon (`chat_animation`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** ChatAnimation for Horizon

**Descripción del catálogo:** Smooth chat for Horizon, recreating Ezzenix's Chat Animation mod as a native mod.

- **New messages slide up**: the chat starts one line lower and eases into place while the new message fades in.
- **The input field slides up** from the bottom of the screen when you open the chat.
- **The indicator line** (the coloured bar left of some messages) can be hidden.

**Settings**: "Chat Animation Settings..." on the Options screen, or Options > Mods > Chat Animation: animate
messages (150 ms by default), hide the indicator line, animate opacity, animate the input field (170 ms), each with
a reset button.

Based on Ezzenix's Chat Animation (MIT), recreated for Horizon.

- Autores: tariksabanuz · Licencia: MIT · Fuente: https://github.com/tariksabanuz/ChatAnimationHorizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.3 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: chat, animation, ui, port

## Cómo funciona
Se engancha en 14 puntos del juego:
- `hook:frame`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client::game::options::Game<'_>::draw_options`
- `horizon_client::game::options::Game<'_>::options_drag`
- `horizon_client::game::options::Game<'_>::options_key`
- `horizon_client::game::options::Game<'_>::options_mouse`
- `horizon_client::game::options::Game<'_>::options_scroll`
- `horizon_client_render::chat::Chat::layout`
- `horizon_client_render::chat::ChatScreen::draw`
- `horizon_client_render::chat::ChatScreen::new`
- `horizon_client_render::gfx::reuse::On<'_>::create_shader_module`
- `horizon_client_render::options_ui::OptionsUi::layout`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
