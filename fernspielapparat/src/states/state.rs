use crate::senses::Input;
pub use builder::StateBuilder;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default, Debug)]
pub struct State {
    /// Name of this state, not guaranteed to be unique.
    name: String,
    speech: String,
    sounds: Vec<usize>,
    /// Inputs against states to transition to
    input_transitions: HashMap<Input, usize>,
    /// If some, transitions to the state with the index
    /// after the specified duration has passed after the
    /// end of speech and all other actuators such as
    /// ringing.
    timeout_transition: Option<(Duration, usize)>,
    /// Transition to make after the speech has been
    /// spoken.
    transition_end: Option<usize>,
    ring_time: Option<Duration>,
    terminal: bool,
}

impl State {
    pub fn builder() -> StateBuilder {
        StateBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speech(&self) -> &str {
        &self.speech
    }

    pub fn ring_time(&self) -> Option<Duration> {
        self.ring_time
    }

    pub fn sounds(&self) -> &[usize] {
        &self.sounds
    }

    /// Returns a transition target ID or `None` for no
    /// transition.
    pub fn transition_for_input(&self, input: Input) -> Option<usize> {
        self.input_transitions.get(&input).copied()
    }

    /// Returns a transition target ID or `None` for no
    /// transition.
    pub fn transition_for_timeout(&self, enter_time: Instant) -> Option<usize> {
        if let Some((timeout_duration, timeout_target)) = self.timeout_transition.as_ref() {
            if enter_time.elapsed() > *timeout_duration {
                return Some(*timeout_target);
            }
        }

        None
    }

    pub fn transition_end(&self) -> Option<usize> {
        self.transition_end
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }
}

mod builder {
    use super::{Duration, Input, State};

    pub struct StateBuilder {
        state: State,
    }

    impl StateBuilder {
        pub fn new() -> Self {
            StateBuilder {
                state: Default::default(),
            }
        }

        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.state.name = name.into();
            self
        }

        pub fn speech(mut self, speech: impl Into<String>) -> Self {
            self.state.speech = speech.into();
            self
        }

        pub fn input(mut self, on_input: Input, transition_to: usize) -> Self {
            self.state.input_transitions.insert(on_input, transition_to);
            self
        }

        pub fn timeout(mut self, after_duration: Duration, transition_to: usize) -> Self {
            self.state.timeout_transition = Some((after_duration, transition_to));
            self
        }

        pub fn end(mut self, transition_to: usize) -> Self {
            self.state.transition_end = Some(transition_to);
            self
        }

        pub fn terminal(mut self, is_terminal: bool) -> Self {
            self.state.terminal = is_terminal;
            self
        }

        pub fn ring_for(mut self, max_duration: Duration) -> Self {
            self.state.ring_time = Some(max_duration);
            self
        }

        pub fn sounds(mut self, sounds: Vec<usize>) -> Self {
            self.state.sounds = sounds;
            self
        }

        pub fn build(self) -> State {
            self.state
        }
    }
}