fn hook(
    root: &Path,
    registry: &BlockRegistry,
) -> core::ops::ControlFlow<Result<horizon_world::block_render::FaceVisibility>> {
    if registry
        .default_state("strangely_charming:moonstone_white")
        .is_none()
    {
        return core::ops::ControlFlow::Continue(());
    }
    // Pinned Horizon resource loader (LICENSE.horizon).
    fn build(
        root: &Path,
        registry: &BlockRegistry,
    ) -> Result<horizon_world::block_render::FaceVisibility> {
        use horizon_world::{
            block_render::{FaceVisibility, RenderDefinition, SkipRendering},
            collision::CachedCollisionRegistry,
            registry::BlockTableFile,
        };
        use std::{collections::BTreeMap, fs};
        let file = BlockTableFile::open(&root.join("occlusion.nbt"), registry)?;
        let occlusion = CachedCollisionRegistry::from_table(file.table("occlusion")?)?;
        let raw: BTreeMap<String, Value> =
            serde_json::from_slice(&fs::read(root.join("render-definitions.json"))?)?;
        let definitions = raw
            .into_iter()
            .map(|(name, v)| {
                let skip = match v["rule"].as_str().ok_or("missing render rule")? {
                    "never" => SkipRendering::Never,
                    "same_block" => SkipRendering::SameBlock,
                    "bars" => SkipRendering::Bars,
                    "leaves" => SkipRendering::Leaves,
                    "liquid" => SkipRendering::Liquid,
                    "mangrove_roots" => SkipRendering::MangroveRoots,
                    _ => return Err("unrecognized render rule"),
                };
                Ok((
                    name,
                    RenderDefinition {
                        skip,
                        can_occlude: v["can_occlude"].as_bool().ok_or("missing occlusion flag")?,
                        bars_tag: v["bars_tag"].as_bool().ok_or("missing bars tag")?,
                    },
                ))
            })
            .collect::<std::result::Result<_, &str>>()?;
        let mut faces = FaceVisibility::new(&occlusion, &definitions)?;
        faces.sc_merge_moonstones(registry);
        Ok(faces)
    }

    core::ops::ControlFlow::Break(build(root, registry))
}
