pub mod commands;
pub mod utils;

use clap::{Parser, Subcommand};
use commands::*;
use utils::*;

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

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    state::try_write_config(&state::Config::new());

    let config = state::try_read_config().unwrap();
    dbg!(&config);

    match args.command {
        Commands::Parse { remote } => {
            parse::parse(&remote, &config).await;
        }
        Commands::Submit { problem } => {
            todo!();
        }
        Commands::Version => {
            println!("1.0");
        }
    }
}
