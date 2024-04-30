use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut anagrams = HashSet::new();
    'possible_anagrams: for &p in possible_anagrams {
        let p_lower = p.to_lowercase();
        let mut available_chars = word.to_lowercase();
        if p_lower.len() != word.len() || p_lower == available_chars {
            continue 'possible_anagrams;
        }

        for c in p_lower.chars() {
            let i = available_chars.find(&c.to_string());
            match i {
                None => continue 'possible_anagrams,
                Some(i) => available_chars.remove(i),
            };
        }

        anagrams.insert(p);
    }

    anagrams
}
