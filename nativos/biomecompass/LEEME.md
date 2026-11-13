# BiomeCompass (`biomecompass`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::craft_recipe'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Craft a Biome Compass, pick a biome and its needle points to the nearest one.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

Craft a **Biome Compass** (a compass with a sapling on each of its four sides, any kinds), right-click it and pick a biome from the list (with a search box). The world generator is searched around you, a little every tick, and the compass's needle then points to the nearest one, with its distance and direction on the action bar.

- Works in single player, on LAN and on a server running the native build, for…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/biomecompass
- Tipo: mixin (parchea el código del juego) · versión 0.2.2 · pide la revisión de Horizon 142388e1c1
- Etiquetas: compass, biomes, exploration, navigation

## Cómo funciona
Se engancha en 11 puntos del juego:
- `horizon_client::game::Game<'_>::craft_recipe`
- `horizon_client::game::lan_actions::Game<'_>::guest_command`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_hud`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`
- `horizon_client::game::potions::Game<'_>::use_potion_item`
- `horizon_client::lan::hub::Hub::set_mod_commands`
- `horizon_client::menu::Menu::recompute`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
