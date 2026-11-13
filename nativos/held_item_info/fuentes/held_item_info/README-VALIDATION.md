# Validation — 2026-11-02

Tested Horizon 3db53df3bde39da07aabddec9a5fa850204fd32a,
Horizon Mixins 07dab17387480ac03829584d9d1ee9912c9bb515, Linux x86-64, Vulkan.

- Native AST apply passed together with the other AltCraft native prototypes.
- The full optimized game and horizon-ctl compiled successfully.
- An actual integrated server was launched with a fresh temporary game home,
  launcher home and XDG folders. No real player worlds were used.
- A real held diamond sword with Sharpness III rendered the extra line containing
  1561 / 1561 durability and Sharpness 3; the screenshot was visually inspected.
- The vanilla item name remained on its own row.
- Long strings are clamped by the same font-width API as centering and truncated on Unicode
  character boundaries. No separate exhaustive localization/layout matrix has been run.
