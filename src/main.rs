// USE https://docs.rs/futures/latest/futures/future/fn.join_all.html
mod config;
mod core;
mod request;
use anyhow::{Ok, Result};
use request::request;
#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", request("test").await?);
    Ok(())
}
