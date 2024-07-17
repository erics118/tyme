//! Utility functions for fuzzy autocomplete.

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

/// Returns a list of autocomplete choices based on a fuzzy search.
pub fn fuzzy_autocomplete<T: ToString + Copy + Clone>(cur: &str, values: &[T]) -> Vec<T> {
    let matcher = SkimMatcherV2::default().ignore_case();

    let mut matches = Vec::new();

    for v in values.iter().rev() {
        let v_str = v.to_string();
        if let Some((score, _)) = matcher.fuzzy_indices(&v_str, cur) {
            matches.push((score, v));
        }
    }

    matches.sort_by_key(|(score, _)| *score);
    matches.reverse();

    let mut choices = Vec::<T>::new();

    for (_, v) in matches.iter().take(25) {
        choices.push(**v);
    }

    choices
}
