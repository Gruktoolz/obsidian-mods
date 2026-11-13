fn hook(width: u32, height: u32, rgba: &[u8], value: Option<Vec<u8>>) -> Option<Vec<u8>> {
    crate::strangely_charming_skin::process(width, height, rgba, value)
}
