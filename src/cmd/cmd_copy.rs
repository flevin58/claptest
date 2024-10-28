use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long = "source", short, required = true, help = "The source file")]
    src: String,
    #[arg(
        long = "destination",
        short,
        required = true,
        help = "The destination file"
    )]
    dst: String,
}

pub fn run(args: &Args) {
    println!("You chose to copy '{}' to '{}'", args.src, args.dst);
}
