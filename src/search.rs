use std::{collections::BTreeSet, cmp::Ordering};

use crate::phrase::Phrase;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SearchResult {
    /// Phrases in order of matching, or None if left as wildcard.
    phrases: Vec<Option<Phrase>>,

    /// Number of unused phrases.
    num_unused_phrases: usize,

    /// Word that matched the search.
    word: String,
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.word.len().partial_cmp(&other.word.len())? {
            Ordering::Equal => self.word.partial_cmp(&other.word),
            ordering => Some(ordering)
        }
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.word.len().cmp(&other.word.len()) {
            Ordering::Equal => self.word.cmp(&other.word),
            ordering => ordering
        }
    }
}

impl SearchResult {
    /// Add a phrase to the start of the search result.
    pub(crate) fn add_start_phrase(&mut self, phrase: Phrase) {
        self.word.insert_str(0, &phrase.pattern);
        self.phrases.insert(0, Some(phrase));
    } 

    /// Add a phrase to the end of the search result.
    pub(crate) fn add_end_phrase(&mut self, phrase: Phrase) {
        self.word.push_str(&phrase.pattern);
        self.phrases.push(Some(phrase));
    }

    pub(crate) fn num_wildcards(&self) -> usize {
        self.phrases
            .iter()
            .filter(|phrase| phrase.is_none())
            .count()
    }

    /// Returns the phrase with wildcard hints filled in from the matching word.
    pub(crate) fn phrase_info(&self) -> String {
        let mut s = String::new();
        let mut i = 0;

        for phrase in self.phrases.iter() {

            // if we match a phrase, add it to the string and advance the index by its pattern length
            if let Some(p) = phrase {
                for word in &p.words {
                    s.push_str(word);
                    s.push(' ');
                }
                i += p.pattern.len();

            // otherwise print that character in the word and advance the index by one
            } else {
                let c = self.word
                    .chars()
                    .nth(i)
                    .expect("should not be OOB")
                    .to_uppercase()
                    .next()
                    .expect("character should have uppercase variant");

                s.push(c);
                s.push_str(".. ");
                i += 1;
            }
        }

        s
    }
}

#[derive(Debug)]
pub(crate) struct AcroynmSearcher {
    /// Acronym must begin with this phrase.
    pub(crate) start: Option<Phrase>,

    /// Acronym must end with this phrase.
    pub(crate) end: Option<Phrase>,

    /// Phrases contained in the acronym.
    pub(crate) phrases: Vec<Phrase>,

    /// Maximum number of wildcards allowed.
    pub(crate) max_wildcards: usize,

    /// Maximum number of unmatched phrases allowed.
    pub(crate) max_unmatched_phrases: usize,

    /// List of words to search in.
    pub(crate) words: Vec<String>,
}

impl AcroynmSearcher {
    // Strip words of the start and end phrases, if any are given.
    fn stripped_words(&self) -> Vec<String> {
        self.words
            .iter()
            .filter_map(|word| {
                let mut stripped: &str = word;
                if let Some(start) = &self.start {
                    stripped = stripped.strip_prefix(&start.pattern)?;
                }
                if let Some(end) = &self.end {
                    stripped = stripped.strip_suffix(&end.pattern)?;
                }
                Some(stripped.to_owned())
            })
            .collect()
    }

    // Add the start and end phrases to a result, if any are given.
    fn add_start_and_end_to_result(&self, mut result: SearchResult) -> SearchResult {
        if let Some(phrase) = &self.start {
            result.add_start_phrase(phrase.clone());
        }
        if let Some(phrase) = &self.end {
            result.add_end_phrase(phrase.clone())
        }
        result
    }

