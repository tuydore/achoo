# **ACHOO!** 🤧

Achoo is an acronym chooser, designed to help find real words that are suitable acronyms for your project's name.

Achoo uses *phrases* instead of words: a phrase is a sequence of ASCII alphabetic words (optionally separated by spaces), where some of the letters are capitalized. These letters make up your acronym. For example, `"Company"` will match the letter `c`, but `"COmpany"` will match the sequence `co` and `"Company Name"` will match the sequence `cn` instead of the individual comprising letters.

Achoo also provides additional features, such as:

- strict matching of the start and end of the acroynm using `--start` and `--end` flags
- wildcards
- printing out the matched sequence of phrases for each acroynm, to help visualize what other words might fit the wildcards

By default, `achoo` tries to pick up the [words file](https://en.wikipedia.org/wiki/Words_(Unix)) on UNIX systems. On Windows, you will have to supply your own.

## Usage

```txt
❯ achoo --help
Acronym Chooser.

Usage: achoo [OPTIONS] [PHRASES]...

Arguments:
  [PHRASES]...  Phrases contained in the acronym. Each phrase is a sequence of space-separated words (a-zA-Z only) where some of the letters are capitalized. These will be included in the acronym

Options:
  -s, --start <START>
          Acronym must begin with this phrase
  -e, --end <END>
          Acronym must end with this phrase
  -w, --max-wildcards <MAX_WILDCARDS>
          Maximum number of allowed wildcards [default: 0]
  -u, --max-unmatched-phrases <MAX_UNMATCHED_PHRASES>
          Maximum number of unmatched phrases during search. Does not include start and end, which are mandatory if specified [default: 0]
  -f, --words-file <WORDS_FILE>
          Path to words file (newline separated). Will ignore case. Will attempt to find the words file on UNIX systems
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples
Output is first sorted and grouped by number of letters in the matched word, then by word alphabetically.

```sh
❯ achoo \
    --start "Company" \
    --max-wildcards 3 \
    --max-unmatched-phrases 3 \
    Buzzword "Technical Term" Amazing
7 LETTERS
cattabu : Company Amazing Technical Term A.. Buzzword U.. 
cattabu : Company A.. Technical Term Amazing Buzzword U.. 

8 LETTERS
cottabus : Company O.. Technical Term Amazing Buzzword U.. S.. 
cuttable : Company U.. Technical Term Amazing Buzzword L.. E.. 
```