#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Uv {
    Box([f32; 2]),
    Faces([[f32; 4]; 6]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    pub from: [f32; 3],
    pub size: [f32; 3],
    pub uv: Uv,
    pub inflate: f32,
    pub mirror: bool,
    pub layer: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bone {
    pub name: String,
    pub pivot: [f32; 3],
    pub rotation: [f32; 3],
    pub cubes: Vec<Cube>,
    pub children: Vec<Bone>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub normal: [f32; 3],
    pub vertices: [[f32; 5]; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct Baked {
    pub name: String,
    pub pose: [f32; 9],
    pub faces: Vec<Face>,
    pub children: Vec<Baked>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Motion {
    pub offset: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Motion {
    pub const REST: Motion = Motion { offset: [0.0; 3], rotation: [0.0; 3], scale: [1.0; 3] };

    pub fn apply(&self, pose: [f32; 9]) -> [f32; 9] {
        let mut out = pose;
        for i in 0..3 {
            out[i] += self.offset[i];
            out[i + 3] += self.rotation[i];
            out[i + 6] *= self.scale[i];
        }
        out
    }
}

pub fn cube(from: [f32; 3], size: [f32; 3], uv: [f32; 2]) -> Cube {
    Cube { from, size, uv: Uv::Box(uv), inflate: 0.0, mirror: false, layer: 0 }
}

pub fn bone(name: &str, pivot: [f32; 3], cubes: Vec<Cube>) -> Bone {
    Bone { name: name.into(), pivot, cubes, ..Bone::default() }
}

impl Cube {
    pub fn inflated(self, inflate: f32) -> Self {
        Cube { inflate, ..self }
    }

    pub fn faced(from: [f32; 3], size: [f32; 3], rects: [[f32; 4]; 6], layer: usize) -> Self {
        Cube { from, size, uv: Uv::Faces(rects), inflate: 0.0, mirror: false, layer }
    }

    fn rects(&self) -> [[f32; 4]; 6] {
        match self.uv {
            Uv::Faces(rects) => rects,
            Uv::Box([u, v]) => {
                let [w, h, d] = self.size;
                [
                    [u + d, v, u + d + w, v + d],
                    [u + d + w, v, u + d + 2.0 * w, v + d],
                    [u, v + d, u + d, v + d + h],
                    [u + d, v + d, u + d + w, v + d + h],
                    [u + d + w, v + d, u + 2.0 * d + w, v + d + h],
                    [u + 2.0 * d + w, v + d, u + 2.0 * d + 2.0 * w, v + d + h],
                ]
            }
        }
    }
}

impl Bone {
    pub fn turned(self, rotation: [f32; 3]) -> Self {
        Bone { rotation, ..self }
    }

    pub fn with(self, children: Vec<Bone>) -> Self {
        Bone { children, ..self }
    }

    pub fn shifted(&self, by: [f32; 3]) -> Bone {
        let add = |p: [f32; 3]| std::array::from_fn(|i| p[i] + by[i]);
        Bone {
            name: self.name.clone(),
            pivot: add(self.pivot),
            rotation: self.rotation,
            cubes: self.cubes.iter().map(|c| Cube { from: add(c.from), ..c.clone() }).collect(),
            children: self.children.iter().map(|c| c.shifted(by)).collect(),
        }
    }

    pub fn mirrored(&self, name: &str) -> Bone {
        let [x, y, z] = self.pivot;
        Bone {
            name: name.into(),
            pivot: [-x, y, z],
            rotation: [self.rotation[0], -self.rotation[1], -self.rotation[2]],
            cubes: self.cubes.iter().map(|c| Cube { from: [-c.from[0] - c.size[0], c.from[1], c.from[2]], mirror: !c.mirror, ..c.clone() }).collect(),
            children: self.children.iter().map(|c| c.mirrored(&format!("{name}.{}", c.name))).collect(),
        }
    }
}

pub fn faces(cube: &Cube, pivot: [f32; 3], texture: [f32; 2]) -> Vec<Face> {
    let g = cube.inflate;
    let lo: [f32; 3] = std::array::from_fn(|i| cube.from[i] - g - pivot[i]);
    let hi: [f32; 3] = std::array::from_fn(|i| cube.from[i] + cube.size[i] + g - pivot[i]);
    let (x0, x1) = if cube.mirror { (hi[0], lo[0]) } else { (lo[0], hi[0]) };
    let ([_, y0, z0], [_, y1, z1]) = (lo, hi);
    let corners: [([f32; 3], [[f32; 3]; 4]); 6] = [
        ([0.0, -1.0, 0.0], [[x1, y0, z1], [x0, y0, z1], [x0, y0, z0], [x1, y0, z0]]),
        ([0.0, 1.0, 0.0], [[x1, y1, z0], [x0, y1, z0], [x0, y1, z1], [x1, y1, z1]]),
        ([-1.0, 0.0, 0.0], [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]]),
        ([0.0, 0.0, -1.0], [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]]),
        ([1.0, 0.0, 0.0], [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]]),
        ([0.0, 0.0, 1.0], [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]]),
    ];
    corners
        .into_iter()
        .zip(cube.rects())
        .filter(|(_, rect)| !rect[0].is_nan())
        .map(|((mut normal, points), [u0, v0, u1, v1])| {
            let (top, bottom) = if normal[1] > 0.0 { (v1, v0) } else { (v0, v1) };
            let uv = [[u1, top], [u0, top], [u0, bottom], [u1, bottom]];
            let mut vertices: [[f32; 5]; 4] = std::array::from_fn(|i| {
                let [x, y, z] = points[i];
                [x, y, z, uv[i][0] / texture[0], uv[i][1] / texture[1]]
            });
            if cube.mirror {
                normal[0] = -normal[0];
                vertices.reverse();
            }
            Face { normal, vertices }
        })
        .collect()
}

pub const NONE: [f32; 4] = [f32::NAN; 4];

pub fn only(face: usize, rect: [f32; 4]) -> [[f32; 4]; 6] {
    std::array::from_fn(|i| if i == face { rect } else { NONE })
}

pub fn layers(bones: &[Bone]) -> usize {
    let mut most = 0;
    let mut stack: Vec<&Bone> = bones.iter().collect();
    while let Some(bone) = stack.pop() {
        stack.extend(&bone.children);
        most = bone.cubes.iter().map(|c| c.layer + 1).fold(most, usize::max);
    }
    most
}

pub fn bake(bone: &Bone, parent: [f32; 3], texture: [f32; 2], layer: usize) -> Baked {
    let p = bone.pivot;
    let r = bone.rotation;
    Baked {
        name: bone.name.clone(),
        pose: [p[0] - parent[0], p[1] - parent[1], p[2] - parent[2], r[0], r[1], r[2], 1.0, 1.0, 1.0],
        faces: bone.cubes.iter().filter(|c| c.layer == layer).flat_map(|c| faces(c, p, texture)).collect(),
        children: bone.children.iter().map(|c| bake(c, p, texture, layer)).collect(),
    }
}

pub fn bounds(bones: &[Bone]) -> Option<([f32; 3], [f32; 3])> {
    let mut out: Option<([f32; 3], [f32; 3])> = None;
    let mut stack: Vec<&Bone> = bones.iter().collect();
    while let Some(bone) = stack.pop() {
        stack.extend(&bone.children);
        for c in &bone.cubes {
            let lo: [f32; 3] = std::array::from_fn(|i| c.from[i] - c.inflate);
            let hi: [f32; 3] = std::array::from_fn(|i| c.from[i] + c.size[i] + c.inflate);
            out = Some(match out {
                None => (lo, hi),
                Some((a, b)) => (std::array::from_fn(|i| a[i].min(lo[i])), std::array::from_fn(|i| b[i].max(hi[i]))),
            });
        }
    }
    out
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Channel {
    pub keys: Vec<(f32, [f32; 3])>,
}

impl Channel {
    pub fn at(&self, t: f32) -> Option<[f32; 3]> {
        let next = self.keys.iter().position(|(time, _)| *time > t);
        match next {
            None => self.keys.last().map(|k| k.1),
            Some(0) => Some(self.keys[0].1),
            Some(i) => {
                let ((t0, a), (t1, b)) = (self.keys[i - 1], self.keys[i]);
                let f = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                Some(std::array::from_fn(|k| a[k] + (b[k] - a[k]) * f))
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Track {
    pub rotation: Channel,
    pub position: Channel,
    pub scale: Channel,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Animation {
    pub length: f32,
    pub tracks: Vec<(String, Track)>,
}

impl Animation {
    pub fn motion(&self, bone: &str, seconds: f32) -> Motion {
        let Some((_, track)) = self.tracks.iter().find(|(name, _)| name == bone) else { return Motion::REST };
        let t = if self.length > 0.0 { seconds.rem_euclid(self.length) } else { 0.0 };
        Motion {
            offset: track.position.at(t).unwrap_or([0.0; 3]),
            rotation: track.rotation.at(t).unwrap_or([0.0; 3]),
            scale: track.scale.at(t).unwrap_or([1.0; 3]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: [[[f32; 5]; 4]; 6] = [
        [[4.0, -8.0, 4.0, 0.25, 0.0], [-4.0, -8.0, 4.0, 0.125, 0.0], [-4.0, -8.0, -4.0, 0.125, 0.125], [4.0, -8.0, -4.0, 0.25, 0.125]],
        [[4.0, 0.0, -4.0, 0.375, 0.125], [-4.0, 0.0, -4.0, 0.25, 0.125], [-4.0, 0.0, 4.0, 0.25, 0.0], [4.0, 0.0, 4.0, 0.375, 0.0]],
        [[-4.0, -8.0, -4.0, 0.125, 0.125], [-4.0, -8.0, 4.0, 0.0, 0.125], [-4.0, 0.0, 4.0, 0.0, 0.25], [-4.0, 0.0, -4.0, 0.125, 0.25]],
        [[4.0, -8.0, -4.0, 0.25, 0.125], [-4.0, -8.0, -4.0, 0.125, 0.125], [-4.0, 0.0, -4.0, 0.125, 0.25], [4.0, 0.0, -4.0, 0.25, 0.25]],
        [[4.0, -8.0, 4.0, 0.375, 0.125], [4.0, -8.0, -4.0, 0.25, 0.125], [4.0, 0.0, -4.0, 0.25, 0.25], [4.0, 0.0, 4.0, 0.375, 0.25]],
        [[-4.0, -8.0, 4.0, 0.5, 0.125], [4.0, -8.0, 4.0, 0.375, 0.125], [4.0, 0.0, 4.0, 0.375, 0.25], [-4.0, 0.0, 4.0, 0.5, 0.25]],
    ];

    #[test]
    fn a_head_cube_matches_the_games_player_head() {
        let head = faces(&cube([-4.0, -8.0, -4.0], [8.0; 3], [0.0, 0.0]), [0.0; 3], [64.0, 64.0]);
        let found: Vec<_> = head.iter().map(|f| f.vertices).collect();
        assert_eq!(found, HEAD.to_vec());
        assert_eq!(head[3].normal, [0.0, 0.0, -1.0]);
    }

    #[test]
    fn mirrored_cubes_flip_x_and_winding() {
        let mut c = cube([0.0; 3], [2.0, 1.0, 1.0], [0.0, 0.0]);
        c.mirror = true;
        let f = faces(&c, [0.0; 3], [16.0, 16.0]);
        assert_eq!(f[2].normal, [1.0, 0.0, 0.0]);
        assert_eq!(f[2].vertices[3][0], 2.0);
        let m = bone("w", [1.0, 2.0, 3.0], vec![cube([1.0, 0.0, 0.0], [3.0, 1.0, 1.0], [0.0, 0.0])]).turned([0.1, 0.2, 0.3]).mirrored("v");
        assert_eq!((m.pivot, m.rotation), ([-1.0, 2.0, 3.0], [0.1, -0.2, -0.3]));
        assert_eq!(m.cubes[0].from[0], -4.0);
        assert!(m.cubes[0].mirror);
    }

    #[test]
    fn bake_makes_poses_relative_to_parents() {
        let tree = bone("a", [0.0, -4.0, 0.0], vec![cube([-1.0, -5.0, -1.0], [2.0; 3], [0.0, 0.0])]).with(vec![bone("b", [2.0, -6.0, 0.0], Vec::new())]);
        let baked = bake(&tree, [0.0; 3], [64.0, 64.0], 0);
        assert!(bake(&tree, [0.0; 3], [64.0, 64.0], 1).faces.is_empty());
        assert_eq!(layers(std::slice::from_ref(&tree)), 1);
        assert_eq!(&baked.pose[..3], &[0.0, -4.0, 0.0]);
        assert_eq!(&baked.children[0].pose[..3], &[2.0, -2.0, 0.0]);
        assert_eq!(baked.faces[0].vertices[0][1], -1.0);
        assert_eq!(bounds(std::slice::from_ref(&tree)), Some(([-1.0, -5.0, -1.0], [1.0, -3.0, 1.0])));
    }

    #[test]
    fn channels_interpolate_and_loop() {
        let channel = Channel { keys: vec![(0.0, [0.0; 3]), (1.0, [2.0, 4.0, 6.0])] };
        assert_eq!(channel.at(0.5), Some([1.0, 2.0, 3.0]));
        assert_eq!(channel.at(3.0), Some([2.0, 4.0, 6.0]));
        let animation = Animation { length: 1.0, tracks: vec![("a".into(), Track { rotation: channel, ..Track::default() })] };
        assert_eq!(animation.motion("a", 1.5).rotation, [1.0, 2.0, 3.0]);
        assert_eq!(animation.motion("b", 1.5), Motion::REST);
        assert_eq!(Motion { offset: [1.0, 0.0, 0.0], rotation: [0.0; 3], scale: [2.0; 3] }.apply([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]), [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0]);
    }
}
