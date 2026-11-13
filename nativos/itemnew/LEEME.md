# Item New (`itemnew`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Newly picked-up items glow in your hotbar and inventory until you look at them.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

Items you just got (picked up, crafted, taken from a chest, given) get a soft glow in the hotbar and in inventory and container screens, until you point at them or close the screen.

- Styles: glow, glow with a star, by rarity colour.
- Which screens, and when a mark clears, are settings; `/itemnew` shows or clears the marks.

Settings: Options → Mods → Item New. Inspired by [Item Highlighter](https://modri…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/itemnew
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon 142388e1c1
- Etiquetas: inventory, highlight, new-items, hotbar

## Cómo funciona
Se engancha en 6 puntos del juego:
- `horizon_client::game::item_draw::Game<'_>::draw_item_model`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::native_hooks::Game<'_>::native_tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
