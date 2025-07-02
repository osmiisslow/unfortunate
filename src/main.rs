mod server;
mod add_quote;

use server::*;
use add_quote::*;
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

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


fn main() -> Result<()> {
    // set up more readable error handling 
    color_eyre::install()?;


    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Server { port}) => {
                match port {
                    Some(port) => start_server(port.parse().expect("not a valid port!")),
                    None => start_server(8080),
                }
            },
        Some(Commands::Add { quote }) => {
            add_quote(quote);
        }
        None => {
            println!("hai :3");
        }
    }

    Ok(())
}
