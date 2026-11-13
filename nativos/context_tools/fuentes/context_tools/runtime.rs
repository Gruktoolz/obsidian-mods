// Context Tool Swapping — port of the Fabric mod `context-tools` 1.3+26.1 (CC0-1.0, Trinketina).
//
// Vanilla pick block (`Game::play_mouse`, middle click) puts the aimed block's item in
// hand. This mod intercepts that: when the held item is a tool and the aimed block wants
// a different tool family, it selects the matching tool from the hotbar instead and
// cancels vanilla pick.
//
// The Java original injected at HEAD of `pickBlockOrEntity` and called `ci.cancel()` on
// success; here the injection is a cancellable HEAD on `play_mouse`, whose middle-click
// branch is Horizon's same code path, so `Break(Ok(()))` reproduces that cancel.
//
// The Java version scans all 36 main slots plus the off-hand and sends container/swap
// packets. With `full_inventory` on (the default) this mod does the same: a tool in the
// hotbar is selected directly, one in the main inventory is swapped into the held slot
// through Horizon's verified menu engine (`Input::Swap`, the same op Baritone's
// `swap_click` uses), and one in the off hand goes through the F-key swap op (button 40).
// With it off, only the hotbar is scanned, like toolswap.
use super::Game;

pub(super) struct State {
    enabled: bool,
    require_control: bool,
    require_crouching: bool,
    full_inventory: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled: true,
            require_control: false,
            require_crouching: false,
            full_inventory: true,
        }
    }
}

thread_local! {
    static STATES: std::cell::RefCell<std::collections::BTreeMap<std::path::PathBuf, State>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

fn with_state<T>(game: &mut Game<'_>, run: impl FnOnce(&mut Game<'_>, &mut State) -> T) -> T {
    struct Lease {
        key: std::path::PathBuf,
        state: State,
    }
    impl Drop for Lease {
        fn drop(&mut self) {
            STATES.with(|states| {
                states
                    .borrow_mut()
                    .insert(self.key.clone(), std::mem::take(&mut self.state));
            });
        }
    }
    let key = game.world_root.clone();
    let state = STATES.with(|states| states.borrow_mut().remove(&key).unwrap_or_default());
    let mut lease = Lease { key, state };
    run(game, &mut lease.state)
}

/// The four tool families the Java mod distinguishes (`ItemTags.PICKAXES`, `SHOVELS`,
/// `AXES`, `HOES`), in its own priority order.
const FAMILIES: [&str; 4] = ["pickaxe", "shovel", "axe", "hoe"];

/// A stack's tool family, from its item id (`minecraft:diamond_pickaxe` -> `pickaxe`).
/// Java asks the tag; Horizon's registry has no tag query reachable from here, and the
/// vanilla ids all end with the family name, which also covers most modded tools.
fn family(id: &str) -> Option<&'static str> {
    FAMILIES.iter().copied().find(|f| id.ends_with(f))
}

/// A tool is unusable when its damage reached its max (the same rule toolswap uses).
fn broken(items: &horizon_client::items::Items, stack: &horizon_client::items::ItemStack) -> bool {
    items
        .get(&stack.item)
        .is_some_and(|info| info.max_damage > 0 && stack.damage >= info.max_damage)
}

/// The game slot of the off hand (`game/shield.rs`'s `OFFHAND_SLOT`; it is not a menu
/// slot, so the vanilla F swap reads and writes `Game::offhand` directly).
const GAME_OFFHAND: usize = 40;

