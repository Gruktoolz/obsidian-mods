// The mod's row in Options > Mods.
fn chat_heads_rust_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::chat::native_chat_heads_rust as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
