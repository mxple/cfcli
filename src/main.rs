mod auth;
mod utils;

use utils::*;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cfcli", version = "1.0")]
// #[clap(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    // "clones" a problem or set of problems (contest)
    #[command(arg_required_else_help = true)]
    Parse {
        remote: ContestOrProblem,
    },
    // submits a specified problem or current problem
    #[command(arg_required_else_help = true)]
    Submit {
        problem: Problem,
    },
    Version,
}

fn main() {
    let args = Cli::parse();
    dbg!(&args);

    match args.command {
        Commands::Parse { remote } => match remote {
            ContestOrProblem::Contest(contest) => {
                todo!();
            }
            ContestOrProblem::Problem(problem) => {
                todo!();
            }
        },
        Commands::Submit { problem } => {
            todo!();
        }
        Commands::Version => {
            println!("1.0");
        }
    }
}
