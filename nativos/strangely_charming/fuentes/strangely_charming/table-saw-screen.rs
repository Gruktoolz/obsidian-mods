use super::{Game, Kind};

impl Game<'_> {
    pub(in crate::game) fn evanders_table_saw(&self) -> bool {
        self.station.as_ref().is_some_and(|s| {
            s.kind == Kind::Stonecutter
                && if self.remote.is_some() {
                    self.remote_menu_title()
                        .is_some_and(|title| title.plain() == "Table Saw")
                } else {
                    self.block_name(s.pos).as_ref() == "strangely_charming:woodcutter"
                }
        })
    }
    pub(in crate::game) fn evanders_cut_recipes(
        &self,
    ) -> &horizon_client::stations::StationRecipes {
        if self.evanders_table_saw() {
            horizon_client::stations::evanders_saw::recipes()
        } else {
            &self.stations().recipes
        }
    }
}
