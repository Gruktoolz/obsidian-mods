use super::{Ingredient, ItemStack, StationRecipes, Stonecutting};
use std::sync::OnceLock;

pub fn recipes() -> &'static StationRecipes {
    static RECIPES: OnceLock<StationRecipes> = OnceLock::new();
    RECIPES.get_or_init(|| {
        let mut recipes = StationRecipes::default();
        let mut cut = |from: &str, into: &str, count| {
            recipes.stonecutting.push(Stonecutting {
                id: format!("strangely_charming:table_saw/{into}_from_{from}"),
                input: Ingredient::of(vec![format!("minecraft:{from}")]),
                result: ItemStack::new(&format!("minecraft:{into}"), count),
            });
        };
        for wood in [
            "acacia", "bamboo", "birch", "cherry", "crimson", "dark_oak", "jungle", "mangrove",
            "oak", "pale_oak", "poplar", "spruce", "warped",
        ] {
            let planks = format!("{wood}_planks");
            for (shape, count) in [
                ("button", 1),
                ("door", 1),
                ("fence", 1),
                ("fence_gate", 1),
                ("sign", 1),
                ("slab", 2),
                ("stairs", 1),
                ("trapdoor", 1),
            ] {
                cut(&planks, &format!("{wood}_{shape}"), count);
            }
            cut(&planks, "ladder", 4);
            if wood == "bamboo" {
                cut(&planks, "bamboo_raft", 1);
                cut("bamboo_block", "stripped_bamboo_block", 1);
                for block in ["bamboo_block", "stripped_bamboo_block"] {
                    cut(block, &planks, 2);
                }
                continue;
            }
            let (log, bark) = if matches!(wood, "crimson" | "warped") {
                ("stem", "hyphae")
            } else {
                cut(&planks, &format!("{wood}_boat"), 1);
                ("log", "wood")
            };
            let log = format!("{wood}_{log}");
            let bark = format!("{wood}_{bark}");
            cut(&log, &bark, 1);
            for block in [log, bark] {
                let stripped = format!("stripped_{block}");
                cut(&block, &stripped, 1);
                cut(&block, &planks, 4);
                cut(&stripped, &planks, 4);
            }
        }
        recipes.stonecutting.sort_by(|a, b| a.id.cmp(&b.id));
        recipes
    })
}
