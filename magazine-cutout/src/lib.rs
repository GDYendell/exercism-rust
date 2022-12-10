// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    if magazine.len() < note.len() {
        return false;
    }

    let mut cuttings: HashMap<String, i32> = HashMap::new();
    for word in magazine {
        *cuttings.entry(word.to_string()).or_insert(0) += 1;
    }

    for word in note {
        *cuttings.entry(word.to_string()).or_default() -= 1;
    }

    cuttings
        .iter()
        .all(|(word, count)| cuttings.get(word).unwrap_or(&0) >= &0)
}
