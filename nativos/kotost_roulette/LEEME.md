# Kotost Roulette (`kotost_roulette`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Stake game items, spin a wheel, win a dearer item or lose the stake. Every item has a price in points; Russian and English.

**Descripción del catálogo:** Open the Roulette (every player gets the item on joining or respawning, or `/roulette`), drag a stack from your inventory onto the left panel and pick the reward: **x2, x4, x5, x8, x10**, a chance (**30 %, 50 %, 70 %**), or any item from the searchable catalog. The chance is shown before the spin: 0.996 / multiplier (x2 is 49.8 %). The ring turns for three seconds, slows down, clicks over the sectors, and plays a win or a loss sound.

The roll is the server's: it takes the stake the moment it rolls, so leaving mid-spin never returns or duplicates anything. A reward goes to your inventory, onto the ground when it is full, or into your saved inventory if you left. Game items only: no money, no…

- Autores: Kotost Games · Licencia: MIT · Fuente: https://github.com/Leks2000/kotost-roulette
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: items, wheel, minigame, luck

## Cómo funciona
Se engancha en 14 puntos del juego:
- `hook:net_from_client`
- `hook:net_from_server`
- `hook:perform_command`
- `hook:player_logged_in`
- `hook:player_respawned`
- `hook:register_commands`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `hook:tick`
- `hook:use_item`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
