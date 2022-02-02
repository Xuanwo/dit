use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Status,
    Commit,
    Add { name: Option<String> },
    Push,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name } => println!("Hello, {}!", name.unwrap_or("World".to_string())),
        _ => todo!()
    }
}