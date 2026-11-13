# Jade (`jade`)

**Estado:** quitado del lote: error[E0308]: mismatched types

**Qué hace:** The look-at tooltip: what you're pointing at, its mod, its details and its state.

**Descripción del catálogo:** An unofficial port of [Jade](https://github.com/Snownee/Jade) 26.3.5 by Snownee to Horizon: the look-at overlay for blocks and entities, with its providers, themes, config screens, drag-to-place screen, profiles, keys, the `/jade` and `/jadec` commands and the `jade:max_position_deviation` game rule.

Rewritten in Rust for Horizon's hooks, so no original code runs. Not made or endorsed by Snownee, so please report problems to this project, not to them.

**Missing or not the same:**
- Java plugins for Jade can't run. Other mods register with this port's own provider functions.
- The icon style of the energy line shows redstone dust, because Jade's energy sprite isn't in Horizon's atlas.
- The…

- Autores: Snownee, Joeypc6 · Licencia: CC-BY-NC-SA-4.0 · Fuente: https://github.com/Joeypc6/Jade-Horizon
- Tipo: mixin (parchea el código del juego) · versión 26.3.8 · pide la revisión de Horizon 6560641ba5
- Etiquetas: tooltip, hud, block-info, waila, port

## Cómo funciona
Se engancha en 21 puntos del juego:
- `hook:client_resources_reloaded`
- `hook:command`
- `hook:frame`
- `hook:hud`
- `hook:mods`
- `hook:net_from_client`
- `hook:net_from_server`
- `hook:perform_command`
- `hook:perform_console_command`
- `hook:register_commands`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- …

## Qué necesita
- Datos que trae: data/lang/de_de.json, data/lang/en_us.json, data/lang/es_es.json, data/lang/fr_fr.json, data/lang/it_it.json, data/lang/ja_jp.json, data/lang/ko_kr.json, data/lang/nl_nl.json, data/lang/pl_pl.json, data/lang/pt_br.json, data/lang/ru_ru.json, data/lang/tr_tr.json
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: keypad.0, keypad.1, keypad.2, keypad.3, keypad.4, keypad.5, r, u · Ratón extra: key.mouse.left, key.mouse.middle, key.mouse.right

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
