fn set_crop_seed(block: &NewBlock, row: &mut J) {
    if block.decl.id == "inferium_lab:inferium_crop" {
        row.set("item", J::Str("inferium_lab:inferium_seeds".into()));
    }
}
