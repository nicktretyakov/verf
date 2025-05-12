use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef; // Import EdgeRef trait
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub is_accepting: bool,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub symbol: char,
}

pub type Automaton = DiGraph<State, Transition>;

// Original simple automaton
pub fn build_simple_automaton() -> Automaton {
    let mut graph = DiGraph::new();

    // Add states
    let s0 = graph.add_node(State {
        name: "S0".to_string(),
        is_accepting: false,
    });

    let s1 = graph.add_node(State {
        name: "S1".to_string(),
        is_accepting: false,
    });

    let s2 = graph.add_node(State {
        name: "S2".to_string(),
        is_accepting: true,
    });

    // Add transitions
    graph.add_edge(s0, s1, Transition { symbol: 'a' });
    graph.add_edge(s1, s2, Transition { symbol: 'b' });
    graph.add_edge(s2, s0, Transition { symbol: 'a' });
    graph.add_edge(s2, s1, Transition { symbol: 'b' });

    graph
}

// More complex automaton: recognizes strings with an even number of 'a's
pub fn build_even_a_automaton() -> Automaton {
    let mut graph = DiGraph::new();

    // Add states: even_state is accepting (even number of 'a's seen)
    let even_state = graph.add_node(State {
        name: "Even".to_string(),
        is_accepting: true,
    });

    let odd_state = graph.add_node(State {
        name: "Odd".to_string(),
        is_accepting: false,
    });

    // Add transitions
    // On 'a': toggle between even and odd
    graph.add_edge(even_state, odd_state, Transition { symbol: 'a' });
    graph.add_edge(odd_state, even_state, Transition { symbol: 'a' });

    // On 'b': stay in the same state
    graph.add_edge(even_state, even_state, Transition { symbol: 'b' });
    graph.add_edge(odd_state, odd_state, Transition { symbol: 'b' });

    graph
}

// Complex automaton: recognizes strings that contain the substring "aba"
pub fn build_contains_aba_automaton() -> Automaton {
    let mut graph = DiGraph::new();

    // States represent progress toward recognizing "aba"
    let s0 = graph.add_node(State {
        name: "S0".to_string(), // Initial state
        is_accepting: false,
    });

    let s1 = graph.add_node(State {
        name: "S1".to_string(), // Seen 'a'
        is_accepting: false,
    });

    let s2 = graph.add_node(State {
        name: "S2".to_string(), // Seen 'ab'
        is_accepting: false,
    });

    let s3 = graph.add_node(State {
        name: "S3".to_string(), // Seen 'aba' - accepting state
        is_accepting: true,
    });

    // Transitions
    // From S0
    graph.add_edge(s0, s1, Transition { symbol: 'a' });
    graph.add_edge(s0, s0, Transition { symbol: 'b' });

    // From S1
    graph.add_edge(s1, s1, Transition { symbol: 'a' });
    graph.add_edge(s1, s2, Transition { symbol: 'b' });

    // From S2
    graph.add_edge(s2, s3, Transition { symbol: 'a' });
    graph.add_edge(s2, s0, Transition { symbol: 'b' });

    // From S3 (once we've seen "aba", we stay in the accepting state)
    graph.add_edge(s3, s3, Transition { symbol: 'a' });
    graph.add_edge(s3, s3, Transition { symbol: 'b' });

    graph
}

// Check if a string is accepted by the automaton
pub fn is_accepted(automaton: &Automaton, input: &str) -> bool {
    let mut current_state = NodeIndex::new(0); // Start at the first state

    for c in input.chars() {
        let mut found_transition = false;

        for edge in automaton.edges(current_state) {
            if edge.weight().symbol == c {
                current_state = edge.target();
                found_transition = true;
                break;
            }
        }

        if !found_transition {
            return false; // No valid transition found
        }
    }

    // Check if the final state is accepting
    automaton[current_state].is_accepting
}

// Visualize the automaton by generating a DOT file
pub fn visualize_automaton(automaton: &Automaton, filename: &str) -> std::io::Result<()> {
    // Custom DOT representation with transition labels
    let mut dot_string = format!("digraph {{\n");

    // Add nodes
    for node_idx in automaton.node_indices() {
        let state = &automaton[node_idx];
        let shape = if state.is_accepting {
            "doublecircle"
        } else {
            "circle"
        };
        dot_string.push_str(&format!(
            "    {} [label=\"{}\", shape={}];\n",
            node_idx.index(),
            state.name,
            shape
        ));
    }

    // Add edges with transition symbols
    for edge in automaton.edge_references() {
        dot_string.push_str(&format!(
            "    {} -> {} [label=\"{}\"];\n",
            edge.source().index(),
            edge.target().index(),
            edge.weight().symbol
        ));
    }

    dot_string.push_str("}\n");

    // Write to file
    let mut file = File::create(filename)?;
    file.write_all(dot_string.as_bytes())?;

    println!("Automaton visualization saved to {}", filename);
    println!(
        "To view it, install Graphviz and run: dot -Tpng {} -o {}.png",
        filename, filename
    );

    Ok(())
}

// Additional model checking properties
pub fn model_check(automaton: &Automaton, property: &str) -> bool {
    use petgraph::visit::{Bfs, Dfs};

    match property {
        "reachable_accepting" => {
            let mut dfs = Dfs::new(automaton, NodeIndex::new(0));
            while let Some(node) = dfs.next(automaton) {
                if automaton[node].is_accepting {
                    return true;
                }
            }
            false
        }
        "all_states_reachable" => {
            let start = NodeIndex::new(0);
            let mut bfs = Bfs::new(automaton, start);
            let mut reachable = HashSet::new();

            while let Some(node) = bfs.next(automaton) {
                reachable.insert(node);
            }

            reachable.len() == automaton.node_count()
        }
        "deadlock_free" => {
            for node in automaton.node_indices() {
                if automaton.edges(node).count() == 0 {
                    return false; // Found a state with no outgoing transitions
                }
            }
            true
        }
        "deterministic" => {
            for node in automaton.node_indices() {
                let mut symbols = HashSet::new();
                for edge in automaton.edges(node) {
                    let symbol = edge.weight().symbol;
                    if symbols.contains(&symbol) {
                        return false; // Found a state with multiple transitions on the same symbol
                    }
                    symbols.insert(symbol);
                }
            }
            true
        }
        _ => false,
    }
}
