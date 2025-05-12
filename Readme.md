
1. Build a finite automaton with three states
2. Generate training data
3. Train a decision tree model
4. Predict whether the string "aab" is accepted
5. Check if an accepting state is reachable

## Components

### Finite Automaton

The automaton consists of three states:
- S0: Initial state (non-accepting)
- S1: Intermediate state (non-accepting)
- S2: Final state (accepting)

And four transitions:
- S0 → S1 on symbol 'a'
- S1 → S2 on symbol 'b'
- S2 → S0 on symbol 'a'
- S2 → S1 on symbol 'b'

This automaton accepts strings that end in state S2, which happens when the string ends with the pattern "ab" or when it follows certain other patterns.

### Machine Learning Integration

The ML component:
1. Generates random strings
2. Checks if each string is accepted by the automaton
3. Converts strings to feature vectors ('a' → 1.0, 'b' → 0.0)
4. Trains a decision tree to predict acceptance
5. Uses the trained model to predict whether new strings will be accepted

### Model Checking

The model checking component verifies properties of the automaton:
- "reachable_accepting": Checks if an accepting state is reachable from the initial state

## Technical Details

### String Acceptance

A string is accepted by the automaton if, after processing all characters, the automaton ends in an accepting state (S2). The process starts at the initial state (S0) and follows transitions based on each character in the string.

### Feature Encoding

Strings are encoded as feature vectors:
- 'a' is encoded as 1.0
- 'b' is encoded as 0.0

For example, "aab" becomes [1.0, 1.0, 0.0].

### Decision Tree Learning

The decision tree learns to classify strings based on patterns in the feature vectors that correlate with acceptance or rejection by the automaton.

## Dependencies

- `petgraph`: For graph data structures and algorithms
- `linfa` and `linfa-trees`: For machine learning (decision trees)
- `ndarray`: For numerical computations
- `rand`: For random string generation

## Expected Output

When running the program with the test string "aab", you should see output similar to:

\`\`\`
Prediction for 'aab': 1
Model checking result for 'reachable_accepting': true
\`\`\`

This indicates that:
1. The ML model predicts that "aab" will be accepted by the automaton
2. The model checking confirms that an accepting state is reachable
