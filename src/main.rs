use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cfcli", version = "1.0")]
// #[clap(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    name: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // "clones" a problem or set of problems (contest)
    Parse {
        remote: String,
    },
    // submits a specified problem or current problem
    Submit {

    },
    Version,
}

#[derive(Debug)]
struct Problem {
    contest_id: u32,
    problem_id: u32,
}

#[derive(Debug)]
struct Contest {
    problems: Vec<Problem>,
}

fn main() {
    let cli = Cli::parse();
  
    println!("Hello, {:?}!", cli);
}

