use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}