use crate::automata::{Automaton, is_accepted};
use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2, Axis};
use rand::Rng;
use std::fmt;

// We'll just use the Decision Tree model to avoid dependency issues
pub struct ModelParams {
    pub name: String,
    pub max_depth: Option<usize>,
}

impl fmt::Display for ModelParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Generate random string
pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let symbols = ['a', 'b'];

    (0..length).map(|_| symbols[rng.gen_range(0..2)]).collect()
}

// Generate training data
pub fn generate_training_data(
    automaton: &Automaton,
    num_samples: usize,
    length: usize,
) -> (Array2<f64>, Array1<usize>) {
    let mut features = Array2::zeros((num_samples, length));
    let mut labels = Array1::zeros(num_samples);

    for i in 0..num_samples {
        let string = generate_random_string(length);
        let accepted = is_accepted(automaton, &string);

        // Convert string to feature vector ('a' -> 1.0, 'b' -> 0.0)
        for (j, c) in string.chars().enumerate() {
            features[[i, j]] = if c == 'a' { 1.0 } else { 0.0 };
        }

        labels[i] = if accepted { 1 } else { 0 };
    }

    (features, labels)
}

// Convert a string to a feature vector
pub fn string_to_features(input: &str) -> Array1<f64> {
    let mut features = Array1::zeros(input.len());

    for (i, c) in input.chars().enumerate() {
        features[i] = if c == 'a' { 1.0 } else { 0.0 };
    }

    features
}

// Train a decision tree model
pub fn train_model(
    features: &Array2<f64>,
    labels: &Array1<usize>,
    params: &ModelParams,
) -> Box<dyn Fn(&Array1<f64>) -> usize> {
    let dataset = Dataset::new(features.clone(), labels.clone());

    // Create a decision tree with the specified max_depth
    let model = DecisionTree::params()
        .max_depth(params.max_depth)
        .fit(&dataset)
        .unwrap();

    // Note: Decision tree visualization removed as export_dot is not available
    println!(
        "Trained {} model with max_depth: {:?}",
        params.name, params.max_depth
    );

    Box::new(move |features: &Array1<f64>| {
        let predictions = model.predict(&features.to_owned().insert_axis(Axis(0)));
        predictions[0]
    })
}

// Evaluate model accuracy on a test set
pub fn evaluate_model(
    model: &dyn Fn(&Array1<f64>) -> usize,
    automaton: &Automaton,
    num_samples: usize,
    length: usize,
) -> f64 {
    let mut correct = 0;

    for _ in 0..num_samples {
        let string = generate_random_string(length);
        let features = string_to_features(&string);

        let actual = is_accepted(automaton, &string);
        let predicted = model(&features) == 1;

        if actual == predicted {
            correct += 1;
        }
    }

    correct as f64 / num_samples as f64
}

// Evaluate model accuracy on strings of different lengths
pub fn evaluate_model_by_length(
    model: &dyn Fn(&Array1<f64>) -> usize,
    automaton: &Automaton,
    num_samples: usize,
    min_length: usize,
    max_length: usize,
) -> Vec<(usize, f64)> {
    let mut results = Vec::new();

    for length in min_length..=max_length {
        let accuracy = evaluate_model(model, automaton, num_samples, length);
        results.push((length, accuracy));
    }

    results
}
