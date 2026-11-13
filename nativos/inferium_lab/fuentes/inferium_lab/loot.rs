fn add_harvest_table(files: &mut Vec<(String, Vec<u8>)>) {
    files.push((
        "loot_table/inferium_lab:blocks/inferium_crop".into(),
        crate::native_data!("inferium_lab", "inferium_crop.json").to_vec(),
    ));
}
