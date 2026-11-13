# Chat Heads Horizon (`chat_heads_rust`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Chat Heads displays player heads in the chat before the players name making it easier to see who is writing

**Descripción del catálogo:** Chat Heads displays player heads in the chat before the players name making it easier to see who is writing. with options being to turn off your own head display 

- Autores: Demerobit · Licencia: MIT · Fuente: https://github.com/Demerobit/Chat-Heads-For-Horizon
- Tipo: mixin (parchea el código del juego) · versión 1.0.2 · pide la revisión de Horizon 4aa6854a88
- Etiquetas: optimization, display

## Cómo funciona
Se engancha en 7 puntos del juego:
- `hook:command`
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
