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
#[derive(Debug, Serialize, Deserialize)]
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

impl Dfa {
    /// Determines which states in the DFA are equivalent. Returns the set of (sorted) pairs of
    /// equivalent states.
    fn lint_states<'a>(&'a self) -> HashSet<(&'a String, &'a String)> {
        let mut output: HashSet<_> = self.nodes
            .keys()
            .cartesian_product(self.nodes.keys())
            .filter(|&(ref x, ref y)| x < y)
            .collect();
        let mut work: VecDeque<_> = self.final_states
            .iter()
            .cartesian_product(self.nodes.keys())
            .filter(|&(_, right)| !self.final_states.contains(right))
            .map(|(left, right)| if left < right {
                (left, right)
            } else {
                (right, left)
            })
            .inspect(|node| { output.remove(node); })
            .collect();

        // construct reversed graph
        let mut backtrack: HashMap<_, HashMap<&String, _>> = self.nodes
            .iter()
            .map(|(node, _)| (node, HashMap::new()))
            .collect();
        for (state, transforms) in &self.nodes {
            for (letter, new_state) in transforms {
                backtrack
                    .get_mut(&new_state)
                    .unwrap()
                    .entry(&letter)
                    .or_insert(HashSet::new())
                    .insert(state);
            }
        }

        while let Some(node) = work.pop_front() {
            for letter in &self.alphabet {
                let (left, right) = node;
                if let (Some(left), Some(right)) =
                    (backtrack[left].get(letter), backtrack[right].get(letter))
                {
                    for left in left {
                        for right in right {
                            if left < right {
                                if output.remove(&(*left, *right)) {
                                    work.push_back((left, right));
                                }
                            } else {
                                if output.remove(&(*right, *left)) {
                                    work.push_back((right, left));
                                }
                            }
                        }
                    }
                }
            }
        }

        output
    }

    /// Minimises the DFA.
    pub fn minimise(mut self) -> Self {
        // construct reversed graph
        let mut backtrack: HashMap<_, HashMap<String, _>> = self.nodes
            .iter()
            .map(|(node, _)| (node.to_owned(), HashMap::new()))
            .collect();
        for (state, transforms) in &self.nodes {
            for (letter, new_state) in transforms {
                backtrack
                    .get_mut(new_state.as_str())
                    .unwrap()
                    .entry(letter.to_owned())
                    .or_insert(HashSet::new())
                    .insert(state.to_owned());
            }
        }

        let mut renames = HashMap::new();

        // TODO -- make this allocate a _little_ less
        let redundancies: HashSet<_> = self.lint_states()
            .into_iter()
            .map(|(left, right)| (left.to_owned(), right.to_owned()))
            .collect();

        for (left, right) in redundancies {
            if !self.nodes.contains_key(&left) || !self.nodes.contains_key(&right) {
                continue;
            }

            for (letter, states) in &backtrack[&right] {
                for state in states {
                    if let Some(transitions) = self.nodes.get_mut(state.as_str()) {
                        transitions.insert(letter.to_owned(), left.to_owned());
                    }
                }
            }

            let new_name = match renames.get(&left) {
                Some(name) => format!("{} | {}", name, right),
                None => format!("{} | {}", left, right),
            };

            renames.insert(left.to_owned(), new_name);

            if self.start == right {
                self.start = left;
            }

            self.final_states.remove(&right);

            self.nodes.remove(&right);
        }

        if renames.contains_key(&self.start) {
            self.start = renames[&self.start].to_owned();
        }

        self.final_states = self.final_states
            .into_iter()
            .map(|x| {
                renames.get(x.as_str()).map(|y| y.to_owned()).unwrap_or(x)
            })
            .collect();

        self.nodes = self.nodes
            .into_iter()
            .map(|(state, transitions)| {
                (
                    state,
                    transitions
                        .into_iter()
                        .map(|(letter, new_state)| {
                            (
                                letter,
                                renames
                                    .get(new_state.as_str())
                                    .map(|y| y.to_owned())
                                    .unwrap_or(new_state),
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        for (old, new) in renames {
            if let Some(val) = self.nodes.remove(&old) {
                self.nodes.insert(new, val);
            }
        }

        self
    }
}

#[test]
pub fn lint_dfa_minimised() {
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
    let dfa = nfa.make_deterministic();
    assert!(dfa.lint_states().is_empty())
}

#[test]
pub fn lint_dfa_unminimised() {
    let input = r#"{
        "start": "1",
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["2"],
                "b": ["3"]
            },
            "2": {},
            "3": {
                "a": ["4"],
                "b": ["1"]
            },
            "4": {}
        },
        "final_states": ["2", "4"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    let nfa = unsanitary.check().unwrap();
    let dfa = nfa.make_deterministic();
    assert_eq!(
        dfa.lint_states()
            .into_iter()
            .map(|(x, y)| (x.to_owned(), y.to_owned()))
            .collect::<HashSet<_>>(),
        vec![("1".into(), "3".into()), ("2".into(), "4".into())]
            .into_iter()
            .collect()
    )
}

#[test]
pub fn optimise_dfa() {
    let input = r#"{
        "start": "1",
        "alphabet": ["a", "b"],
        "nodes": {
            "1": {
                "a": ["2"],
                "b": ["3"]
            },
            "2": {},
            "3": {
                "a": ["4"],
                "b": ["1"]
            },
            "4": {}
        },
        "final_states": ["2", "4"]
    }"#;
    let unsanitary: Nfa<_> = serde_json::from_str(input).unwrap();
    let nfa = unsanitary.check().unwrap();
    let dfa = nfa.make_deterministic();
    panic!("Output: {:?}", dfa.minimise());
}
