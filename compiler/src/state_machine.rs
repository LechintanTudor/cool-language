use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

type State = String;
type Symbol = String;
type StateArc = Arc<State>;
type SymbolArc = Arc<String>;

#[derive(Deserialize)]
#[serde(try_from = "SerializedStateMachine")]
pub struct StateMachine {
    states: HashSet<StateArc>,
    alphabet: HashSet<SymbolArc>,
    transitions: HashMap<State, HashMap<String, StateArc>>,
    initial_state: StateArc,
    final_state: StateArc,
}

impl StateMachine {
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

        state == self.final_state.as_str()
    }

    pub fn iter_states(&self) -> impl Iterator<Item = &str> {
        self.states.iter().map(|state| state.as_str())
    }

    pub fn iter_symbols(&self) -> impl Iterator<Item = &str> {
        self.alphabet.iter().map(|state| state.as_str())
    }

    pub fn iter_transitions(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.transitions.iter().flat_map(|(src_state, transitions)| {
            transitions.iter().map(|(symbol, dst_state)| {
                (src_state.as_str(), symbol.as_str(), dst_state.as_str())
            })
        })
    }

    pub fn initial_state(&self) -> &str {
        self.initial_state.as_str()
    }

    pub fn final_state(&self) -> &str {
        self.final_state.as_str()
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

        let final_state = states
            .get(&machine.final_state)
            .cloned()
            .ok_or_else(|| StateMachineDeserError::InvalidState(machine.final_state))?;

        Ok(StateMachine { states, alphabet: symbols, transitions, initial_state, final_state })
    }
}

#[derive(Clone, Debug)]
pub enum StateMachineDeserError {
    InvalidState(String),
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
    final_state: String,
}
