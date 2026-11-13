# Enchantment Hints (`enchantmenthints`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** A short description under every enchantment in item tooltips, in English and Russian.

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to Horizon 142388e and horizon-mixins 0d6b103.

Every enchantment on books and gear gets a one-line description in its tooltip: what it does, in plain words. Our own text for every 26.3 enchantment, in English and Russian.

- Always, or only while Shift is held (with a reminder line).
- Colour and line width are settings.

Settings: Options → Mods → Enchantment Hints. Inspired by [Enchantment Descriptions](https://modrinth.com/mod/enchantment-description…

- Autores: d3grando · Licencia: MIT · Fuente: https://github.com/d3grando/enchantmenthints
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon 142388e1c1
- Etiquetas: enchantments, tooltip, descriptions

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::stations::Game<'_>::stack_tooltip`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
