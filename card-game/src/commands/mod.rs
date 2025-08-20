pub trait Command {
    type Data;
    type InState;
    type OutState;
    /// Use [`CommandManager::execute`](CommandManager::execute).
    fn new(data: Self::Data) -> Self;
    /// Use [`CommandManager`](CommandManager) to execute.
    fn execute(&mut self, state: Self::InState) -> Self::OutState;
    /// Use [`CommandManager`](CommandManager) to undo.
    fn undo(self, state: Self::OutState) -> Self::InState;
}

pub struct CommandManager<C: Command> {
    history: Vec<C>,
}

impl<C: Command> CommandManager<C> {
    pub fn new() -> Self {
        CommandManager {
            history: Vec::new(),
        }
    }
    pub fn execute<ExecuteC: Command + Into<C>>(
        &mut self,
        data: <ExecuteC as Command>::Data,
        state: <ExecuteC as Command>::InState,
    ) -> <ExecuteC as Command>::OutState {
        let mut command = ExecuteC::new(data);
        let out_state = command.execute(state);
        self.history.push(command.into());
        out_state
    }
    pub fn undo(&mut self, state: impl Into<C::OutState>) -> Result<C::InState, C::OutState> {
        if let Some(command) = self.history.pop() {
            Ok(command.undo(state.into()))
        } else {
            Err(state.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use card_game_derive::SuperCommand;

    use super::*;

    struct CommandA;
    impl Command for CommandA {
        type Data = ();
        type InState = ();
        type OutState = usize;
        fn new((): ()) -> Self {
            CommandA
        }
        fn execute(&mut self, _state: Self::InState) -> Self::OutState {
            0
        }
        fn undo(self, _state: Self::OutState) -> Self::InState {
            ()
        }
    }
    struct CommandB(usize);
    impl Command for CommandB {
        type Data = usize;
        type InState = usize;
        type OutState = String;
        fn new(value: usize) -> Self {
            CommandB(value)
        }
        fn execute(&mut self, state: Self::InState) -> Self::OutState {
            self.0 = state;
            format!("MEOW: {}", self.0)
        }
        fn undo(self, _state: Self::OutState) -> Self::InState {
            self.0
        }
    }
    enum Commands {
        A(CommandA),
        B(CommandB),
    }
    impl From<CommandA> for Commands {
        fn from(value: CommandA) -> Self {
            Commands::A(value)
        }
    }
    impl From<CommandB> for Commands {
        fn from(value: CommandB) -> Self {
            Commands::B(value)
        }
    }
    enum CommandInStates {
        A(<CommandA as Command>::InState),
        B(<CommandB as Command>::InState),
    }
    impl TryFrom<CommandInStates> for <CommandA as Command>::InState {
        type Error = ();
        fn try_from(value: CommandInStates) -> Result<Self, Self::Error> {
            if let CommandInStates::A(value) = value {
                Ok(value)
            } else {
                Err(())
            }
        }
    }
    impl TryFrom<CommandInStates> for <CommandB as Command>::InState {
        type Error = ();
        fn try_from(value: CommandInStates) -> Result<Self, Self::Error> {
            if let CommandInStates::B(value) = value {
                Ok(value)
            } else {
                Err(())
            }
        }
    }
    impl From<<CommandA as Command>::InState> for CommandInStates {
        fn from(value: <CommandA as Command>::InState) -> Self {
            CommandInStates::A(value)
        }
    }
    impl From<<CommandB as Command>::InState> for CommandInStates {
        fn from(value: <CommandB as Command>::InState) -> Self {
            CommandInStates::B(value)
        }
    }
    enum CommandOutStates {
        A(<CommandA as Command>::OutState),
        B(<CommandB as Command>::OutState),
    }
    impl TryFrom<CommandOutStates> for <CommandA as Command>::OutState {
        type Error = ();
        fn try_from(value: CommandOutStates) -> Result<Self, Self::Error> {
            if let CommandOutStates::A(value) = value {
                Ok(value)
            } else {
                Err(())
            }
        }
    }
    impl TryFrom<CommandOutStates> for <CommandB as Command>::OutState {
        type Error = ();
        fn try_from(value: CommandOutStates) -> Result<Self, Self::Error> {
            if let CommandOutStates::B(value) = value {
                Ok(value)
            } else {
                Err(())
            }
        }
    }
    impl From<<CommandA as Command>::OutState> for CommandOutStates {
        fn from(value: <CommandA as Command>::OutState) -> Self {
            CommandOutStates::A(value)
        }
    }
    impl From<<CommandB as Command>::OutState> for CommandOutStates {
        fn from(value: <CommandB as Command>::OutState) -> Self {
            CommandOutStates::B(value)
        }
    }
    impl Command for Commands {
        type Data = Self;
        type InState = CommandInStates;
        type OutState = CommandOutStates;
        fn new(commands: Self) -> Self {
            commands
        }
        fn execute(&mut self, state: Self::InState) -> Self::OutState {
            match self {
                Commands::A(command) => command.execute(state.try_into().unwrap()).into(),
                Commands::B(command) => command.execute(state.try_into().unwrap()).into(),
            }
        }
        fn undo(self, state: Self::OutState) -> Self::InState {
            match self {
                Commands::A(command) => command.undo(state.try_into().unwrap()).into(),
                Commands::B(command) => command.undo(state.try_into().unwrap()).into(),
            }
        }
    }
    #[test]
    fn command_history() {
        let mut command_history: CommandManager<Commands> = CommandManager::new();
        command_history.execute::<CommandA>((), ());
        let r = command_history.undo(0);
        assert!(r.is_ok());
    }
    #[test]
    #[should_panic]
    fn command_history_incorrect_undo_state() {
        let mut command_history: CommandManager<Commands> = CommandManager::new();
        command_history.execute::<CommandA>((), ());
        let _ = command_history.undo(format!("MEOW")); // Should panic!
    }
}
