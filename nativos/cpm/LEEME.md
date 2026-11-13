# CustomPlayerModels-Horizon (`cpm`)

**Estado:** no encaja: E_MANIFEST: TOML parse error at line 14, column 2
   |
14 | [api]
   |  ^^^
unknown field `api`, expected one of `mod`, `compatibility`, `injection`, `bridge`, `module`, `state`, `crate`, `post_build`


**Qué hace:** Customize your Minecraft avatar with custom models and animations

**Descripción del catálogo:** #For version "Nightly 2026-11-01 c9d474d"
CustomPlayerModels-Horizon allows you to use custom player models and animations directly in Minecraft. Customize your avatar with unique shapes, extra parts, and glowing textures. 

## How to use
go to options > mods > CPM-settings and add some models, and load it onto your character. when in game you can also use gestures with G and in the same menu pick different models

- Autores: Scrapper1824, Claude · Licencia: MIT · Fuente: https://github.com/Scrapper1824/CustomPlayerModels-Horizon
- Tipo: trait (impl Hooks) · versión 0.1.2 · pide la revisión de Horizon 02c8607ee8
- Etiquetas: cpm, player-models, skin, cosmetics, animations, avatar, customization, model, gestures

## Cómo funciona
Se engancha en 15 puntos del juego:
- `hook:frame`
- `hook:key`
- `hook:mods`
- `hook:player_render_listened`
- `hook:player_submit`
- `hook:screen_char_typed_pre`
- `hook:screen_key_pressed_pre`
- `hook:screen_mouse_button_pressed_pre`
- `hook:screen_mouse_button_released_pre`
- `hook:screen_mouse_dragged_pre`
- `hook:screen_mouse_scrolled_pre`
- `hook:screen_render_post`
- `hook:settings`
- `hook:settings_click`
- …

## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: g, v · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
