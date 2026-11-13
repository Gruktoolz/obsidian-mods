# Carry Block (`carryblock`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::use_item'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Sneak and right-click with an empty hand to pick up a chest (with its items) or a small mob, carry it and put it down.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

Sneak and right-click with an empty hand to pick up a block with its contents (chests, furnaces, barrels...) or a small, friendly mob, and carry it; right-click to put it down where you want, with everything it held.

- Slowness while carrying; a small HUD box shows the load.
- Never an item: nothing to duplicate. On death the load is put down nearby, or dropped as its item with its contents.
- Follows the …

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/carryblock
- Tipo: mixin (parchea el código del juego) · versión 0.1.2 · pide la revisión de Horizon 142388e1c1
- Etiquetas: carry, chests, pick-up, animals

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::Game<'_>::use_item`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
