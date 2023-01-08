// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words_available: HashMap<&str, u32> = HashMap::new();

    for word in magazine {
        words_available.entry(word).or_insert(0);
        words_available.insert(word, words_available[word] + 1);
    }

    for word in note {
        if words_available.contains_key(word) {
            let word_amount = words_available.get(word).unwrap().clone();

            if word_amount <= 0 {
                return false;
            }

            words_available.insert(word, word_amount - 1);
        } else {
            return false;
        }
    }

    true
}
