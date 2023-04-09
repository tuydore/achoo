use clap::Parser;

use crate::{search::AcroynmSearcher, phrase::Phrase, words::{try_find_words_file, try_load_words_file}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct AcronymSearcherCli {
    /// Acronym must begin with this phrase.
    #[arg(short, long)]
    start: Option<String>,

    /// Acronym must end with this phrase.
    #[arg(short, long)]
    end: Option<String>,

    /// Phrases contained in the acronym. Each phrase is a sequence of space-separated words
    /// (a-zA-Z only) where some of the letters are capitalized. These will be included in the acronym.
    phrases: Vec<String>,

    /// Maximum number of allowed wildcards.
    #[arg(short = 'w', long, default_value_t = 0)]
    max_wildcards: usize,

    /// Maximum number of unmatched phrases during search.
    /// Does not include start and end, which are mandatory if specified.
    #[arg(short = 'u', long, default_value_t = 0)]
    max_unmatched_phrases: usize,

    /// Path to words file (newline separated). Will ignore case.
    /// Will attempt to find the words file on UNIX systems.
    #[arg(short = 'f', long)]
    words_file: Option<std::path::PathBuf>,
}

impl TryFrom<AcronymSearcherCli> for AcroynmSearcher {
    type Error = anyhow::Error;

    fn try_from(value: AcronymSearcherCli) -> Result<Self, Self::Error> {
        Ok(AcroynmSearcher {
            start: value.start.map(Phrase::new).transpose()?,
            end: value.end.map(Phrase::new).transpose()?,
            phrases: value.phrases.into_iter().map(Phrase::new).collect::<Result<Vec<_>, _>>()?,
            max_wildcards: value.max_wildcards,
            max_unmatched_phrases: value.max_unmatched_phrases,
            words: try_load_words_file(try_find_words_file(value.words_file)?)?,
        })
    }
}