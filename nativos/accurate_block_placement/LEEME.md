# Accurate Block Placement (`accurate_block_placement`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Hold right click with a block and every new spot you look at gets one, however fast you move.

**Descripción del catálogo:** Placing blocks fast is all about timing: vanilla places a block every four ticks while you hold right click, so moving fast skips spots and moving slowly stacks extra blocks. **Accurate Block Placement** times it for you: hold right click and every *new* spot you look at gets a block at once.

- **A new spot, a block**: bridging, building walls and floors, or sweeping along a row; no spot skipped, however fast you move.
- **No extra blocks**: staring at one spot places nothing more.
- **A single click stays a single block**: right after the click it waits as vanilla does, unless you turn the view.
- **Building backwards**: walking backwards while placing toward yourself places one block per …

- Autores: Lutziano888 · Licencia: MIT · Fuente: https://github.com/Lutziano888/horizon-accurate-block-placement
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon aa27eca326
- Etiquetas: building, block-placement, bridging, port

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::remote::Game<'_>::remote_handle_input`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
