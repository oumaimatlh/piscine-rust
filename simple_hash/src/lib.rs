use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut frequency_count = HashMap::new();

    for &word in words {
        *frequency_count.entry(word).or_insert(0) += 1;
    }

    frequency_count
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::{nb_distinct_words, word_frequency_counter};

    #[test]
    fn counts_repetitions_and_distinct_words() {
        let words = ["rust", "is", "fun", "rust"];
        let counts = word_frequency_counter(&words);

        assert_eq!(counts.get("rust"), Some(&2));
        assert_eq!(counts.get("is"), Some(&1));
        assert_eq!(nb_distinct_words(&counts), 3);
    }

    #[test]
    fn handles_empty_input() {
        let counts = word_frequency_counter(&[]);

        assert!(counts.is_empty());
        assert_eq!(nb_distinct_words(&counts), 0);
    }
}
