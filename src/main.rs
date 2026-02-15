#![allow(clippy::missing_errors_doc)]
use clap::Parser;

use crate::req::encode_query;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    query: String,
    #[arg(short, long, default_value_t = 5)]
    count: usize,
    #[arg(short, long)]
    open: bool,
}

pub mod engines;
pub mod req;
pub mod search_result;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .filter(Some("quick"), log::LevelFilter::Trace)
        .init();

    // let arg = std::env::args().nth(1); // (program name)
    // if arg == quick:
    // try parsing subcommands
    // else:
    // if arg == search (or some alias)
    // parse search subcommand
    // else
    // fallback to parsing subcommands

    let args = Args::parse();
    let query = encode_query(&args.query);
    let results = req::search_duckduckgo(&query, args.count)?;
    for r in results {
        println!("{r}");
    }

    Ok(())
}
