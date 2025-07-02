mod server;
mod client;

use server::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="unfortunate")]
#[command(version, about="a fortune spinoff with quotes made by the discretion of the internet. Default quotes ripped from unfortunate", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// host a server to accept and send fortunes
    Server {
        /// port to host on, default 8080
        port: Option<String>
    },
    /// add a quote for other users to see at random!
    Add {
        /// the quote
        quote: String,
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Server { port}) => {
                match port {
                    Some(port) => server(port.parse().unwrap()),
                    None => server(8080),
                }
            },
        Some(Commands::Add { quote }) => {
            println!("add {quote}");
        }
        None => {
            println!("hai :3");
        }
    }
}
