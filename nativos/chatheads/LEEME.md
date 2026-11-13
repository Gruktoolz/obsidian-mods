# ChatHeads (`chatheads`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A player's head left of every chat line they wrote.

**Descripción del catálogo:** A player's 8x8 skin face left of every chat line they wrote, as a native mod for Horizon.

- The face comes from the skin the game resolved for that player; with none, a default skin picked by the name, so a
  player always keeps the same head.
- System messages, `/me` lines and wrapped continuations get no head, and the head fades with its line.
- F1 hides it as it hides the chat.

Turn it on or off in Options > Mods > ChatHeads > Settings.

- Autores: Domanffe · Licencia: MIT · Fuente: https://github.com/Domanffe/chatheads
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon 4aa6854a88
- Etiquetas: chat, player-heads, skins

## Cómo funciona
Se engancha en 6 puntos del juego:
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `horizon_client::game::chat::Game<'_>::draw_chat`
- `horizon_client_render::chat::Chat::draw`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
