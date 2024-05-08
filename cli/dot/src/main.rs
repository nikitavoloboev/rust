use clap::Parser;

#[macro_use]
extern crate log_macro;

#[derive(Parser, Debug)]
struct Args {
    // Define your command line arguments here
    #[clap(short, long)]
    example: String,
}

fn main() {
    let args = Args::parse();
    log!(args.example);
}