impl Game<'_> {
    /// Whether this mod handled the pick, so vanilla pick must not run.
    pub(super) fn context_tools_pick(&mut self, pressed: bool) -> Option<bool> {
        if !pressed {
            return None;
        }
        with_state(self, |game, state| {
            if !state.enabled {
                return None;
            }
            // Java gates 1 and 2 (`isCrouching`, `hasControlDown`), both off by default.
            if state.require_crouching && !game.has_shift_down() {
                return None;
            }
            if state.require_control && !game.has_control_down() {
                return None;
            }
            // Java gate 3: never interfere in creative, where pick clones blocks.
            if game.creative {
                return None;
            }
            let current = game.selected.min(8);
            let held = game.inventory.get(current).and_then(|slot| slot.as_ref())?;
            // Java gates 4 and 5: only when the held item is itself a tool.
            let held_family = family(&held.item)?;
            if broken(game.items(), held) {
                return None;
            }
            // Java gate 6: needs a block under the crosshair; entities keep vanilla.
            let hit = game.pick()?;
            let pos = hit.pos;
            if pos[1] < game.world.shape.min_y
                || pos[1] >= game.world.shape.max_y()
                || !game.world.loaded(pos[0], pos[2])
            {
                return None;
            }
            let facts = game.res.facts().get(game.world.block(pos[0], pos[1], pos[2]));
            // Unbreakable (-1) and instant-break (0) blocks need no tool.
            if facts.hardness <= 0.0 {
                return None;
            }
            // Java gate 7: the block must want a family other than the held one. Which
            // family is wanted is the game's own `Tool.isCorrectForDrops` rule, probed
            // with a representative tool per family rather than a hard-coded tag list.
            let mut wanted = Vec::new();
            for f in FAMILIES {
                let probe = format!("minecraft:netherite_{f}");
                let stack = horizon_client::items::ItemStack {
                    item: probe.into(),
                    count: 1,
                    damage: 0,
                    extra: None,
                };
                if game.items().correct_for_drops(Some(&stack), &facts.name) {
                    wanted.push(f);
                }
            }
            if wanted.is_empty() || wanted.iter().any(|w| *w == held_family) {
                return None;
            }
            let usable = |game: &Game<'_>, stack: &horizon_client::items::ItemStack| {
                !broken(game.items(), stack)
                    && family(&stack.item).is_some_and(|f| wanted.iter().any(|w| *w == f))
            };
            // Java step 8, hotbar first: select the first hotbar tool of a wanted family.
            for slot in 0..9 {
                if game
                    .inventory
                    .get(slot)
                    .and_then(|slot| slot.as_ref())
                    .is_some_and(|stack| usable(game, stack))
                {
                    game.selected = slot;
                    return Some(true);
                }
            }
            if !state.full_inventory {
                // Without the full scan this is where Java returns false and vanilla pick runs.
                return None;
            }
            // The off hand: Java's `SWAP_ITEM_WITH_OFFHAND` packet. `tweak_click` is the
            // one menu-click entry a mod can reach; on a server it sends the click
            // unpredicted, which is what vanilla F does for the swap packet anyway.
            if game.offhand.as_ref().is_some_and(|stack| usable(game, stack)) {
                let done = game.tweak_click(current, 40, horizon_client::menu::Input::Swap);
                if done {
                    game.hand_stack_replaced(current);
                    game.hand_stack_replaced(GAME_OFFHAND);
                }
                return Some(done);
            }
            // The main inventory: Java's `handleContainerInput(containerId, slot,
            // selectedSlot, SWAP, player)`, here the menu engine's SWAP op — the same one
            // Baritone's `swap_click` sends. A server that rejects it keeps its own state,
            // so the swap is only claimed when the click went through.
            for slot in 9..36 {
                if game
                    .inventory
                    .get(slot)
                    .and_then(|stack| stack.as_ref())
                    .is_some_and(|stack| usable(game, stack))
                {
                    let done =
                        game.tweak_click(slot, current as i32, horizon_client::menu::Input::Swap);
                    if done {
                        game.hand_stack_replaced(current);
                    }
                    return Some(done);
                }
            }
            // Nowhere a wanted tool: Java logs "could not find slot with …" and lets
            // vanilla pick run.
            None
        })
    }

    pub(super) fn context_tools_command(&mut self, command: &str) -> Option<bool> {
        with_state(self, |game, state| {
            let mut words = command.split_whitespace();
            if words.next()? != "contexttools" {
                return None;
            }
            let args: Vec<_> = words.collect();
            let message = match args.as_slice() {
                [] | ["status"] => format!(
                    "Context Tool Swapping: {} (control: {}, crouching: {}, inventory: {})",
                    if state.enabled { "on" } else { "off" },
                    state.require_control,
                    state.require_crouching,
                    state.full_inventory,
                ),
                ["toggle"] | ["on"] | ["off"] => {
                    state.enabled = match args[0] {
                        "on" => true,
                        "off" => false,
                        _ => !state.enabled,
                    };
                    format!(
                        "Context Tool Swapping: {}",
                        if state.enabled { "on" } else { "off" }
                    )
                }
                ["control"] | ["crouching"] | ["inventory"] => {
                    let flag = match args[0] {
                        "control" => &mut state.require_control,
                        "crouching" => &mut state.require_crouching,
                        _ => &mut state.full_inventory,
                    };
                    *flag = !*flag;
                    format!("{}: {}", args[0], flag)
                }
                _ => {
                    game.command_failure(horizon_client::text::Text::literal(
                        "Usage: /contexttools [status|toggle|on|off|control|crouching|inventory]",
                    ));
                    return Some(false);
                }
            };
            game.system_message(horizon_client::text::Text::literal(message));
            Some(true)
        })
    }
}
