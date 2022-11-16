use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

type State = String;
type Symbol = String;
type StateArc = Arc<State>;
type SymbolArc = Arc<String>;

/// Implements the logic required for a finite state machine.
#[derive(Deserialize)]
#[serde(try_from = "SerializedStateMachine")]
pub struct StateMachine {
    /// All possible states.
    states: HashSet<StateArc>,
    /// All symbols that make up the state machine alphabet.
    alphabet: HashSet<SymbolArc>,
    /// All transitions supported by the state machine.
    transitions: HashMap<State, HashMap<Symbol, StateArc>>,
    /// The initial state of the state machie.
    initial_state: StateArc,
    /// The final state of the state machine.
    final_states: HashSet<State>,
}

impl StateMachine {
    /// Returns whether a sequence is accepted by the state machine.
    pub fn is_accepted<S>(&self, sequence: &[S]) -> bool
    where
        S: AsRef<str>,
    {
        let mut state: &str = &self.initial_state;

        for symbol in sequence.iter().map(|symbol| symbol.as_ref()) {
            let next_state = self
                .transitions
                .get(state)
                .and_then(|transitions| transitions.get(symbol))
                .map(|state| state.as_str());

            match next_state {
                Some(next_state) => state = next_state,
                None => return false,
            }
        }

        self.final_states.contains(state)
    }

    /// Returns an iterator over all states.
    pub fn iter_states(&self) -> impl Iterator<Item = &str> {
        self.states.iter().map(|state| state.as_str())
    }

    /// Returns an iterator over all symbols.
    pub fn iter_symbols(&self) -> impl Iterator<Item = &str> {
        self.alphabet.iter().map(|state| state.as_str())
    }

    /// Returns an iterator over all transitions, where each item is a tuple of the shape
    /// `(src_state, symbol, dst_state)`.
    pub fn iter_transitions(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.transitions.iter().flat_map(|(src_state, transitions)| {
            transitions.iter().map(|(symbol, dst_state)| {
                (src_state.as_str(), symbol.as_str(), dst_state.as_str())
            })
        })
    }

    /// Returns the initial state of the state machine.
    pub fn initial_state(&self) -> &str {
        self.initial_state.as_str()
    }

    /// Returns the final state of the state machine.
    pub fn final_states(&self) -> impl Iterator<Item = &str> {
        self.final_states.iter().map(|state| state.as_str())
    }
}

impl TryFrom<SerializedStateMachine> for StateMachine {
    type Error = StateMachineDeserError;

    fn try_from(mut machine: SerializedStateMachine) -> Result<Self, Self::Error> {
        let states = machine.states.drain().map(Arc::new).collect::<HashSet<_>>();

        let symbols = machine.alphabet.drain().map(Arc::new).collect::<HashSet<_>>();

        let mut transitions = HashMap::<State, HashMap<Symbol, StateArc>>::new();

        for transition in machine.transitions.drain() {
            let src_state = states
                .get(&transition.src_state)
                .ok_or_else(|| StateMachineDeserError::InvalidState(transition.src_state))?;

            let symbol = symbols
                .get(&transition.symbol)
                .ok_or_else(|| StateMachineDeserError::InvalidSymbol(transition.symbol))?;

            let dst_state = states
                .get(&transition.dst_state)
                .cloned()
                .ok_or_else(|| StateMachineDeserError::InvalidState(transition.dst_state))?;

            transitions
                .entry(src_state.as_str().to_owned())
                .or_default()
                .insert(symbol.as_str().to_owned(), dst_state.clone());
        }

        let initial_state = states
            .get(&machine.initial_state)
            .cloned()
            .ok_or_else(|| StateMachineDeserError::InvalidState(machine.initial_state))?;

        let mut final_states = HashSet::<State>::new();
        for state in machine.final_states {
            match states.get(&state) {
                Some(state) => {
                    final_states.insert(state.as_str().to_owned());
                }
                None => return Err(StateMachineDeserError::InvalidState(state)),
            }
        }

        Ok(StateMachine { states, alphabet: symbols, transitions, initial_state, final_states })
    }
}

/// Errors the may be returned when deserializing a state machine.
#[derive(Clone, Debug)]
pub enum StateMachineDeserError {
    /// The given state is invalid.
    InvalidState(String),
    /// The given symbol is invalid.
    InvalidSymbol(String),
}

impl Error for StateMachineDeserError {}

impl fmt::Display for StateMachineDeserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState(state) => write!(f, "Invalid state \"{}\"", state),
            Self::InvalidSymbol(symbol) => write!(f, "Invalid symbol \"{}\"", symbol),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
struct SerializedTrans {
    src_state: String,
    symbol: String,
    dst_state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SerializedStateMachine {
    states: HashSet<String>,
    alphabet: HashSet<String>,
    transitions: HashSet<SerializedTrans>,
    initial_state: String,
    final_states: HashSet<String>,
}
