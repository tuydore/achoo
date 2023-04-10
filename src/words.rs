//! Utilities for finding word collections.

use std::{io::BufRead, path::PathBuf};

use anyhow::Context;

#[cfg(unix)]
/// Possible locations of the UNIX words file.
const WORDS_FILE_PATHS: [&str; 2] = ["/usr/share/dict/words", "/usr/dict/words"];

#[cfg(unix)]
/// Attempt to locate the UNIX words file.
fn try_find_unix_words_file() -> Result<PathBuf, anyhow::Error> {
    use std::str::FromStr;

    for path in WORDS_FILE_PATHS
        .into_iter()
        .map(|path| PathBuf::from_str(path).expect("infallible"))
    {
        if path.is_file() {
            return Ok(path);
        }
    }
    Err(anyhow::anyhow!(
        "could not find UNIX words file at {} or {}",
        WORDS_FILE_PATHS[0],
        WORDS_FILE_PATHS[1]
    ))
}

/// Try and find the default system words file.
fn try_find_default_words_file() -> Result<PathBuf, anyhow::Error> {
    if cfg!(unix) {
        try_find_unix_words_file()
    } else {
        // QUESTION: does such a file exist on other OSes?
        Err(anyhow::anyhow!("platform does not have a default words file"))
    }
}

/// Verify the existence of the given words file or attempt to find a system default.
pub(crate) fn try_find_words_file(words_file: Option<PathBuf>) -> Result<PathBuf, anyhow::Error> {
    if let Some(filepath) = words_file {
        if filepath.is_file() {
            Ok(filepath)
        } else {
            Err(anyhow::anyhow!("file does not exist: {filepath:?}"))
        }
    } else {
        try_find_default_words_file()
    }
}

/// Load words into a vector from the word file, filtering those that are empty and/or not made up of a-zA-Z.
/// Returns an error if the resulting word vector is empty.
pub(crate) fn try_load_words_file(words_file: PathBuf) -> Result<Vec<String>, anyhow::Error> {
    let file = std::fs::File::open(words_file).context("error loading words file")?;
    std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.context("error reading line from words file")
                // TODO: toggle capitalization
                .map(|s| s.to_lowercase())
        })
        .collect()
}
