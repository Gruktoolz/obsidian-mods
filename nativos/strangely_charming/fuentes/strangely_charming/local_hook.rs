fn hook(
    downloader: &mut crate::skins::Downloader,
    kind: crate::skins::Kind,
    url: &str,
) -> core::ops::ControlFlow<Download> {
    if kind == crate::skins::Kind::Skin {
        if let Some(hash) = crate::skins::texture_hash(url) {
            let location = crate::skins::skin_location(&hash);
            if let Some(pixels) = crate::strangely_charming_skin::local(&location) {
                if !downloader.images.contains_key(&location) {
                    downloader.queue(location, pixels);
                }
                return core::ops::ControlFlow::Break(crate::skins::Download::Ready);
            }
        }
    }
    core::ops::ControlFlow::Continue(())
}
