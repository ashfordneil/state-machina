use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

use serde::de::{Deserialize, Deserializer};

use itertools::Itertools;

#[cfg(test)]
use serde_json;

/// Marker for unsanitized input.
#[derive(Debug)]
pub struct Unsanitary;

impl<'de> Deserialize<'de> for Unsanitary {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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

    /// Start state.
    start: String,

    /// The final (accepting) states of the automata.
    final_states: HashSet<String>,

    /// The alphabet of symbols the automata accepts.
    alphabet: HashSet<String>,

    /// The nodes within the automata. Each node has mappings from alphabet symbols to sets of
    /// other states.
    nodes: HashMap<String, HashMap<String, HashSet<String>>>,
}

impl Nfa<Unsanitary> {
    /// Ensures that the NFA is valid, and that relevant invariants within the structure hold.
    pub fn check(self) -> Result<Nfa<Sanitary>, NfaError> {
        let Nfa {
            start,
            final_states,
            alphabet,
            nodes,
            ..
        } = self;

        // ensure that the start state is a valid state
        if !nodes.contains_key(&start) {
            return Err(NfaError::UnknownState(start));
        }

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
            start,
            final_states,
            alphabet,
            nodes,
        })
    }
}

#[test]
fn valid_nfa() {
    let input = r#"{
        "start": "1",
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
        "start": "1",
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
        err @ _ => panic!(err),
    }
}

#[test]
fn invalid_nfa_state_transition() {
    let input = r#"{
        "start": "1",
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
        err @ _ => panic!(err),
    }
}

#[test]
fn invalid_nfa_alphabet() {
    let input = r#"{
        "start": "1",
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
        err @ _ => panic!(err),
    }
}

#[test]
fn invalid_nfa_start_state() {
    let input = r#"{
        "start": "4",
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
    match unsanitary.check().unwrap_err() {
        NfaError::UnknownState(err) => assert_eq!(err, "4"),
        err @ _ => panic!(err),
    }
}

fn hash_states<'a, I>(states: I) -> String
where
    I: IntoIterator<Item = &'a String>,
{
    states
        .into_iter()
        .map(|x| x.as_str())
        .intersperse(" + ")
        .collect()
}

impl Nfa<Sanitary> {
    pub fn make_deterministic(self) -> Dfa {
        let Nfa {
            alphabet,
            start: nfa_start,
            nodes: nfa_nodes,
            final_states: nfa_final_states,
            ..
        } = self;
        let mut work = VecDeque::new();
        let mut final_states = HashSet::new();
        let mut nodes = HashMap::new();

        let start = hash_states(iter::once(&nfa_start));
        work.push_back(vec![nfa_start]);

        while let Some(node) = work.pop_front() {
            let dfa_state = hash_states(&node);
            assert!(!nodes.contains_key(&dfa_state));
            let transition_table = alphabet
                .iter()
                .map(|letter| {
                    let transition = node.iter()
                        .filter_map(|state| nfa_nodes.get(state))
                        .filter_map(|transitions| transitions.get(letter.as_str()))
                        .flatten()
                        .map(|x| x.to_owned())
                        .unique()
                        .sorted();
                    let transition_state = hash_states(&transition);
                    if transition != node && !nodes.contains_key(&transition_state) &&
                        !work.contains(&transition)
                    {
                        work.push_back(transition)
                    }
                    (letter.to_owned(), transition_state)
                })
                .collect();

            if node.iter().any(|state| nfa_final_states.contains(state)) {
                final_states.insert(dfa_state.to_owned());
            }

            nodes.insert(dfa_state, transition_table);
        }

        Dfa {
            final_states,
            start,
            alphabet,
            nodes,
        }
    }
}

#[test]
fn basic_deterministic_conversion() {
    let input = r#"{
        "start": "1",
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
            "3": {}
        },
        "final_states": ["3"]
        }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    let nfa = unsanitary.check().unwrap();
    let Dfa {
        final_states,
        start,
        alphabet,
        nodes,
    } = nfa.make_deterministic();
    assert_eq!(
        final_states,
        vec!["1 + 2 + 3".into(), "1 + 3".into()]
            .into_iter()
            .collect()
    );
    assert_eq!(start, "1".to_owned());
    assert_eq!(alphabet, vec!["a".into(), "b".into()].into_iter().collect());
    assert_eq!(
        nodes,
        vec![
            (
                "1".into(),
                vec![("a".into(), "1 + 2".into()), ("b".into(), "1".into())]
                    .into_iter()
                    .collect(),
            ),
            (
                "1 + 2".into(),
                vec![
                    ("a".into(), "1 + 2 + 3".into()),
                    ("b".into(), "1 + 3".into()),
                ].into_iter()
                    .collect(),
            ),
            (
                "1 + 2 + 3".into(),
                vec![
                    ("a".into(), "1 + 2 + 3".into()),
                    ("b".into(), "1 + 3".into()),
                ].into_iter()
                    .collect(),
            ),
            (
                "1 + 3".into(),
                vec![("a".into(), "1 + 2".into()), ("b".into(), "1".into())]
                    .into_iter()
                    .collect(),
            ),
        ].into_iter()
            .collect()
    );
}

/// Deterministic finite automata.
#[derive(Debug, Serialize)]
pub struct Dfa {
    /// The final (accepting) states of the automata.
    final_states: HashSet<String>,

    /// Start state.
    start: String,

    /// The alphabet of symbols the automata accepts.
    alphabet: HashSet<String>,

    /// The nodes within the automata. Each node has mappings from alphabet symbols to transition
    /// states.
    nodes: HashMap<String, HashMap<String, String>>,
}
