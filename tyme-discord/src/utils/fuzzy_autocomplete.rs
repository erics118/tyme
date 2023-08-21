//! Utility functions for fuzzy autocomplete.

use anyhow::Result;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serenity::{self, builder::AutocompleteChoice};

/// Returns a list of autocomplete choices based on a fuzzy search.
pub fn fuzzy_autocomplete(cur: &str, values: &[&str]) -> Result<Vec<AutocompleteChoice>> {
    let matcher = SkimMatcherV2::default().ignore_case();

    let mut matches = Vec::new();

    for v in values.iter() {
        if let Some((score, _)) = matcher.fuzzy_indices(v, cur) {
            matches.push((score, v));
        }
    }

    matches.sort_by_key(|(score, _)| *score);
    matches.reverse();

    let mut choices = Vec::<AutocompleteChoice>::new();

    for (_, v) in matches.iter().take(25) {
        choices.push(AutocompleteChoice::new(v.to_string(), v.to_string()));
    }

    Ok(choices)
}
