# Armor HUD (`armorhud`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Your worn armor and its durability in a column left to the off-hand slot.

**Descripción del catálogo:** Your worn armor and its durability in a column of four slots left of the hotbar's off-hand slot, as a
native mod for Horizon.

- **Durability numbers**: press **K** (rebindable in Options > Mods) to show or hide them; a notice above the
  hotbar says which. Off by default.
- **Low-durability alerts**: a piece about to break pulses so you notice it in time. On by default.

Both switches are also on the mod's settings page (Options > Mods > Armor HUD).

- Autores: Thomas9950 · Licencia: MIT · Fuente: https://github.com/Thomas-Slot/horizon-armor-hud
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon c9d474dd91
- Etiquetas: hud, armor, durability

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: numbers → k
- Teclas fijas en el código: k · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
