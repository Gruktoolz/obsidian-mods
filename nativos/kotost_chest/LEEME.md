# Kotost Chest (`kotost_chest`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Two buttons beside a chest's sort button: everything into the chest, everything out of it.

**Descripción del catálogo:** Two buttons beside a chest's sort button: **arrow up** moves your whole main inventory into the chest (the hotbar stays), **arrow down** moves everything out of the chest into your inventory.

Each button is a run of ordinary Shift-clicks, so it works on any server, vanilla included, and nothing is needed on the server. Needs Horizon's sort buttons on (`sort:true` in `horizon-extras.txt`).

Native source mod: the launcher compiles it after a warning; it runs with your account's permissions. No Minecraft code or assets are included.

- Autores: Kotost Games · Licencia: MIT · Fuente: https://github.com/Leks2000/kotost-chest
- Tipo: mixin (parchea el código del juego) · versión 0.1.0 · pide la revisión de Horizon fdc87aecdb
- Etiquetas: chest, inventory, quality-of-life

## Cómo funciona
Se engancha en 2 puntos del juego:
- `horizon_client::game::tweaks::Game<'_>::draw_sort_buttons`
- `horizon_client::game::tweaks::Game<'_>::sort_press`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
