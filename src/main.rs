extern crate core;

mod chat;

use std::ffi::OsString;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "rakun")] // requires `clap` feature
#[clap(about = "A Multi Agent System CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start Agent
    #[clap(arg_required_else_help = true)]
    Start {
        /// Starting Agent Name
        name: String,
        /// optional: Host to connect
        host: Option<String>,
    },
    /// Stop Agent
    #[clap(arg_required_else_help = true)]
    Stop {
        /// Stopping Agent Name
        name: String,
    },
    Client(Client),
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct Client {
    #[clap(subcommand)]
    command: Option<ClientCommands>,

    #[clap(short, long, value_parser)]
    name: String,

    host: Option<String>,
}

#[derive(Debug, Subcommand)]
enum ClientCommands {
    /// Get Agent Status
    Status,
    /// Chat with Agent
    Chat {
        /// Agent message
        message: String,
    },
}

#[derive(Debug, Args)]
struct StashPush {
    #[clap(short, long)]
    message: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Start { name, host } => {
            println!("Starting Agent {} on {}", name, host.unwrap_or_else(|| "F-NODE".to_string()));
        }
        Commands::Stop { name } => {
            println!("Stopping Agent {}", name);
        }
        Commands::Client(client) => {
            let client_cmd = client.command.unwrap_or(ClientCommands::Status);
            let host = client.host.unwrap_or_else(|| "F-NODE".to_string());
            match client_cmd {
                ClientCommands::Status => {
                    println!("Agent {} is online at {} ", client.name, host);
                }
                ClientCommands::Chat { message } => {
                    println!("Agent {} said: {}", client.name, message);
                }
            }
        }
    }
}

// fn main() {
//     // let agent_service = chat::chat::AgentService::new("AgentOne".to_string(), None);
//     // agent_service.start();
//     // agent_service.chat("Hello, world!".to_string());
//     // agent_service.chat("Hello, world!".to_string());
//     // agent_service.chat("Hello, world!".to_string());
//     // agent_service.chat("Hello, world!".to_string());
//     // agent_service.chat("Hello, world!".to_string());
//     // agent_service.stop();
// }