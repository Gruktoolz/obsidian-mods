# Invisible Frames (`invisible_frames`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Sneak-right-click a filled frame to hide and lock it, or restore it.

**Descripción del catálogo:** Hide the frame of a filled item frame, so only the item shows, as a native mod for Horizon.

- **Sneak and right-click** a filled item frame or glow item frame: its frame disappears and it is locked, so ordinary
  right-clicks no longer turn or take the item.
- **Sneak and right-click it again** to show the frame and unlock it.
- Empty frames stay visible.

It runs where the world is hosted (single player, or a world you open to LAN) and uses the game's own entity data, so
players who join need nothing installed. It is not a protection system: breaking the block behind a frame, and
creative mode, work as usual. Inspired by [InvisibleFrames](https://github.com/ADHDMC/InvisibleFrames); an orig…

- Autores: AltMugen · Licencia: MIT · Fuente: https://github.com/AltMugen/horizon-invisible-frames
- Tipo: mixin (parchea el código del juego) · versión 0.1.1 · pide la revisión de Horizon 3db53df3bd
- Etiquetas: item-frames, invisible, building, server-side

## Cómo funciona
Se engancha en 3 puntos del juego:
- `horizon_client::game::lan_actions::Game<'_>::enter_guest`
- `horizon_client::game::native_hooks::Game<'_>::native_interact_entity`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
