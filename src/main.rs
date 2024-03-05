mod cli;
mod config;
mod core;

use cli::CLI;
use core::info::info;

#[tokio::main]
async fn main() {
    // info word
    if CLI.command.is_none() {
        return info(CLI.words.clone()).await;
    }
}
