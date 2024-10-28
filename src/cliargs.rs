use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Copy {
        #[arg(long = "source", short, required = true, help = "The source file")]
        src: String,
        #[arg(
            long = "destination",
            short,
            required = true,
            help = "The destination file"
        )]
        dst: String,
    },
    Paste {
        #[arg(long, short, required = true, help = "What to copy")]
        what: String,
    },
}

pub fn parse_and_run() {
    let args = CliArgs::parse();

    match &args.command {
        Some(Commands::Copy { src, dst }) => {
            println!("You chose to copy {src} to {dst}");
        }
        Some(Commands::Paste { what }) => {
            println!("You chose to paste {what}");
        }
        None => {
            println!("No subcommand given");
        }
    }
}
