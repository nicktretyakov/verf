mod automata;
mod ml;

use automata::{
    build_contains_aba_automaton, build_even_a_automaton, build_simple_automaton, model_check,
    visualize_automaton,
};
use ml::{
    ModelParams, evaluate_model, evaluate_model_by_length, generate_training_data,
    string_to_features, train_model,
};

fn main() -> std::io::Result<()> {
    println!("=== Finite Automaton with Machine Learning and Model Checking ===\n");

    // Create different automata
    let automata = [
        ("Simple", build_simple_automaton()),
        ("Even A's", build_even_a_automaton()),
        ("Contains ABA", build_contains_aba_automaton()),
    ];

    // ML model configurations to evaluate
    let model_params = [
        ModelParams {
            name: "Decision Tree (Depth 3)".to_string(),
            max_depth: Some(3),
        },
        ModelParams {
            name: "Decision Tree (Depth 5)".to_string(),
            max_depth: Some(5),
        },
        ModelParams {
            name: "Decision Tree (Unlimited)".to_string(),
            max_depth: None,
        },
    ];

    // Properties to check
    let properties = [
        "reachable_accepting",
        "all_states_reachable",
        "deadlock_free",
        "deterministic",
    ];

    // Test strings
    let test_strings = ["aab", "abba", "aba", "baba", "aabb"];

    // Process each automaton
    for (name, automaton) in &automata {
        println!("\n=== {} Automaton ===", name);

        // Visualize the automaton
        let dot_filename = format!("{}_automaton.dot", name.to_lowercase().replace(" ", "_"));
        visualize_automaton(automaton, &dot_filename)?;

        // Model checking
        println!("\nModel Checking Results:");
        for property in &properties {
            let result = model_check(automaton, property);
            println!("  {}: {}", property, result);
        }

        // Generate training data
        let (features, labels) = generate_training_data(automaton, 1000, 5);

        // Train and evaluate different ML models
        println!("\nMachine Learning Models:");
        for params in &model_params {
            println!("\n  {} Model:", params);

            // Train the model
            let model = train_model(&features, &labels, params);

            // Test on specific strings
            println!("    Predictions:");
            for test_string in &test_strings {
                let features = string_to_features(test_string);
                let prediction = model(&features);
                println!("      '{}': {}", test_string, prediction);
            }

            // Evaluate accuracy
            let accuracy = evaluate_model(&model, automaton, 500, 5);
            println!("    Accuracy on test set: {:.2}%", accuracy * 100.0);

            // Evaluate on different string lengths
            println!("    Accuracy by string length:");
            let length_results = evaluate_model_by_length(&model, automaton, 200, 1, 10);
            for (length, acc) in length_results {
                println!("      Length {}: {:.2}%", length, acc * 100.0);
            }
        }
    }

    Ok(())
}
