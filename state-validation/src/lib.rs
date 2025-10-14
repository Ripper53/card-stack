//! ## State Validation
//! `state-validation` lets you validate an input for a given state.
//!
//! Ex. You want to remove an admin from `UserStorage`, given a `UserID`, you want to retrieve the `User` who maps onto the `UserID` and validate they are an existing user whose privilege level is admin.
//! The state is `UserStorage`, the input is a `UserID`, and the valid output is an `AdminUser`.
//! ```
//! # use std::collections::{HashSet, HashMap};
//! # use state_validation::{Validator, ValidAction, StateFilterInput, StateFilter, Condition};
//! // Input
//! // Mark each input with the `StateFilterInput` derive macro.
//! #[derive(StateFilterInput, Hash, PartialEq, Eq, Clone, Copy)]
//! struct UserID(usize);
//!
//! // State
//! #[derive(Default)]
//! struct UserStorage {
//!     maps: HashMap<UserID, User>,
//! }
//! #[derive(StateFilterInput, Clone)]
//! struct User {
//!     id: UserID,
//!     username: String,
//! }
//!
//! // Valid Output
//! // Mark with `StateFilterInput`, too.
//! #[derive(StateFilterInput)]
//! struct AdminUser(User);
//!
//! // FILTERS
//! struct UserExists;
//! # #[derive(Debug)]
//! # struct UserDoesNotExistError;
//! # impl std::error::Error for UserDoesNotExistError {}
//! # impl std::fmt::Display for UserDoesNotExistError {
//! #    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//! #        write!(f, "user does not exist")
//! #     }
//! # }
//! impl StateFilter<UserStorage, UserID> for UserExists {
//!     type ValidOutput = User;
//!     type Error = UserDoesNotExistError;
//!     fn filter(state: &UserStorage, user_id: UserID) -> Result<Self::ValidOutput, Self::Error> {
//!         if let Some(user) = state.maps.get(&user_id) {
//!             Ok(user.clone())
//!         } else {
//!             Err(UserDoesNotExistError)
//!         }
//!     }
//! }
//!
//! struct UserIsAdmin;
//! # #[derive(Debug)]
//! # struct UserIsNotAdminError;
//! # impl std::error::Error for UserIsNotAdminError {}
//! # impl std::fmt::Display for UserIsNotAdminError {
//! #    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//! #        write!(f, "user is not an admin")
//! #     }
//! # }
//! impl StateFilter<UserStorage, User> for UserIsAdmin {
//!     type ValidOutput = AdminUser;
//!     type Error = UserIsNotAdminError;
//!     fn filter(state: &UserStorage, user: User) -> Result<Self::ValidOutput, Self::Error> {
//!         if user.username == "ADMIN" {
//!             Ok(AdminUser(user))
//!         } else {
//!             Err(UserIsNotAdminError)
//!         }
//!     }
//! }
//!
//! // Action that removes the admin from user storage.
//! struct RemoveAdmin;
//! impl ValidAction<UserStorage, UserID> for RemoveAdmin {
//!     type Filter = (
//!         Condition<UserID, UserExists>,
//!         Condition<User, UserIsAdmin>,
//!     );
//!     type Output = UserStorage;
//!     fn with_valid_input(
//!         self,
//!         mut state: UserStorage,
//!         admin_user: <Self::Filter as StateFilter<UserStorage, UserID>>::ValidOutput,
//!     ) -> Self::Output {
//!         let _ = state.maps.remove(&admin_user.0.id).unwrap();
//!         state
//!     }
//! }
//!
//! // State setup
//! let mut user_storage = UserStorage::default();
//! user_storage.maps.insert(UserID(0), User {
//!     id: UserID(0),
//!     username: "ADMIN".to_string(),
//! });
//!
//! // Create validator which will validate the input.
//! // No error is returned if validation succeeds.
//! let validator = Validator::try_new(user_storage, UserID(0)).expect("admin user did not exist");
//!
//! // Execute an action which requires the state and input above.
//! let user_storage = validator.execute(RemoveAdmin);
//!
//! assert!(user_storage.maps.is_empty());
//! ```

mod actions;
mod condition;
#[cfg(feature = "input_collector")]
mod input_collector;
mod state_filter;
pub use actions::*;
pub use condition::*;
#[cfg(feature = "input_collector")]
pub use input_collector::*;
pub use state_filter::*;
#[cfg(feature = "derive")]
pub use state_validation_derive::*;

pub struct Validator<State, Input: StateFilterInput, Filter: StateFilter<State, Input>> {
    state: State,
    value: Filter::ValidOutput,
    _p: std::marker::PhantomData<(Input, Filter)>,
}

impl<State, Input: StateFilterInput, Filter: StateFilter<State, Input>>
    Validator<State, Input, Filter>
{
    pub fn try_new(state: State, input: Input) -> Result<Self, Filter::Error> {
        let value = Filter::filter(&state, input)?;
        Ok(Validator {
            state,
            value,
            _p: std::marker::PhantomData::default(),
        })
    }
    pub fn execute<Action: ValidAction<State, Input, Filter = Filter>>(
        self,
        valid_action: Action,
    ) -> Action::Output {
        valid_action.with_valid_input(self.state, self.value)
    }
}
