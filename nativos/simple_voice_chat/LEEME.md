# Simple Voice Chat client (unofficial) (`simple_voice_chat`)

**Estado:** compila en Android junto con el lote (aún sin probar en el teléfono)

**Qué hace:** Client-side voice chat: mic capture, Opus, positional mixing, push-to-talk, groups. Speaks Simple Voice Chat 2.6.x protocol (compat version 20).

**Descripción del catálogo:** **A native Horizon mod**: the launcher builds it into the game (install it on the launcher's Mods page, then press Build there). Native mods are not sandboxed and run with your account's rights, so read the source before you build it. Pinned to the Horizon tree this was built against.

- **Mic capture + playback** through the platform's audio stack, Opus encode/decode (libopus vendored, Xiph BSD).
- **UDP voice** with positional mixing, so players hear each other from where they stand.
- **Push-to-talk**, voice **groups**, and activation controls.
- Speaks the **Simple Voice Chat 2.6.x** protocol (compatibility version 20). Tested so far only against Horizon's own voicechat-server, not yet a…

- Autores: sparticle62ops · Licencia: MIT · Fuente: https://github.com/Sparticle62ops/horizon-simple-voice-chat
- Tipo: mixin (parchea el código del juego) · versión 1.0.0 · pide la revisión de Horizon 3fb4c3138e
- Etiquetas: voice, voicechat, opus, multiplayer

## Cómo funciona
Se engancha en 6 puntos del juego:
- `hook:command`
- `hook:frame`
- `hook:input_key`
- `hook:net_from_server`
- `hook:player_submit`
- `hook:world_lifecycle`


## Qué necesita
- Datos que trae: data/opus/celt/.deps/opus_custom_demo.Po, data/opus/celt/_kiss_fft_guts.h, data/opus/celt/arch.h, data/opus/celt/arm/arm2gnu.pl, data/opus/celt/arm/arm_celt_map.c, data/opus/celt/arm/armcpu.c, data/opus/celt/arm/armcpu.h, data/opus/celt/arm/armopts.s, data/opus/celt/arm/celt_fft_ne10.c, data/opus/celt/arm/celt_mdct_ne10.c, data/opus/celt/arm/celt_neon_intr.c, data/opus/celt/arm/celt_pitch_xcorr_arm-gnu.S
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
