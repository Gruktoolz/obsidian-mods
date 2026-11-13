# Mouse Tricks (`mousetricks`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::inventory_drag_move'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Inventory mouse tricks: drag to spread or gather, the wheel moves items one by one, and R sorts.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

- **Right-drag**: one item into each slot you pass.
- **Left-drag**: gather the same items; with Shift, quick-move many stacks.
- **Mouse wheel** over a slot: move items one at a time between your inventory and a container.
- **Sort** (R, or middle click on an empty spot): merges partial stacks and sorts the part under the mouse (a chest, or your main inventory; the hotbar only if you ask). Creative-tab ord…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/mousetricks
- Tipo: mixin (parchea el código del juego) · versión 0.2.0 · pide la revisión de Horizon 142388e1c1
- Etiquetas: inventory, mouse, sorting, drag, quick-move

## Cómo funciona
Se engancha en 9 puntos del juego:
- `horizon_client::game::Game<'_>::inventory_drag_move`
- `horizon_client::game::Game<'_>::inventory_mouse_release`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::tweaks::Game<'_>::sort_mouse_press`
- `horizon_client::game::tweaks::Game<'_>::tweak_scroll`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: r · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
