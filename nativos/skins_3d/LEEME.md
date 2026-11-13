# 3D Skins (`skins_3d`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::first_person_arm'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** The outer layer of every player's skin drawn as real 3D pixels.

**Descripción del catálogo:** The outer layer of every player’s skin drawn as real 3D pixels: hat, jacket, sleeves and pants follow the head, arms and legs exactly as the game moves them. Lighter and cleaner than Horizon’s built-in option: about 4x fewer polygons, closed corners, nothing poking through armor, and 3D sleeves in first person. Works for everyone you can see, respects Skin Customization, and switches to the flat layer further away. Client side only.

- Autores: _idom_ · Licencia: MIT · Fuente: https://github.com/umbx-dot/horizon-3d-skins
- Tipo: mixin (parchea el código del juego) · versión 1.1.0 · pide la revisión de Horizon 66f3981f52
- Etiquetas: skins, 3d-skins, players, cosmetic

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::Game<'_>::first_person_arm`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::remote::Game<'_>::skin_3d_model`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
