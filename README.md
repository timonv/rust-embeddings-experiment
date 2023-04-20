# Bag of Words Embedding in Rust

This Rust project demonstrates how to compute the average word embedding for a given bag of words using a HashMap of word embeddings.

## Features

- Compute the average word embedding for a given bag of words.
- Use of standard Rust data structures and idiomatic Rust code.
- Test included for the `average_word_embedding` function.

## Usage

Clone the repository:

```bash
git clone https://github.com/timonv/rust-embeddings-experiment.git
cd bag-of-words-embedding-rust
```

Build and run the project:

```bash
cargo build
cargo run
```

To run the tests:

```bash
cargo test
```

## Example

The following example computes the average word embedding for a bag of words:

```rust
let bag_of_words = vec!["apple", "orange", "banana", "apple", "grape"];
let word_embeddings: HashMap<&str, [f64; 3]> = [
    ("apple", [1.0, 2.0, 3.0]),
    ("orange", [2.0, 3.0, 4.0]),
    ("banana", [3.0, 4.0, 5.0]),
    ("grape", [4.0, 5.0, 6.0]),
].into();
let avg_embedding = average_word_embedding(&bag_of_words, &word_embeddings);
println!("The average word embedding is: {:?}", avg_embedding);
```

Output:

```
The average word embedding is: [2.2, 3.2, 4.2]
```

## License

This project is licensed under the MIT License.
