# X To Xray (`xtoxray`)

**Estado:** no compila en Android: MeshTables: campos nuevos sin inicializar

**Qué hace:** Opensource Xraymod

**Descripción del catálogo:** Press **X** to see ores through the world, as a native mod for Horizon.

While it is on, every block except ores (coal, copper, iron, gold, redstone, lapis, diamond, emerald, their
deepslate kinds, nether gold and quartz, ancient debris), chests, spawners, water and lava is drawn as air, and what is left shines at full brightness. Press X again to see the world as usual. Rebind the key in
Options > Mods.

Single player only: it turns itself off as soon as the world is no longer a closed single player world, so it does
not work on servers or in a world opened to LAN.

- Autores: 1_st_Cat · Licencia: Apache-2.0 · Fuente: https://github.com/RealCat1st/XToXrayRust
- Tipo: mixin (parchea el código del juego) · versión 2026.29.0 · pide la revisión de Horizon 942e3b2ded
- Etiquetas: xray, ores, mining

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client_render::mesher::MeshTables::new`
- `horizon_client_render::mesher::Mesher<'_>::mesh`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