    pub(crate) fn search(&self) -> Vec<SearchResult> {
        // create a copy of the words, filtering out the start and end
        self.stripped_words()
            .iter()
            // create an iterator of results based on stripped words
            .flat_map(|word| search_unchecked(&self.phrases, word)
                // filter by wildcards and unmatched phrases
                .filter(|result| {
                    result.num_wildcards() <= self.max_wildcards && result.num_unused_phrases <= self.max_unmatched_phrases
                })
            )
            // add back start and end phrases
            .map(|result| self.add_start_and_end_to_result(result))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

/// Finds all possible word matches and returns them as a `Vec<Vec<Option<usize>>>`, as follows:
/// 
/// * the outer `Vec` contains all matches
/// * the inner `Vec` has the same length as the word
/// * each `Option<usize>` contains either the index of the phrase that matched at that location
///     or None if the characted is left as a wildcard
/// 
/// For example, for phrases `["ab", "b", "c"]` and the word `"ababc"`, this will return `[00X12]` and `[X1002]`.
fn find_word_matches(phrases: &[Phrase], word: &str) -> Vec<Vec<Option<usize>>> {
    // start with a single match of all-wildcards
    let mut iter_vec: Vec<Vec<Option<usize>>> = vec![vec![None; word.len()]];
    let mut push_vec: Vec<Vec<Option<usize>>> = Vec::new();

    for (phrase_idx, phrase) in phrases.iter().enumerate() {

        // iterate over all potential matches
        for word_match in iter_vec.drain(..) {
            
            // find all occurences of the phrase's search substring in the word,
            // where the current potential match is not None at that index
            for word_match_idx in word.match_indices(&phrase.pattern)
                .map(|(idx, _)| idx)
                .filter(|idx| word_match[*idx].is_none())
            {
                    
                // copy the current word match and add the current phrase to the new index
                // add that to the push vector
                let mut extended_word_match = word_match.clone();
                for idx in extended_word_match.iter_mut()
                    .skip(word_match_idx)
                    .take(phrase.pattern.len()) 
                {
                    *idx = Some(phrase_idx);
                }
                push_vec.push(extended_word_match);
            }
        }

        // when all current word matches have been processed, swap the vectors around
        (iter_vec, push_vec) = (push_vec, iter_vec);
    }

    // at the end, the iter vec should contain all search results
    iter_vec
}

/// Deduplicates the word match, removing only identical `Some` values (which are guaranteed to be
/// consecutive) and leaving all `None` in place.
fn deduplicate_word_match(word_match: &mut Vec<Option<usize>>) {
    word_match.dedup_by(|a, b| {
        if let Some(av) = a {
            if let Some(bv) = b {
                return av == bv;
            }
        }
        false
    });
}

/// Attempts to match a sequence of phrases with a word. Assumes the word is not empty and only lowercase a-z.
fn search_unchecked<'a>(phrases: &'a [Phrase], word: &'a str) -> impl Iterator<Item = SearchResult> + 'a {
    find_word_matches(phrases, word).into_iter().map(|mut word_match| {
        // remove only duplicate non-wildcards
        deduplicate_word_match(&mut word_match);

        SearchResult {
            num_unused_phrases: phrases.len() - word_match.iter()
                .filter(|m| m.is_some())
                .count(),
            phrases: word_match.into_iter()
                .map(|idx| idx.map(|i| phrases[i].clone()))
                .collect(),
            word: word.to_owned(),
        }
    })
}

/// Pretty-print results to STDOUT.
pub(crate) fn pretty_print_results(results: &[SearchResult]) {
    if results.is_empty() {
        println!("No results found.");
        return;
    }

    let mut prev_len = None;
    for result in results.iter() {
        let word_len = result.word.len();

        // separate results by a whitespace
        if let Some(pl) = prev_len.as_mut() {
            if word_len > *pl {
                println!("\n{word_len} LETTERS");
                *pl = word_len;
            }
        } else {
            println!("{word_len} LETTERS");
            prev_len = Some(word_len)
        }

        // write the pattern and pad right with whitespace
        print!("{}", result.word);

        // write the word definition
        println!(" : {}", result.phrase_info());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_matches() {
        let phrases = vec![
            Phrase { words: Vec::new(), pattern: "ab".to_string() },
            Phrase { words: Vec::new(), pattern: "b".to_string() },
            Phrase { words: Vec::new(), pattern: "c".to_string() },
        ];
        let word = "ababc";
        
        let result = find_word_matches(&phrases, word);
        let expected = vec![
            vec![Some(0), Some(0), None, Some(1), Some(2)],
            vec![None, Some(1), Some(0), Some(0), Some(2)],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deduplication() {
        let mut vec = vec![Some(0), Some(0), None, None, Some(1), Some(1), None];
        deduplicate_word_match(&mut vec);
        let expected = vec![Some(0), None, None, Some(1), None];
        assert_eq!(vec, expected);
    }
}