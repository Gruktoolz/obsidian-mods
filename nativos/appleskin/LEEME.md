# AppleSkin (`appleskin`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Shows how much HP, hunger, and saturation will be restored.

**Descripción del catálogo:** A port of [AppleSkin](https://github.com/squeek502/AppleSkin) by squeek502 for Horizon, as a native mod. It
draws the hunger details AppleSkin shows over the food bar:

- **Saturation**: how much saturation you have, as food icons over the food bar.
- **Exhaustion**: a thin strip showing how close you are to losing the next food point.
- **Held food preview**: the food in your hand, with how much food and saturation it would restore.
- **Animation**: fade in, pulse, or both (the default).

Each part can be turned on or off, and the animation changed, in the mod's settings (Options > Mods > AppleSkin).

Based on AppleSkin by squeek502. See the LICENSE in the source for the original copyright …

- Autores: sillysoufi · Licencia: MIT · Fuente: https://github.com/soufied/AppleSkin-Horizon
- Tipo: mixin (parchea el código del juego) · versión 0.1.3 · pide la revisión de Horizon 02c8607ee8
- Etiquetas: hud, hunger, saturation, food, port

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_post`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
