# Status Effect HUD (`status_effect_hud`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** 1:1 Status Effect Bars & Inventory HUD+ Potion HUD: displays active potion/beacon effects with icons, Roman numeral amplifiers, color-coded mm:ss countdown time

**Descripción del catálogo:** A 1:1 native Rust port of Status Effect Bars and Inventory HUD+ (Potion HUD) for Horizon. Renders all active potion and beacon status effects directly on the HUD during gameplay with their official mob_effect sprites, translated effect names, and Roman numeral potency levels (II, III, IV). Includes color-coded mm:ss countdown timers (White for >30s, Yellow for 10-30s, Flashing Red for <=10s urgent expiration warning, and Aqua infinity symbol for permanent effects) alongside smooth duration progress bars colored in each effect's authentic vanilla potion color.

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/status-effect-hud
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: hud, status-effects, potions, timers, port

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
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
