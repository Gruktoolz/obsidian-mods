# Blockscope (`blockscope`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** The block under your crosshair: its name, id, distance and icon, plus hardness, states and a container's slots.

**Descripción del catálogo:** A Jade-style tooltip for Horizon: the block under your crosshair, in a small box you can place where you
want.

- Its name in your language, its block id and its distance, with its icon.
- Its hardness and the tool it wants, and whether what you hold harvests it.
- Its state properties, a crop's growth, and a container's slots laid out as its own screen has them.
- Its coordinates on request, and a creature's name and health when you look at one.

Nothing shows when you look at no block within reach, while a screen is open, or with the HUD hidden (F1). Ten
switches on its settings page (Options > Mods > Blockscope > Settings) choose what the box shows.

- Autores: Domanffe · Licencia: MIT · Fuente: https://github.com/Domanffe/blockscope
- Tipo: mixin (parchea el código del juego) · versión 0.3.1 · pide la revisión de Horizon 4aa6854a88
- Etiquetas: tooltip, hud, block-info, waila

## Cómo funciona
Se engancha en 5 puntos del juego:
- `hook:hud`
- `hook:mods`
- `hook:settings`
- `hook:settings_click`
- `hook:tick`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
