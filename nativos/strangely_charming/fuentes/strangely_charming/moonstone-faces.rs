use super::*;
impl FaceVisibility {
    pub fn sc_merge_moonstones(&mut self, registry: &crate::registry::BlockRegistry) {
        let Some(common) = registry.default_state("strangely_charming:moonstone_white") else {
            return;
        };
        for (i, state) in Arc::make_mut(&mut self.states).iter_mut().enumerate() {
            if registry
                .state(BlockStateId(i as u32))
                .is_some_and(|s| s.name.starts_with("strangely_charming:moonstone_"))
            {
                state.block = common;
                state.definition.skip = SkipRendering::SameBlock;
            }
        }
    }
}
