use clap::{Args, Parser, Subcommand, ValueEnum};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None, after_help = r#"Examples:
wordinfo apple          # Query for English `apple`, = wordinfo -e apple
wordinfo -j すき        # Query Japanese `すき`, = wordinfo -j suki.
wordinfo -s oxf apple   # Specify to query apple using the oxford selector.
wordinfo show           # Show all available selectors.
"#)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Cli {
    /// words to look up
    #[clap(required = true)]
    pub words: Vec<String>,
    /// info English words
    #[arg(short, long)]
    #[clap(conflicts_with_all = &["japanese", "lang"])]
    pub english: bool,
    /// info Japanese words
    #[arg(short, long)]
    #[clap(conflicts_with_all = &["english", "lang"])]
    pub japanese: bool,
    /// Which selector to use
    #[arg(short, long)]
    pub selector: Option<String>,
    /// subcommand
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// do not output url
    #[arg(short, long)]
    pub no_url: bool,
    /// specify language
    #[arg(short, long)]
    #[clap(conflicts_with_all = &["english", "japanese"])]
    pub lang: Option<String>,
}

/// Subcommand
#[derive(Subcommand, Clone)]
pub enum Commands {
    /// config wordinfo, WIP
    Config(Config),
    /// export default config file
    Export(Export),
    /// show all selectors
    Show,
}

#[derive(Args, Debug, Serialize, Clone)]
pub struct Config {
    #[clap(required = true)]
    key: String,
    #[clap(required = true)]
    value: String,
}

#[derive(Args, Clone)]
pub struct Export {
    /// config export format
    #[arg(short, long)]
    #[clap(default_value = "json")]
    pub format: Format,
}

#[derive(ValueEnum, Clone, Default, Debug)]
pub enum Format {
    #[default]
    Json,
    Toml,
    Yaml,
}

lazy_static! {
    pub static ref CLI: Cli = Cli::parse();
}

impl Cli {
    pub fn get_cli_language(&self) -> Option<&str> {
        if self.english {
            Some("en")
        } else if self.japanese {
            Some("jp")
        } else {
            self.lang.as_deref()
        }
    }
}
