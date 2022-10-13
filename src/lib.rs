use std::{
    cmp::min,
    collections::{hash_map::Entry, HashMap, HashSet},
    mem::swap,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref NON_WORD_REGEX: Regex =
        Regex::new(r"[^a-zA-Z0-9\u00C0-\u00FF\u0621-\u064A\u0660-\u0669, ]+").unwrap();
}

fn levenshtein(str1: &str, str2: &str) -> usize {
    let mut prev = vec![0; str2.len() + 1];
    let mut curr = vec![0; str2.len() + 1];

    for i in 0..str2.len() + 1 {
        prev[i] = i;
    }
    let mut str1_chars = str1.chars();
    for i in 0..str1.len() {
        let str1_i = str1_chars.next();
        let mut str2_chars = str2.chars();
        curr[0] = i + 1;
        for j in 0..str2.len() {
            let str2_j = str2_chars.next();
            let deletion_cost = prev[j + 1] + 1;
            let insertion_cost = curr[j] + 1;
            let mut subsitution_cost = prev[j] + 1;
            if str1_i == str2_j {
                subsitution_cost = prev[j];
            }
            curr[j + 1] = min(deletion_cost, insertion_cost);
            curr[j + 1] = min(curr[j + 1], subsitution_cost);
        }
        swap(&mut prev, &mut curr);
    }
    prev.pop().unwrap()
}

fn distance(str1: &str, str2: &str) -> f64 {
    let distance = levenshtein(str1, str2) as f64;
    if str1.len() > str2.len() {
        1.0 - (distance / str1.len() as f64)
    } else {
        1.0 - (distance / str2.len() as f64)
    }
}

fn iterate_grams(str: &str, gram_size: usize) -> Vec<String> {
    let mut word = format!("-{}-", NON_WORD_REGEX.replace(str, ""));
    let length_difference = gram_size as i32 - word.len() as i32;
    if length_difference > 0 {
        for _ in 0..length_difference {
            word.push('-');
        }
    };
    let mut results = Vec::with_capacity(word.len() - gram_size + 1);
    for i in 0..=(word.len() - gram_size) {
        results.push(word.chars().skip(i).take(gram_size).collect());
    }
    results
}

fn gram_counter(str: &str, gram_size: usize) -> HashMap<String, usize> {
    let mut grams = iterate_grams(str, gram_size);
    let mut gram_map = HashMap::with_capacity(grams.len());
    for gram in grams.drain(0..) {
        match gram_map.get(&gram) {
            Some(v) => gram_map.insert(gram, v + 1),
            None => gram_map.insert(gram, 1),
        };
    }
    gram_map
}

fn normalize_string(str: &str) -> String {
    str.to_lowercase()
}

pub enum ComparisonAlgorithm {
    Levenshtein,
    Other,
}

pub struct FuzzySet {
    exact: HashSet<String>,
    match_map: HashMap<String, Vec<(usize, usize)>>,
    items: HashMap<usize, Vec<(f64, String)>>,
    alg: ComparisonAlgorithm,
    gram_size_lower: usize,
    gram_size_upper: usize,
}

// var results = [];
// // start with high gram size and if there are no results, go to lower gram sizes
// for (var gramSize = this.gramSizeUpper; gramSize >= this.gramSizeLower; --gramSize) {
//     results = this.__get(value, gramSize, minMatchScore);
//     if (results && results.length > 0) {
//         return results;
//     }
// }
// return null;

impl FuzzySet {
    pub fn new(lower: usize, upper: usize, alg: ComparisonAlgorithm) -> Self {
        FuzzySet {
            exact: HashSet::new(),
            match_map: HashMap::new(),
            items: HashMap::new(),
            alg,
            gram_size_lower: lower,
            gram_size_upper: upper,
        }
    }

