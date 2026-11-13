# Context_tool_swap (`context_tools`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::play_mouse'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** Classic Tool swap

**Descripción del catálogo:** Classic Toolswap for Rust. Just press middle click when you have a tool in the hand. It will swap the right tool out.

- Autores: TTCjonasTTC, Qwen · Licencia: MIT · Fuente: https://github.com/TTCjonasTTC/context_toolswap
- Tipo: mixin (parchea el código del juego) · versión 1.3.0 · pide la revisión de Horizon 6475f62f3c
- Etiquetas: —

## Cómo funciona
Se engancha en 2 puntos del juego:
- `horizon_client::game::Game<'_>::play_mouse`
- `horizon_client::game::commands::Game<'_>::run_command`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
