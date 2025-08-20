use card_game::{
    SuperCommand,
    commands::{Command, CommandManager},
};

#[test]
pub fn super_command() {
    struct A;
    impl Command for A {
        type Data = ();
        type InState = ();
        type OutState = ();
        fn new(data: Self::Data) -> Self {
            A
        }
        fn execute(&mut self, state: Self::InState) -> Self::OutState {
            ()
        }
        fn undo(self, state: Self::OutState) -> Self::InState {
            todo!()
        }
    }
    struct B;
    impl Command for B {
        type Data = ();
        type InState = ();
        type OutState = ();
        fn new(data: Self::Data) -> Self {
            B
        }
        fn execute(&mut self, state: Self::InState) -> Self::OutState {
            todo!()
        }
        fn undo(self, state: Self::OutState) -> Self::InState {
            todo!()
        }
    }
    #[derive(SuperCommand)]
    enum SuperCommand {
        A(A),
        B(B),
    }
    let mut history: CommandManager<SuperCommand> = CommandManager::new();
    history.execute::<A>((), ());
    assert_eq!(1, history.command_history().len());
}
