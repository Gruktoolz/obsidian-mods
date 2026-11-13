# Hitbox+ (`hitbox_plus`)

**Estado:** no compila en Android: falta crate::official_hitbox_plus

**Qué hace:** Customizable entity hitboxes with exact weapon & spear in-reach colors, thickness, eye rays, and sprint HUD.

**Descripción del catálogo:** Hitbox+ turns the game's F3+B debug hitboxes into a customizable PvP aid, as a native mod for Horizon.

- **In-reach colour**: a hitbox takes your chosen colour while your crosshair is on it within hitting distance:
  3 blocks for swords, axes, maces and fists, 4.5 blocks for spears in survival (6.5 in creative).
- **11 reach colours**, from bright green to a smooth rainbow, and hitbox styles (white, cyan, yellow, red, rainbow).
- **Three thicknesses**: thin, default or thick lines.
- **Eye rays**: optional looking-direction lines and eye-level markers.
- **Sprint HUD**: a sprint indicator in the screen corner you pick.

Press **H** to show or hide hitboxes. Colours, thickness and the HUD's …

- Autores: taxtii, itzglecni · Licencia: MIT · Fuente: https://github.com/taxtii/hitbox_plus.git
- Tipo: mixin (parchea el código del juego) · versión 1.3.0 · pide la revisión de Horizon 68ac6f647e
- Etiquetas: hitboxes, pvp, reach, combat, sprint, hud

## Cómo funciona
Se engancha en 7 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_key`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::gizmos::Gizmos::entity_box`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: h · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
