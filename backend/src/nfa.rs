use std::collections::{HashMap, HashSet};

/// Marker for unsanitized input.
pub struct Unsanitary;

/// Marker for sanitized input.
pub struct Sanitary;

/// Nondeterministic finite automata.
#[derive(Deserialize, Serialize)]
pub struct Nfa<T> {
    /// Marker to ensure that the state machine has been validated before it is used for any
    /// computations.
    #[serde(skip_serializing, default)]
    sanitized: T,

    /// The final (accepting) states of the automata.
    final_states: HashSet<String>,

    /// The alphabet of symbols the automata accepts.
    alphabet: HashSet<String>,

    /// The nodes within the automata. Each node has a set of mappings from alphabet symbols to
    /// other states.
    nodes: HashMap<String, HashMap<String, HashSet<String>>>
}

impl Nfa<Unsanitary> {
    // todo -- add error type
    fn check(self) -> Result<Nfa<Sanitary>, ()> {
        unimplemented!()
    }
}

impl Nfa<Sanitary> {

}
