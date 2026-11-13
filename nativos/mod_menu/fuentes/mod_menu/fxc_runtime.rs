// DirectX 12 compiles shaders with FXC, which refuses Horizon's terrain shader (a corner written
// one component at a time: "array reference cannot be used as an l-value"), so the game stopped at
// its start there. The same position, put together whole.

/// The terrain shader's text with that corner made ready for FXC. None when `text` has no such
/// lines (another shader, or a Horizon that changed them).
pub fn fxc_terrain(text: &str) -> Option<String> {
    const OLD: &str = "    var pos = block;
    pos[axis] = block[axis] + side;
    pos[(axis + 1u) % 3u] = block[(axis + 1u) % 3u] + du;
    pos[(axis + 2u) % 3u] = block[(axis + 2u) % 3u] + dw;
";
    const NEW: &str = "    // Mod Menu: the corner whole, not one component at a time (FXC).
    let pos = block + select(select(vec3(side, du, dw), vec3(dw, side, du), axis == 1u),
        vec3(du, dw, side), axis == 2u);
";
    if !text.contains(OLD) {
        return None;
    }
    Some(text.replace(OLD, NEW))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_corner_is_written_whole() {
        let text = "fn f() {\n    var pos = block;\n    pos[axis] = block[axis] + side;\n    pos[(axis + 1u) % 3u] = block[(axis + 1u) % 3u] + du;\n    pos[(axis + 2u) % 3u] = block[(axis + 2u) % 3u] + dw;\n}\n";
        let fixed = fxc_terrain(text).unwrap();
        assert!(!fixed.contains("pos[axis]"));
        assert!(fixed.contains("let pos = block + select("));
        assert!(fxc_terrain(&fixed).is_none());
        assert!(fxc_terrain("fn other() {}").is_none());
    }
}
