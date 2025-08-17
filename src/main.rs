use clap::Parser;
use cli_arguments::TracepixCliArguments;

mod cli_arguments;

fn main() {
    let args = TracepixCliArguments::parse();

    println!("{:#?}", args);
}
