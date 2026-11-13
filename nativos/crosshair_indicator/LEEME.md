# Crosshair Indicator (`crosshair_indicator`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Dynamic target reach awareness: visual feedback on your crosshair when hovering over an entity in attack range.

**Descripción del catálogo:** A port of the Java Crosshair Indicator mod (AmerDz01) for Horizon, as a native mod: your crosshair tells you
when the mob or player you aim at is within attack reach (3 blocks).

- **Four styles**: corner brackets around the crosshair (the default), a centre dot, diamond pips, or a chevron above
  it.
- **Colour themes**: classic white, dynamic (green when your weapon is fully charged, orange while it charges), combat
  red, pink, diamond cyan and gold.
- **Cooldown bar**: an optional thin weapon-cooldown bar below the crosshair.

Settings are in Options > Mods > Crosshair Indicator; a key to turn it on or off can be bound there (unbound by
default).

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/crosshair-indicator
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: crosshair, pvp, combat, hud, port

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
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
