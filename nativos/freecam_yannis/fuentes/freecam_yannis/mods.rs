fn freecam_mods(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::freecam_yannis as m;

    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("FreeCam {}", m::VERSION),
        description: "Free camera and F6 follow view by Yannis_".into(),
        keys: vec![
            [
                "free".into(),
                "Free camera / tripod modifier".into(),
                "key.keyboard.f4".into(),
            ],
            [
                "follow".into(),
                "Follow camera".into(),
                "key.keyboard.f6".into(),
            ],
        ],
    });
}
