use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None, after_help = r#"Examples:
wordinfo apple          # Query for English `apple`, = wordinfo -e apple
wordinfo -j すき        # Query Japanese `すき`, = wordinfo -j suki.
wordinfo -s oxf apple   # Specify to query apple using the oxford selector.
"#)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Cli {
    /// words to look up
    #[clap(required = true)]
    pub words: Vec<String>,
    /// info English words
    #[arg(short, long)]
    #[clap(conflicts_with_all = &["japanese"])]
    pub english: bool,
    /// info Japanese words
    #[arg(short, long)]
    #[clap(conflicts_with_all = &["english"])]
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
}

/// Subcommand
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// config wordinfo, WIP
    Config(Config),
    /// export default config file
    Export,
}

#[derive(Args, Debug, Serialize, Clone)]
pub struct Config {
    #[clap(required = true)]
    key: String,
    #[clap(required = true)]
    value: String,
}

lazy_static! {
    pub static ref CLI: Cli = Cli::parse();
}

impl Cli {
    pub fn get_cli_language(&self) -> Option<&'static str> {
        if self.english {
            Some("en")
        } else if self.japanese {
            Some("jp")
        } else {
            None
        }
    }
}
