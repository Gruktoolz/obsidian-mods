fn hook(mut rows: Vec<Row>, ui: &OptionsUi) -> Vec<Row> {
    if ui.page() == Page::Controls {
        rows.insert(
            1.min(rows.len()),
            Row::Buttons(
                button(
                    0.,
                    0.,
                    150.,
                    Text::literal("Mod key bindings…"),
                    Act::Push(Page::Mods),
                ),
                None,
            ),
        );
    }
    rows
}
