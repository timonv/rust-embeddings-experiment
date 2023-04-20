use std::collections::HashMap;

// Function to compute the average word embedding for a bag of words.
fn average_word_embedding(
    bag_of_words: &[&str],
    word_embeddings: &HashMap<&str, [f64; 3]>,
) -> [f64; 3] {
    // Initialize total_embedding.
    let mut total_embedding = [0.0; 3];

    // Sum the embeddings of the words in the bag.
    bag_of_words
        .iter()
        .filter_map(|word| word_embeddings.get(word))
        .for_each(|embedding| {
            total_embedding
                .iter_mut()
                .zip(embedding.iter())
                .for_each(|(total, value)| *total += *value);
        });

    // Compute the average word embedding.
    let bag_size = bag_of_words.len() as f64;
    [
        total_embedding[0] / bag_size,
        total_embedding[1] / bag_size,
        total_embedding[2] / bag_size,
    ]
}

fn main() {
    let bag_of_words = vec!["apple", "orange", "banana", "apple", "grape"];

    let word_embeddings: HashMap<&str, [f64; 3]> = [
        ("apple", [1.0, 2.0, 3.0]),
        ("orange", [2.0, 3.0, 4.0]),
        ("banana", [3.0, 4.0, 5.0]),
        ("grape", [4.0, 5.0, 6.0]),
    ]
    .into();

    // Compute the average word embedding for the bag of words.
    let avg_embedding = average_word_embedding(&bag_of_words, &word_embeddings);

    println!("The average word embedding is: {:?}", avg_embedding);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn float_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_average_word_embedding() {
        let bag_of_words = vec!["apple", "orange", "banana", "apple", "grape"];
        let word_embeddings: HashMap<&str, [f64; 3]> = [
            ("apple", [1.0, 2.0, 3.0]),
            ("orange", [2.0, 3.0, 4.0]),
            ("banana", [3.0, 4.0, 5.0]),
            ("grape", [4.0, 5.0, 6.0]),
        ]
        .into();
        let avg_embedding = average_word_embedding(&bag_of_words, &word_embeddings);
        let expected_embedding = [2.2, 3.2, 4.2];
        let eps = 1e-9;
        assert!(float_eq(avg_embedding[0], expected_embedding[0], eps));
        assert!(float_eq(avg_embedding[1], expected_embedding[1], eps));
        assert!(float_eq(avg_embedding[2], expected_embedding[2], eps));
    }
}
