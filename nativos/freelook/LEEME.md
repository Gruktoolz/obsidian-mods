# Freelook (`freelook`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::apply_mouse_look'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Hold a key for a third person camera you can look around in

**Descripción del catálogo:** Hold **X** (rebindable in Options > Mods) for a third-person view you can look around in, while your
character keeps walking and facing the way it was. Let go and the camera goes back to what it was.

Scroll while holding the key to move the camera closer or farther, from 1 to 24 blocks (the game's own third person
is 4). A native mod for Horizon, in the spirit of the Freelook and Perspective mods.

- Autores: Krzysix17 · Licencia: MIT · Fuente: https://github.com/Krzysix17/horizon-freelook
- Tipo: mixin (parchea el código del juego) · versión 0.3.1 · pide la revisión de Horizon a491181961
- Etiquetas: freelook, perspective, camera, third-person

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::Game<'_>::apply_mouse_look`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_wheel`
- `horizon_client::game::perspective::Game<'_>::camera_view`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: x · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
