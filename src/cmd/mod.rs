mod cmd_one;
mod cmd_two;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct CmdArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Command one descritption")]
    One(cmd_one::Args),
    #[command(about = "Command two descritption")]
    Two(cmd_two::Args),
}

pub fn parse_and_run() {
    let args = CmdArgs::parse();

    match &args.command {
        Some(Commands::One(args)) => cmd_one::run(&args),
        Some(Commands::Two(args)) => cmd_two::run(&args),
        None => {
            println!("No subcommand given");
        }
    }
}
