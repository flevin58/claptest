mod cmd_copy;
mod cmd_paste;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CmdArgs {
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
    let args = CmdArgs::parse();

    match &args.command {
        Some(Commands::Copy { src, dst }) => cmd_copy::run(src, dst),
        Some(Commands::Paste { what }) => cmd_paste::run(what),
        None => {
            println!("No subcommand given");
        }
    }
}
