use std::{collections::{HashSet, HashMap}, cmp::min, mem::swap};

fn levenshtein(str1: &str, str2: &str) -> usize {
    let mut prev = Vec::with_capacity(str2.len());
    let mut curr = Vec::with_capacity(str2.len());

    for i in 0..str2.len() {
        prev[i] = i;
    }
    let mut str1_chars = str1.chars();
    for i in 0..str1.len() {
        let str1_i = str1_chars.next();
        let mut str2_chars = str2.chars();
        curr[0] = i + 1;
        for j in 0..(str2.len()-1) {
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
        swap(&mut prev, &mut curr);
    }
    curr[str2.len()]

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
    fn it_works() {
        assert_eq!(1, 1);
    }
}