    fn get_per_gram(
        &self,
        key: &str,
        gram_size: usize,
        min_match: f64,
    ) -> Option<Vec<(f64, String)>> {
        let normalized = normalize_string(key);
        let mut matches = HashMap::new();
        let gram_counts = gram_counter(key, gram_size);
        let items = self.items.get(&gram_size).unwrap();
        let mut sum_of_squares = 0;
        for (gram, count) in gram_counts {
            sum_of_squares += usize::pow(count, 2);
            match self.match_map.get(&gram) {
                Some(v) => {
                    for (idx, gram_count) in v {
                        match matches.get(idx) {
                            Some(mch) => {
                                matches.insert(*idx, mch + (count * gram_count));
                            }
                            None => {
                                matches.insert(*idx, count * gram_count);
                            }
                        };
                    }
                }
                None => {}
            }
        }
        if matches.is_empty() {
            return None;
        }
        let vector_normal = f64::sqrt(sum_of_squares as f64);
        let mut results = Vec::new();
        for (index, score) in matches {
            results.push((
                score as f64 / (vector_normal * items[index].0),
                &items[index].1,
            ));
        }
        results.sort_by(|a, b| b.0.total_cmp(&a.0));
        match self.alg {
            ComparisonAlgorithm::Levenshtein => {
                // Arbitrary truncation
                for res in results.iter_mut() {
                    *res = (distance(&res.1, &normalized), res.1);
                }
                results.sort_by(|a, b| b.0.total_cmp(&a.0));
            }
            _ => {}
        }

        Some(
            results
                .iter()
                .filter_map(|r| {
                    if r.0 >= min_match {
                        return Some((r.0, self.exact.get(r.1).unwrap().to_owned()));
                    }
                    None
                })
                .collect(),
        )
    }

    pub fn get(&self, key: &str, min_match: f64) -> Option<Vec<(f64, String)>> {
        for i in (self.gram_size_lower..=self.gram_size_upper).rev() {
            match self.get_per_gram(key, i, min_match) {
                Some(v) => {
                    return Some(v);
                }
                None => {}
            }
        }
        None
    }

    pub fn insert(&mut self, key: &str) -> bool {
        let normalized = normalize_string(key);
        if self.exact.contains(&normalized) {
            return false;
        };
        for i in self.gram_size_lower..=self.gram_size_upper {
            let mut empty = Vec::new();
            let items = self.items.get_mut(&i).unwrap_or(&mut empty);
            let gram_count = gram_counter(&normalized, i);
            let mut sum_of_squares = 0;
            for (gram, count) in gram_count {
                sum_of_squares += usize::pow(count, 2);
                match self.match_map.entry(gram) {
                    Entry::Vacant(e) => {
                        e.insert(vec![(items.len(), count)]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((items.len(), count));
                    }
                }
            }
            items.push((f64::sqrt(sum_of_squares as f64), normalized.clone()));
            if items.len() == 1 {
                self.items.insert(i, empty);
            }
        }
        self.exact.insert(normalized);
        true
    }

    pub fn len(&self) -> usize {
        self.exact.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levenshtein_swap_brand() {
        assert_eq!(levenshtein("brand", "swap"), 4);
    }
    #[test]
    fn levenshtein_saturday_sunday() {
        assert_eq!(levenshtein("saturday", "sunday"), 3);
    }
    #[test]
    fn levenshtein_sunday_saturday() {
        assert_eq!(levenshtein("sunday", "saturday"), 3);
    }
    #[test]
    fn levenshtein_kitten_sitting() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }
    #[test]
    fn levenshtein_a_giraffe() {
        assert_eq!(levenshtein("a", "giraffe"), 6);
    }

    #[test]
    fn fzy_set_exact() {
        let mut set = FuzzySet::new(2, 4, ComparisonAlgorithm::Levenshtein);
        set.insert("michael axiak");
        assert_eq!(set.get("michael axiak", 0.0).unwrap()[0].0, 1.0);
    }

    #[test]
    fn fzy_set_michael_axiak() {
        let mut set = FuzzySet::new(2, 4, ComparisonAlgorithm::Levenshtein);
        set.insert("michael axiak");
        assert_eq!(
            set.get("micael asiak", 0.0).unwrap()[0].0,
            0.8461538461538461
        );
    }

    #[test]
    fn fzy_set_multipl() {
        let mut set = FuzzySet::new(2, 4, ComparisonAlgorithm::Levenshtein);
        set.insert("michael axiak");
        set.insert("michael aziak");
        let results = set.get("micael asiak", 0.0).unwrap();
        assert_eq!(results[0].0, 0.8461538461538461);
        assert_eq!(results[1].0, 0.8461538461538461);
    }
}
