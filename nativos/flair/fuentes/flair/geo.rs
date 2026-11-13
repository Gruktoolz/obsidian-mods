use super::umbx_flair_model::{Animation, Bone, Channel, Cube, Track, Uv};
use serde_json::Value;

pub const FEET: f32 = 24.0;
const FACES: [&str; 6] = ["up", "down", "east", "north", "west", "south"];

fn number(v: &Value, default: f32) -> f32 {
    v.as_f64().map_or(default, |n| n as f32)
}

fn vec3(v: &Value, default: f32) -> [f32; 3] {
    match v {
        Value::Array(a) => std::array::from_fn(|i| a.get(i).map_or(default, |n| number(n, default))),
        Value::Number(_) => [number(v, default); 3],
        _ => [default; 3],
    }
}

pub fn geometry(text: &str) -> Result<(Vec<Bone>, [f32; 2]), String> {
    let json: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let geo = json["minecraft:geometry"].get(0).ok_or("no minecraft:geometry entry")?;
    let texture = [number(&geo["description"]["texture_width"], 64.0), number(&geo["description"]["texture_height"], 64.0)];
    let list = geo["bones"].as_array().ok_or("no bones")?;
    let mut flat: Vec<(Option<String>, Bone)> = Vec::new();
    for b in list {
        let name = b["name"].as_str().ok_or("a bone has no name")?.to_owned();
        let [px, py, pz] = vec3(&b["pivot"], 0.0);
        let mirror = b["mirror"].as_bool().unwrap_or(false);
        let mut bone = Bone { name: name.clone(), pivot: [px, FEET - py, pz], rotation: vec3(&b["rotation"], 0.0).map(f32::to_radians), ..Bone::default() };
        for (i, c) in b["cubes"].as_array().into_iter().flatten().enumerate() {
            let cube = parse_cube(c, mirror)?;
            if c.get("rotation").is_some() {
                let [cx, cy, cz] = vec3(&c["pivot"], 0.0);
                bone.children.push(Bone {
                    name: format!("{name}#{i}"),
                    pivot: [cx, FEET - cy, cz],
                    rotation: vec3(&c["rotation"], 0.0).map(f32::to_radians),
                    cubes: vec![cube],
                    children: Vec::new(),
                });
            } else {
                bone.cubes.push(cube);
            }
        }
        flat.push((b["parent"].as_str().map(str::to_owned), bone));
    }
    Ok((tree(&mut flat, None), texture))
}

fn parse_cube(c: &Value, mirror: bool) -> Result<Cube, String> {
    let [ox, oy, oz] = vec3(&c["origin"], 0.0);
    let size = vec3(&c["size"], 0.0);
    let uv = match &c["uv"] {
        Value::Object(faces) => Uv::Faces(FACES.map(|name| match faces.get(name) {
            Some(face) => {
                let [u, v, _] = vec3(&face["uv"], 0.0);
                let [w, h, _] = vec3(&face["uv_size"], 0.0);
                [u, v, u + w, v + h]
            }
            None => [f32::NAN; 4],
        })),
        other => {
            let [u, v, _] = vec3(other, 0.0);
            Uv::Box([u, v])
        }
    };
    Ok(Cube {
        from: [ox, FEET - oy - size[1], oz],
        size,
        uv,
        inflate: number(&c["inflate"], 0.0),
        mirror: c["mirror"].as_bool().unwrap_or(mirror),
        layer: 0,
    })
}

fn tree(flat: &mut Vec<(Option<String>, Bone)>, parent: Option<&str>) -> Vec<Bone> {
    let mut mine = Vec::new();
    let mut i = 0;
    while i < flat.len() {
        if flat[i].0.as_deref() == parent {
            mine.push(flat.remove(i).1);
        } else {
            i += 1;
        }
    }
    for bone in &mut mine {
        let name = bone.name.clone();
        bone.children.extend(tree(flat, Some(&name)));
    }
    mine
}

pub fn animation(text: &str) -> Result<Animation, String> {
    let json: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let first = json["animations"].as_object().and_then(|a| a.values().next()).ok_or("no animations")?;
    let mut length = number(&first["animation_length"], 0.0);
    let mut tracks = Vec::new();
    for (bone, channels) in first["bones"].as_object().into_iter().flatten() {
        let rotation = channel(&channels["rotation"], 0.0, |[x, y, z]| [x.to_radians(), y.to_radians(), z.to_radians()]);
        let position = channel(&channels["position"], 0.0, |[x, y, z]| [x, -y, z]);
        let scale = channel(&channels["scale"], 1.0, |v| v);
        for c in [&rotation, &position, &scale] {
            length = length.max(c.keys.last().map_or(0.0, |k| k.0));
        }
        tracks.push((bone.clone(), Track { rotation, position, scale }));
    }
    Ok(Animation { length, tracks })
}

fn channel(v: &Value, default: f32, convert: impl Fn([f32; 3]) -> [f32; 3]) -> Channel {
    let keys = match v {
        Value::Object(keys) => {
            let mut keys: Vec<(f32, [f32; 3])> = keys
                .iter()
                .filter_map(|(time, value)| {
                    let value = if value.is_object() { value.get("post").or_else(|| value.get("pre")).unwrap_or(&Value::Null) } else { value };
                    Some((time.parse().ok()?, convert(vec3(value, default))))
                })
                .collect();
            keys.sort_by(|a, b| a.0.total_cmp(&b.0));
            keys
        }
        Value::Null => Vec::new(),
        constant => vec![(0.0, convert(vec3(constant, default)))],
    };
    Channel { keys }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::umbx_flair_model::{cube, faces};

    const GEO: &str = r#"{"format_version":"1.12.0","minecraft:geometry":[{"description":{"identifier":"geometry.test","texture_width":64,"texture_height":64},
        "bones":[{"name":"head","pivot":[0,24,0],"cubes":[{"origin":[-4,24,-4],"size":[8,8,8],"uv":[0,0]}]},
                 {"name":"feather","parent":"head","pivot":[0,32,0],"rotation":[0,0,30],"cubes":[{"origin":[-1,32,0],"size":[2,4,0],"uv":{"north":{"uv":[0,16],"uv_size":[2,4]}}}]}]}]}"#;

    #[test]
    fn bedrock_bones_become_head_space_bones() {
        let (bones, texture) = geometry(GEO).unwrap();
        assert_eq!(texture, [64.0, 64.0]);
        assert_eq!(bones.len(), 1);
        let head = &bones[0];
        assert_eq!(head.pivot, [0.0, 0.0, 0.0]);
        assert_eq!(head.cubes[0], cube([-4.0, -8.0, -4.0], [8.0; 3], [0.0, 0.0]));
        let feather = &head.children[0];
        assert_eq!(feather.pivot, [0.0, -8.0, 0.0]);
        assert!((feather.rotation[2] - 30f32.to_radians()).abs() < 1e-6);
        assert_eq!(faces(&feather.cubes[0], feather.pivot, texture).len(), 1);
    }

    #[test]
    fn animations_read_keyframes_and_constants() {
        let text = r#"{"animations":{"animation.test.idle":{"loop":true,"bones":{
            "wing":{"rotation":{"0.0":[0,0,0],"1.0":{"post":[0,90,0]}},"position":[0,2,0]}}}}}"#;
        let animation = animation(text).unwrap();
        assert_eq!(animation.length, 1.0);
        let motion = animation.motion("wing", 0.5);
        assert!((motion.rotation[1] - 45f32.to_radians()).abs() < 1e-5);
        assert_eq!(motion.offset, [0.0, -2.0, 0.0]);
        assert!(geometry("{}").is_err());
    }
}
