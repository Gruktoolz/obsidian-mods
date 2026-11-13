fn hook(game: &crate::game::Game<'_>, files: &mut Vec<(String, Vec<u8>)>) {
    horizon_mods::strangely_charming_content::merchant_files(files);
    files.extend(
        horizon_client::recipe_book::evanders_recipe_files::read(
            &game.options.resources.join("data"),
        )
        .into_iter()
        .map(|(id, bytes)| (format!("recipe/{id}"), bytes)),
    );
}
