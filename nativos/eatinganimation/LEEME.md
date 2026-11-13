# Eating Animation (`eatinganimation`)

**Estado:** no compila en Android: content_root con firma antigua

**Qué hace:** While you eat food, or drink a potion or milk, the item in your hand shows its bitten frames.

**Descripción del catálogo:** **Port of Eating Animation for Horizon.** Based on [Eating Animation](https://github.com/Theoness1/EatingAnimation) by Theoness1.

## What it does

While you eat food, or drink a potion or milk, the item in your hand shows its bitten frames as you go.

The frames are built into the mod. No resource pack is needed: the textures and models are compiled into the native
build as a content mod (`mixins/eatinganimation/data/eatinganimation-content.hmod`), and loaded with the game's resources.

To rebuild that content from a frame pack (for example a new version of the Eating Animation textures):

```sh
python3 tools/pack_to_content.py <frame pack (zip or folder)> mixins/eatinganimation/data/eating…

- Autores: sillysoufi · Licencia: MIT · Fuente: https://github.com/soufied/EatingAnimation-Horizon
- Tipo: mixin (parchea el código del juego) · versión 0.2.0 · pide la revisión de Horizon 6d99e14ff0
- Etiquetas: eating, animation, food, port

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::mods::content_root`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client_lib::item_render::ItemRenderer::resolve_with_rule`


## Qué necesita
- Datos que trae: data/eatinganimation-content.hmod
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
