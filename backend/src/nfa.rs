use std::collections::{HashMap, HashSet};

/// Marker for unsanitized input.
#[derive(Default)]
pub struct Unsanitary;

/// Marker for sanitized input.
pub struct Sanitary;

/// Nondeterministic finite automata.
#[derive(Deserialize, Serialize)]
pub struct Nfa<T> {
    /// Marker to ensure that the state machine has been validated before it is used for any
    /// computations.
    #[serde(skip_serializing, default)]
    _sanitized: T,

    /// The final (accepting) states of the automata.
    final_states: HashSet<String>,

    /// The alphabet of symbols the automata accepts.
    alphabet: HashSet<String>,

    /// The nodes within the automata. Each node has a set of mappings from alphabet symbols to
    /// other states.
    nodes: HashMap<String, HashMap<String, HashSet<String>>>,
}

impl Nfa<Unsanitary> {
    // todo -- add error type
    pub fn check(self) -> Result<Nfa<Sanitary>, ()> {
        let Nfa {
            final_states,
            alphabet,
            nodes,
            ..
        } = self;

        if !final_states.iter().all(|state| nodes.contains_key(state)) {
            return Err(());
        }

        if !nodes.iter().all(|(_node, maps)| {
            maps.iter().all(|(symbol, _)| alphabet.contains(symbol))
        }) {
            return Err(());
        }

        Ok(Nfa {
            _sanitized: Sanitary,
            final_states,
            alphabet,
            nodes,
        })
    }
}

impl Nfa<Sanitary> {}
