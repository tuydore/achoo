use clap::Parser;
use cli::AcronymSearcherCli;
use search::AcroynmSearcher;

use crate::search::pretty_print_results;

pub(crate) mod phrase;
pub(crate) mod search;
pub(crate) mod cli;
pub(crate) mod words;


fn main() -> Result<(), anyhow::Error> {
    if std::env::args().len() == 1 {
        println!("Bless you!");
        return Ok(())
    }

    let cli = AcronymSearcherCli::parse();
    let searcher = AcroynmSearcher::try_from(cli)?;

    let results = searcher.search();
    pretty_print_results(&results);

    Ok(())
}
