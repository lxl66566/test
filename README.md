# wordinfo

English | [简体中文](./docs/README.zh-CN.md)

wordinfo is a CLI tool to get information of words from web, including pronunciation, meaning, examples, etc.

It's the complement of [bakadict](https://github.com/flaribbit/bakadict): bakadict looks up words from local dictionary, while wordinfo gets word information from the web; bakadict supports only Japanese, while wordinfo supports multiple languages.

wordinfo allows you to write your own [CSS Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_selectors) to adapt word search to different languages and websites.

## Installation

Go to [Releases](https://github.com/lxl66566/wordinfo/releases) to download the platform-specific executable, or use [bpm](https://github.com/lxl66566/bpm) to install it.

## Usage

```sh
Usage: wordinfo.exe [OPTIONS] <WORDS>...
       wordinfo.exe <COMMAND>

Commands:
  config  config wordinfo, (not yet implemented)
  export  Export configuration file
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <WORDS>...  words to look up

Options:
  -e, --english              English (default)
  -j, --japanese             Japanese
  -s, --selector <SELECTOR>  Which selector to use
  -n, --no-url               Do not output url
  -h, --help                 Print help
  -V, --version              Print version

Examples:
wordinfo apple          # Query for English `apple`, = wordinfo -e apple
wordinfo -j すき        # Query Japanese `すき`, = wordinfo -j suki.
wordinfo -s oxf apple   # Specify to query apple using the oxford selector with abbreviation.
wordinfo show           # Show all available selectors.
```

wordinfo supports multiple languages, English is used by default. When querying Japanese, you can convert kana by typing romanization.

Selector specifying can be abbreviated, e.g. `wordinfo -s oxf ... ` is equivalent to `wordinfo -s oxford ... `.

## Configuration

Run `wordinfo export` to export the default configuration file to `~/.config/wordinfo.json` (Windows: `%userprofile%/.config/wordinfo.json`), and then edit the configuration items. You can choose _json_, _toml_, _yaml_ as the exported configuration format.

The currently adjustable items are:

<!-- prettier-ignore -->
| Key | Contents |
| :-: | :-: |
| default_language | Default language |
| color | Default color for each section of output |
| en | English CSS Selectors |
| jp | Japanese CSS Selectors | | other_languages | other_languages
| other_languages | Other languages and their CSS Selectors |
| delimiter_between_paragraphs | Separators for parts within each word |
| delimiter_between_words | Separators between words |

To adjust the default CSS Selector used, simply put it at the top of the current language array.

If you need to add a CSS Selector for a new language, you can change `other_languages` to refer to the following form (`<lang>` is the language identifier).

```json
"other_languages": {
    "<lang>": [
      ... Selectors
    ]
  }
```
