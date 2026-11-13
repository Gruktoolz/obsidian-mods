# Rider HUD (`rider_hud`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Improves the Minecraft HUD while riding mounts, keeping hunger, mount health, and experience visible.

**Descripción del catálogo:** Rider HUD improves the vanilla Minecraft interface while riding horses and other mounts.

**Features:**
- Displays player hunger and mount health at the same time.
- Keeps the experience bar visible while riding.
- Shows the jump bar when charging a jump.
- Preserves the familiar vanilla Minecraft HUD style.
- Includes in-game settings to toggle features.

Lightweight, client-side native mod built in Rust for Horizon.

- Autores: Xariami · Licencia: MIT · Fuente: https://github.com/maliferen/horizon-rider-hud
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: hud, mounts, horse, riding, utility, client-side

## Cómo funciona
Se engancha en 5 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_gui_layer_pre`
- `horizon_client::game::native_hooks::Game<'_>::native_health_hud`
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
