# Inferium Lab (`inferium_lab`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Grow Inferium on farmland and harvest its essence in Horizon 26.3.

**Descripción del catálogo:** An experimental native crop mod inspired by [Mystical Agriculture](https://github.com/BlakeBr0/MysticalAgriculture) by BlakeBr0.

Plant Inferium Seeds on ordinary farmland. A young crop drops one seed. At age 7 it drops one seed and one Inferium Essence. Ordinary bone meal is refused.

Build the instance with this native mod in Horizon Launcher. In a new world, use `/give @s inferium_lab:inferium_seeds` to get the first seeds. Tested on Horizon 26.3 Nightly `66f3981` with Horizon Mixins `e8cd0aa`. The bundle credits the MIT-licensed reference in its `NOTICE`.

- Autores: pehe1 · Licencia: MIT · Fuente: https://github.com/pehe1/horizon/tree/64587f988382bc4b2d69d5d0c00fd6668a40b0fc
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 66f3981f52
- Etiquetas: agriculture, crops, inferium

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_blocks`
- `horizon_client::game::native_hooks::Game<'_>::native_data_pack_files`
- `horizon_client::game::native_hooks::Game<'_>::native_items`
- `horizon_client::game::native_hooks::GameLevel<'_>::native_is_valid_bonemeal_target`
- `horizon_client_lib::mod_content::J::rename`
- `horizon_client_lib::mod_content::Plan<'_>::override_gameplay`


## Qué necesita
- Datos que trae: data/inferium_crop.json
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
