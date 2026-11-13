# AppleSkin (`appleskin_horizon`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::titles::Game<'_>::draw_titles'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** 1:1 Minecraft Java parity food value preview & saturation HUD overlay.

**Descripción del catálogo:** A port of AppleSkin for Horizon, as a native mod, drawn to match the Java Edition HUD.

- **Saturation overlay**: a golden border and glow around the drumsticks your saturation covers.
- **Food value preview**: pulsing preview drumsticks, and a preview of the saturation border, while you hold
  something you can eat.
- **On or off** at any time in Options > Mods > AppleSkin.

- Autores: CayMac · Licencia: MIT · Fuente: https://github.com/CayMac97/appleskin
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon ba6344a59c
- Etiquetas: hud, hunger, saturation, food, port

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::titles::Game<'_>::draw_titles`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
