mod init;
mod add;
mod commit;

use clap::{Parser, Subcommand};
use init::OptionInit;
use add::OptionAdd;
use commit::OptionCommit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(OptionAdd),
    Commit(OptionCommit),
    Init(OptionInit),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(v) => println!("Add, {:?}!", v),
        Commands::Init(v) => println!("Init, {:?}!", v),
        _ => todo!()
    }
}
