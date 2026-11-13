fn hook(value: [f32; 4], game: &crate::game::Game<'_>, camera: [f64; 3]) -> [f32; 4] {
    if !horizon_mods::evanders_config::enabled("moonstones") {
        return value;
    }
    let name = game.block_name(camera.map(|n| n.floor() as i32));
    let Some(color) = name.strip_prefix("strangely_charming:moonstone_") else {
        return value;
    };
    let rgb: u32 = match color {
        "white" => 0xf9fffe,
        "orange" => 0xf9801d,
        "magenta" => 0xc74ebd,
        "light_blue" => 0x3ab3da,
        "yellow" => 0xfed83d,
        "lime" => 0x80c71f,
        "pink" => 0xf38baa,
        "gray" => 0x474f52,
        "light_gray" => 0x9d9d97,
        "cyan" => 0x169c9c,
        "purple" => 0x8932b8,
        "blue" => 0x3c44aa,
        "brown" => 0x835432,
        "green" => 0x5e7c16,
        "red" => 0xb02e26,
        "black" => 0x1d1d21,
        _ => return value,
    };
    [
        ((rgb >> 16) & 255) as f32 / 255.,
        ((rgb >> 8) & 255) as f32 / 255.,
        (rgb & 255) as f32 / 255.,
        value[3],
    ]
}
