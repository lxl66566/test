use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None, after_help = r#"Examples:
"#)]
pub struct Cli {
    /// The words to info
    #[clap(required = true)]
    pub words: Vec<String>,
    /// English words
    #[arg(short, long)]
    pub english: bool,
    /// Japanese words
    #[arg(short, long)]
    pub japanese: bool,
    /// Which selector to use
    #[arg(short, long)]
    pub selector: Option<String>,
    /// subcommand
    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommand
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Config(Config),
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
