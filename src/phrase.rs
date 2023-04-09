/// Phrase that makes up part of the acronym.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Phrase {
    /// Cache the vector of words for printing in final output.
    pub(crate) words: Vec<String>,

    /// Pre-computed vector of characters, for matching words. All lowercase.
    pub(crate) pattern: String,
}

impl Phrase {
    pub(crate) fn new(phrase: String) -> Result<Self, anyhow::Error> {
        let mut search = Vec::new();

        for c in phrase.chars() {
            if !(c.is_alphabetic() || c == ' ') {
                return Err(anyhow::anyhow!("phrase characters can only be a-zA-Z or space: {phrase}"))

            // space is not valid uppercase, so every character at this point is either a-z or A-Z
            // and thus has a lowecase variant
            } else if c.is_uppercase() {
                search.push(c.to_lowercase().next().expect("must have lowercase variant"));
            }
        }

        if search.is_empty() {
            return Err(anyhow::anyhow!("phrase must have at least one capital letter: {phrase}"))
        }

        Ok(Self { 
            words: phrase.split(' ')
                .map(|word| word.to_owned())
                .collect(), 
            pattern: search.into_iter().collect()
        })
    }
}