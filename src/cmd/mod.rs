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
    Copy(cmd_copy::Args),
    Paste(cmd_paste::Args),
}

pub fn parse_and_run() {
    let args = CmdArgs::parse();

    match &args.command {
        Some(Commands::Copy(args)) => cmd_copy::run(&args),
        Some(Commands::Paste(args)) => cmd_paste::run(&args),
        None => {
            println!("No subcommand given");
        }
    }
}
