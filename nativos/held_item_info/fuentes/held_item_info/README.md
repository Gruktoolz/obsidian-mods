# Held Item Info for Horizon

An original MIT native HUD implementation for Minecraft 26.3 / Horizon. Tested on
3db53df3bde39da07aabddec9a5fa850204fd32a with Horizon Mixins
07dab17387480ac03829584d9d1ee9912c9bb515.

Shows extra details above the hotbar for the currently held item:

- Remaining / maximum durability.
- Stack count when greater than one.
- Up to six enchantments, using the game's translations.
- Long text is shortened to the current GUI width.

Client side only. No network access, account access or file writes. The line is
shown while an item is held; potion details, configurable layouts and a fade timer
are not implemented. This is a minimal initial implementation, not complete Java
mod parity. Inspired by [Held Item Info](https://github.com/A5b84/held-item-info),
not affiliated. No code from that LGPL project was read or incorporated.

Armor Status in the registry shows equipped-item icons/durability; this mod adds
a text line including enchantments. Do not install it merely to duplicate a HUD
layout you already use.

Validation details are in VALIDATION.md. Build instructions are in tools/build.py.

## Build

Clone the MIT Horizon and horizon-mixins repositories. Build the latter at the
exact commit in compatibility.json. Then:

```sh
python3 tools/build.py check --horizon /path/to/horizon --mixins-cli /path/to/horizon-mixins --target-dir /path/to/cargo-target
python3 tools/build.py package
```

The build exports the committed Horizon revision and applies the native mixins to
a temporary source tree. It does not launch the game or read player saves.
Development assisted by Codex.
