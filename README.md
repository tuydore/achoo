# ACHOO! - Acronym Chooser

Simple acronym chooser, based on phrases (i.e. you can input `"Technical Term"` for matching `tt` in words) and supporting mandatory start/end and wildcards.

By default, `achoo` tries to pick up the words file on UNIX systems. On Windows, you will have to supply your own.

# Usage

```sh
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

# Examples
Output is first grouped and sorted by number of letters in the matched word, then by word alphabetically.

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