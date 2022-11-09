use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct StateIndex(pub usize);

#[derive(Clone, Copy, Debug)]
pub struct SymbolIndex(pub usize);

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub transitions: Vec<Trans>,
}

#[derive(Clone, Debug)]
pub struct Trans {
    pub symbol: SymbolIndex,
    pub dst_state: StateIndex,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "SerializedStateMachine")]
pub struct StateMachine {
    states: Vec<State>,
    alphabet: Vec<String>,
    initial_state: StateIndex,
    final_state: StateIndex,
}

impl TryFrom<SerializedStateMachine> for StateMachine {
    type Error = StateMachineDeserError;

    fn try_from(mut machine: SerializedStateMachine) -> Result<Self, Self::Error> {
        let mut state_indexes = HashMap::<String, usize>::new();
        let mut alphabet_indexes = HashMap::<String, usize>::new();

        let mut states = machine
            .states
            .drain()
            .inspect(|name| {
                state_indexes.insert(name.to_string(), state_indexes.len());
            })
            .map(|name| State { name, transitions: Vec::new() })
            .collect::<Vec<_>>();

        let alphabet = machine
            .alphabet
            .drain()
            .inspect(|symbol| {
                alphabet_indexes.insert(symbol.to_string(), state_indexes.len());
            })
            .collect::<Vec<_>>();

        for trans in machine.transitions.drain() {
            let src_state = state_indexes
                .get(&trans.src_state)
                .copied()
                .map(StateIndex)
                .ok_or_else(|| StateMachineDeserError::InvalidState(trans.src_state))?;

            let symbol = state_indexes
                .get(&trans.symbol)
                .copied()
                .map(SymbolIndex)
                .ok_or_else(|| StateMachineDeserError::InvalidSymbol(trans.symbol))?;

            let dst_state = state_indexes
                .get(&trans.dst_state)
                .copied()
                .map(StateIndex)
                .ok_or_else(|| StateMachineDeserError::InvalidState(trans.dst_state))?;

            states[src_state.0].transitions.push(Trans { symbol, dst_state });
        }

        let initial_state = state_indexes
            .get(&machine.initial_state)
            .copied()
            .map(StateIndex)
            .ok_or_else(|| StateMachineDeserError::InvalidState(machine.initial_state))?;

        let final_state = state_indexes
            .get(&machine.final_state)
            .copied()
            .map(StateIndex)
            .ok_or_else(|| StateMachineDeserError::InvalidState(machine.final_state))?;

        Ok(StateMachine { states, alphabet, initial_state, final_state })
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
