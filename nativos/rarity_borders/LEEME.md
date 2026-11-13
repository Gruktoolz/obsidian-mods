# Rarity Borders (`rarity_borders`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Adds clean, vanilla-style colored borders to inventory items based on their rarity.

**Descripción del catálogo:** Rarity Borders makes item rarity easier to recognize with subtle, pixel-perfect borders inspired by Minecraft's vanilla interface.

**Features:**
- Colored borders for Uncommon, Rare, and Epic items.
- Clean square outlines with a highlighted bottom edge.
- Works with inventory and container slots.
- Optional borders for Common items and the hotbar.
- In-game settings to toggle features.
- Lightweight, client-side native Rust mod.

- Autores: Xariami · Licencia: MIT · Fuente: https://github.com/maliferen/horizon-rarity-borders
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon 2ebf00a028
- Etiquetas: rarity, borders, inventory, items, equipment, vanilla, client-side

## Cómo funciona
Se engancha en 4 puntos del juego:
- `horizon_client::game::native_hooks::Game<'_>::native_item_decorations`
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
