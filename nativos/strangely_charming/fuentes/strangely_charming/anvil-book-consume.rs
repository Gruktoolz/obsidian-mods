fn hook(menu: &mut Menu) {
    if menu.anvil.repair_count == 0
        && !menu.anvil.only_renaming
        && menu.containers[MAIN][1]
            .as_ref()
            .is_some_and(|s| s.count > 1 && s.item.as_ref() == "minecraft:enchanted_book")
    {
        // Use the vanilla material-consumption path to preserve unused books.
        menu.anvil.repair_count = 1;
    }
}
