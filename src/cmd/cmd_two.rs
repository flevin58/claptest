use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        required = true,
        long, short,
        help = "Your selections",
        default_value_t = String::from("window"),
        value_parser = clap::builder::PossibleValuesParser::new(["window", "selection", "screen"]),
    )]
    what: String,
}

pub fn run(args: &Args) {
    println!("Command two choice is '{}'", args.what);
}
