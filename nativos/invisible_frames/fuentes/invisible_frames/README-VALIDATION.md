# Validation — 2026-11-02

Tested Horizon 3db53df3bde39da07aabddec9a5fa850204fd32a,
Horizon Mixins 07dab17387480ac03829584d9d1ee9912c9bb515, Linux x86-64, Vulkan.

- Native AST apply passed together with the other AltCraft native prototypes.
- The full optimized game and horizon-ctl compiled successfully.
- An actual integrated server was launched with a fresh temporary game home,
  launcher home and XDG folders. No real player worlds were used.
- Shift + ordinary right-mouse input changed a filled item frame to Invisible=1 and Fixed=1,
  verified through the real server's /data get command.
- Repeating the input restored Invisible=0 and Fixed=0.
- Both states were captured in real rendered frames.
- Dedicated servers, protection-mod interoperability and disconnect/restart persistence
  have not been separately exercised in this first release. The engine's persisted frame
  fields are used; there is no separate custom frame save format.

## 0.1.1 — 2026-11-03

- Snapshot comes from `Guest.sneaking` at `Game::enter_guest`, then is selected by
  `native_acting_player().uuid` during interaction. Physical keyboard keys are not read.
- A regression test verifies two independent UUIDs, both directions of state changes
  without waiting for movement, and refusal when there is no acting player.
- Native AST apply and the full optimized game build passed on the same engine/mixin pins.
- Real isolated integrated server: ordinary click does not hide a frame; server sneak
  plus right-click sets Invisible/Fixed to 1; repeating restores both to 0.
- The control tool's `key` command bypasses physical key remapping. Therefore this
  automated game check does NOT claim a physical Right Shift/rebound-key test.
  Different physical bindings are handled upstream by the client's sneak input; the
  mod consumes only the server flag. A simultaneous two-client/dedicated run remains
  untested. No dedicated-server or controller hardware test is claimed.
