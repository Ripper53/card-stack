use card_game::{
    cards::CardID,
    identifications::{CastTo, PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{StateFilter, StateFilterInputConversion},
    zones::Zone,
};

use crate::{
    Game,
    cards::{
        CardKind,
        monster::{MonsterCard, MonsterCardType},
    },
    filters::{CardIn, FilterInput, Free, In, MonsterSlot, OfType},
    identifications::{SlotDoesNotExistError, ValidSlotID},
    zones::{GetZone, SlotID, hand::HandZone},
};

pub struct With<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone, F>
    StateFilter<State, FilterInput<(ValidPlayerID<F>, SlotID)>>
    for With<(Free<MonsterSlot>, In<Z>)>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidSlotID<In<Z>>)>;
    type Error = SlotDoesNotExistError;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, slot_id)): FilterInput<(ValidPlayerID<F>, SlotID)>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_id)
            .map(|valid_slot_id| FilterInput((valid_player_id, valid_slot_id)))
    }
}

pub struct EqualOrLowerThan<T>(std::marker::PhantomData<T>);
pub struct Level<const LEVEL: usize>;

impl<State: GetState<Game>, F, const LEVEL: usize>
    StateFilter<
        State,
        FilterInput<(
            ValidPlayerID<F>,
            ValidCardID<(CardIn<HandZone>, OfType<MonsterCard>)>,
        )>,
    > for With<EqualOrLowerThan<Level<LEVEL>>>
{
    type ValidOutput = FilterInput<(
        ValidPlayerID<F>,
        ValidCardID<(CardIn<HandZone>, OfType<MonsterCard>, Self)>,
    )>;
    type Error = MonsterLevelIsNotEqualOrLessThanError;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, valid_card_id)): FilterInput<(
            ValidPlayerID<F>,
            ValidCardID<(CardIn<HandZone>, OfType<MonsterCard>)>,
        )>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        let card = state
            .state()
            .zone_manager()
            .valid_zone(&valid_player_id)
            .hand_zone
            .valid_card(&valid_card_id.cast_ref());
        let CardKind::Monster(MonsterCardType::Monster(monster)) = card.kind() else {
            unreachable!();
        };
        if monster.level() > crate::cards::monster::Level::new(LEVEL) {
            Err(MonsterLevelIsNotEqualOrLessThanError {
                card_id: valid_card_id.id(),
                level: LEVEL,
            })
        } else {
            Ok(FilterInput((
                valid_player_id,
                valid_card_id.unchecked_replace_filter(),
            )))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("monster {card_id} level is not equal to or less than {level}")]
pub struct MonsterLevelIsNotEqualOrLessThanError {
    card_id: CardID,
    level: usize,
}

/*impl<State: GetState<Game>> StateFilter<State, (SlotID,)> for With<MonsterSlot> {
    type ValidOutput = ValidSlotID<Free<MonsterSlot>>;
    fn filter(state: &State, valid_player_id: (SlotID,)) -> Option<Self::ValidOutput> {
        //ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
        //.map(|valid_slot_id| (valid_player_id, c, valid_slot_id))
        None
    }
}*/

/*impl StateFilterInput<SlotID> for (ValidPlayerID<()>, SlotID) {
    type Remainder = ValidPlayerID<()>;
    fn new(input: ValidPlayerID<()>, remainder: Self::Remainder) -> Self {
        todo!()
    }
    fn split_take(self) -> (SlotID, Self::Remainder) {
        todo!();
    }
}*/
