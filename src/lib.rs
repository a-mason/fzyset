use std::{collections::{HashSet, HashMap}, cmp::min, mem::swap};

fn levenshtein(str1: &str, str2: &str) -> usize {
    let mut prev = vec![0; str2.len()+1];
    let mut curr = vec![0; str2.len()+1];

    for i in 0..str2.len()+1 {
        prev[i] = i;
    }
    let mut str1_chars = str1.chars();
    for i in 0..str1.len() {
        let str1_i = str1_chars.next();
        let mut str2_chars = str2.chars();
        curr[0] = i + 1;
        for j in 0..str2.len() {
            let str2_j = str2_chars.next();
            let deletion_cost = prev[j+1] + 1;
            let insertion_cost = curr[j] + 1;
            let mut subsitution_cost = prev[j] + 1;
            if str1_i == str2_j {
                subsitution_cost = prev[j];
            }
            curr[j+1] = min(deletion_cost, insertion_cost);
            curr[j+1] = min(curr[j+1], subsitution_cost);
        }
        println!("{:?}", prev);
        swap(&mut prev, &mut curr);
    }
    println!("{:?}", prev);
    prev.pop().unwrap()
}

fn distance(str1: &str, str2: &str) -> f32 {
    let distance = levenshtein(str1, str2) as f32;
    if str1.len() > str2.len() {
        return 1.0 - (distance / str1.len() as f32);
    } else {
        return 1.0 - (distance / str2.len() as f32);
    }
}

enum ComparisonAlgorithm {
    Levenshtein,
    Other
}

struct FuzzySet {
    exact: HashSet<String>,
    match_map: HashMap<String, String>,
    alg: ComparisonAlgorithm,
    gram_size_lower: u8,
    gram_size_upper: u8,
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
}
