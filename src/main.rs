#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-keepass", about = "SigmaOS KeePass database manager")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List => println!("{}: no entries configured.", "sigmaos-keepass"),
        Commands::Status => println!("{}: ready.", "sigmaos-keepass"),
    }
}
