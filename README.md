# wordinfo

[English] | [简体中文](./docs/README.zh-CN.md)

wordinfo is a CLI tool to get information of words from web, including pronunciation, meaning, examples, etc.

It's the complement of [bakadict](https://github.com/flaribbit/bakadict): bakadict looks up words from local dictionary, while wordinfo gets word information from the web; bakadict supports only Japanese, while wordinfo supports multiple languages. (Now English and Japanese only, but can be extended to support more languages easily.)

wordinfo allows you to write your own [CSS Selector](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_selectors) to adapt word search to different languages and websites.

## Installation

Go to [Releases](https://github.com/lxl66566/wordinfo/releases) to download the platform-specific executable, or use [bpm](https://github.com/lxl66566/bpm) to install it.

## Use

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
wordinfo -s oxf apple   # Specify to query apple using the oxford selector.
```

wordinfo supports multiple languages, English is used by default. When querying Japanese, you can convert kana by typing romanization.

`wordinfo -s anything.. anything..` (any meaningless words) looks up the currently available CSS Selectors, and can be abbreviated when specifying a CSS Selector, e.g. `wordinfo -s oxf ... ` is equivalent to `wordinfo -s oxford ... `.

## Configuration

Run `wordinfo export` to export the default configuration file to `~/.config/wordinfo.toml` (Windows: `%userprofile%/.config/wordinfo.toml`), and edit the configuration items.

The currently adjustable items are:

```
default_language: default language
color:            default color for each section of the output
en:               English CSS Selectors
jp:               Japanese CSS Selectors

```

To adjust the default CSS Selector used, just put it at the top. (Each of the three consecutive `[[en]]`, `[en.selector]`, `[en.delimiter]` belongs to one CSS Selector as a whole, and need to be moved together)
