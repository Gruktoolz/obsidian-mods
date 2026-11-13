# MineLights (`minelights`)

**Estado:** no encaja: E_TARGET_NOT_FOUND: no source function 'horizon_client::game::Game<'_>::save_for_exit'; use an exact crate::module::function, Type::method or <Type as Trait>::method ID

**Qué hace:** MineLights is a Minecraft mod that brings RGB lighting integrations to Minecraft Java Edition and Horizon.

**Descripción del catálogo:** MineLights is a Minecraft Fabric, Quilt, Forge, NeoForge and Horizon mod that brings your world to life with dynamic RGB lighting effects for your peripherals. Effects respond to your environment, weather and player status; keyboard lighting can also display health, hunger, experience and a compass.

Configure MineLights through Horizon's Mods menu. OpenRGB devices and Yeelight bulbs can connect directly. Other supported RGB devices use [MineLights Server](https://github.com/megabytesme/MineLights-Server).

MineLights connects to local RGB services and may discover Yeelight bulbs on the local network. MineLights Server uses encrypted, authenticated connections and supports optional pairing w…

- Autores: MegaBytesMe · Licencia: CC-BY-NC-SA-4.0 · Fuente: https://github.com/megabytesme/MineLights/tree/246c37d4c0c29832fb66236a366265e20a0c70f2/horizon
- Tipo: mixin (parchea el código del juego) · versión 2.4.0 · pide la revisión de Horizon 67dad0d1a5
- Etiquetas: —

## Cómo funciona
Se engancha en 8 puntos del juego:
- `horizon_client::game::Game<'_>::save_for_exit`
- `horizon_client::game::native_hooks::Game<'_>::native_command`
- `horizon_client::game::native_hooks::Game<'_>::native_commands`
- `horizon_client::game::native_hooks::Game<'_>::native_frame`
- `horizon_client::game::native_hooks::Game<'_>::native_mods`
- `horizon_client::game::native_hooks::Game<'_>::native_settings`
- `horizon_client::game::native_hooks::Game<'_>::native_settings_click`
- `horizon_client::game::saving::Game<'_>::begin_closing_save`


## Qué necesita
- Datos que trae: ninguno
- Otros mods que menciona/necesita: ninguno detectado
- Teclas reasignables: ninguna
- Teclas fijas en el código: ninguna · Ratón extra: ninguno

## Parches míos
- ninguno

## Cómo probarlo
Ver `prueba.json` (lo lee `obsidian/probar_mod.py`, aún por escribir): instalar, lanzar un mundo creativo, inyectar las teclas de arriba y comprobar que no hay panic/ANR/errores en el registro y, cuando se sepa, el efecto esperado.
