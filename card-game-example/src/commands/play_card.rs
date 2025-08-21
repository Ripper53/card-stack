use card_game::{
    commands::Command,
    zones::{ArrayZone, FiniteZone, ZoneCardID},
};

use crate::{steps::MainStep, zones::hand::HandZone};

pub struct PlayCardCommand<'a>(ZoneCardID<'a, HandZone>);

impl<'a> Command for PlayCardCommand<'a> {
    type Data = ZoneCardID<'a, HandZone>;
    type InState = MainStep;
    type OutState = MainStep;
    fn new(card_id: Self::Data) -> Self {
        PlayCardCommand(card_id)
    }
    fn execute(&mut self, mut state: Self::InState) -> Self::OutState {
        /*let zones = state.game.active_player_zones_mut();
        let card = zones.hand_zone.remove_card(self.0);*/
        /*zones.monster_zone.slot_a()
        self.0
            .remove(|| state.game.zone_manager.get_zone(player_id))*/
        //todo!()
        state
    }
    fn undo(self, state: Self::OutState) -> Self::InState {
        todo!()
    }
}
