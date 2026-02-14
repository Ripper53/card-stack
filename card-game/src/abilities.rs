/*use state_validation::ValidAction;

pub struct Passive {}

pub struct ReplacementEffects<State, Input> {
    required_effects: Vec<Box<dyn ValidAction<State, Input>>>,
    optional_effects: Vec<Box<dyn ValidAction<State, Input>>>,
}

impl<State, Input> ReplacementEffects<State, Input> {
    pub fn required_replacement(&mut self, replacement_action: impl ValidAction<State, Input>) {
        self.required_effects.push(Box::new(replacement_action))
    }
    pub fn optional_replacement(&mut self, replacement_action: impl ValidAction<State, Input>) {
        self.optional_effects.push(Box::new(replacement_action))
    }
}*/
