use std::collections::{HashMap, HashSet};

use serde::de::{Deserialize, Deserializer};

#[cfg(test)]
use serde_json;

/// Marker for unsanitized input.
pub struct Unsanitary;

impl<'de> Deserialize<'de> for Unsanitary {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Unsanitary)
    }
}

/// Marker for sanitized input.
#[derive(Debug)]
pub struct Sanitary;

/// Used to represent errors
#[derive(Debug)]
pub enum NfaError {
    UnknownState(String),
    UnknownSymbol(String),
}

/// Nondeterministic finite automata.
#[derive(Debug, Deserialize, Serialize)]
pub struct Nfa<T> {
    /// Marker to ensure that the state machine has been validated before it is used for any
    /// computations.
    #[serde(skip_serializing)]
    _sanitized: T,

    /// The final (accepting) states of the automata.
    final_states: HashSet<String>,

    /// The alphabet of symbols the automata accepts.
    alphabet: HashSet<String>,

    /// The nodes within the automata. Each node has mappings from alphabet symbols to other states.
    nodes: HashMap<String, HashMap<String, HashSet<String>>>,
}

impl Nfa<Unsanitary> {
    /// Ensures that the NFA is valid, and that relevant invariants within the structure hold.
    pub fn check(self) -> Result<Nfa<Sanitary>, NfaError> {
        let Nfa {
            final_states,
            alphabet,
            nodes,
            ..
        } = self;

        // ensure that all final states are listed as actual states
        if let Some(unknown_state) = final_states
            .iter()
            .find(|&state| !nodes.contains_key(state))
        {
            return Err(NfaError::UnknownState(unknown_state.to_owned()));
        }

        // ensure that all state transitions are on valid symbols
        if let Some((unknown_symbol, _)) = nodes
            .iter()
            .filter_map(|(_, maps)| {
                maps.iter().find(|&(symbol, _)| !alphabet.contains(symbol))
            })
            .next()
        {
            return Err(NfaError::UnknownSymbol(unknown_symbol.to_owned()));
        }

        // ensure that all state transitions are to valid states
        if let Some(unknown_state) = nodes
            .iter()
            .filter_map(|(_, maps)| {
                maps.iter()
                    .filter_map(|(_, states)| {
                        states.iter().find(|&state| !nodes.contains_key(state))
                    })
                    .next()
            })
            .next()
        {
            return Err(NfaError::UnknownState(unknown_state.to_owned()));
        }

        Ok(Nfa {
            _sanitized: Sanitary,
            final_states,
            alphabet,
            nodes,
        })
    }
}

#[test]
fn valid_nfa() {
    let input = r#"{
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["1", "2"],
                "b": ["1"]
            },
            "2": {
                "a": ["3"],
                "b": ["3"]
            },
            "3": {
                "a": ["1"],
                "b": ["2"]
            }
        },
        "final_states": ["3"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    unsanitary.check().unwrap();
}

#[test]
fn invalid_nfa_final_state() {
    let input = r#"{
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["1", "2"],
                "b": ["1"]
            },
            "2": {
                "a": ["3"],
                "b": ["3"]
            },
            "3": {
                "a": ["1"],
                "b": ["2"]
            }
        },
        "final_states": ["3", "4"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    match unsanitary.check().unwrap_err() {
        NfaError::UnknownState(err) => assert_eq!(err, "4"),
        err @ _ => panic!(err)
    }
}

#[test]
fn invalid_nfa_state_transition() {
    let input = r#"{
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["1", "2"],
                "b": ["1"]
            },
            "2": {
                "a": ["3"],
                "b": ["3"]
            },
            "3": {
                "a": ["1"],
                "b": ["2", "4"]
            }
        },
        "final_states": ["3"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    match unsanitary.check().unwrap_err() {
        NfaError::UnknownState(err) => assert_eq!(err, "4"),
        err @ _ => panic!(err)
    }
}

#[test]
fn invalid_nfa_alphabet() {
    let input = r#"{
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["1", "2"],
                "b": ["1"],
                "c": ["1"]
            },
            "2": {
                "a": ["3"],
                "b": ["3"]
            },
            "3": {
                "a": ["1"],
                "b": ["2"]
            }
        },
        "final_states": ["3"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    match unsanitary.check().unwrap_err() {
        NfaError::UnknownSymbol(err) => assert_eq!(err, "c"),
        err @ _ => panic!(err)
    }
}
