# RightClickHarvest (`rightharvest`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::use_hand'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Right-click a ripe crop to pick it and sow it again.

**Descripción del catálogo:** With an empty hand, right-click a ripe wheat, carrot, potato, beetroot or nether wart: the crop is harvested with the game's own drops, sound and statistics, and sown again from a seed in your inventory.

- Autores: Domanffe · Licencia: MIT · Fuente: https://github.com/Domanffe/rightharvest
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon 4aa6854a88
- Etiquetas: farming, crops, harvest, replant

## Cómo funciona
Se engancha en 2 puntos del juego:
- `hook:mods`
- `horizon_client::game::Game<'_>::use_hand`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
