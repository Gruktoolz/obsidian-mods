// Options' first page: the "Chat Animation Settings..." button, in the free half of the grid's
// last row (beside Credits), else above or under the grid. Its press is the mouse hook's.
fn chat_animation_options_layout(
    ui: &mut OptionsUi,
    s: &Settings,
    w: f32,
    h: f32,
) -> ::core::ops::ControlFlow<(Vec<Widget>, f32, f32, f32)> {
    use crate::native_chat_animation as ca;
    if ui.page() != Page::Main || !ca::button_wanted() || !ca::enter(4) {
        return ::core::ops::ControlFlow::Continue(());
    }
    let (mut widgets, top, bottom, content) = ui.layout(s, w, h);
    ca::leave(4);
    let grid: Vec<[f32; 3]> = widgets
        .iter()
        .filter(|wd| wd.w == 150.0 && wd.y > 40.0 && wd.y < h - 30.0)
        .map(|wd| [wd.x, wd.y, wd.w])
        .collect();
    let others: Vec<[f32; 4]> = widgets.iter().map(|wd| [wd.x, wd.y, wd.w, wd.h]).collect();
    match ca::place(&grid, &others, w, h) {
        Some(r) => {
            widgets.push(button(r[0], r[1], r[2], Text::literal(ca::BUTTON_LABEL), Act::None));
            ca::set_button(r);
        }
        None => ca::set_button([-1000.0, -1000.0, 0.0, 0.0]),
    }
    ::core::ops::ControlFlow::Break((widgets, top, bottom, content))
}
