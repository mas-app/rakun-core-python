extern crate core;

mod rakun;

use clap::{Args, Parser, Subcommand};
use names::Generator;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "rakun")] // requires `clap` feature
#[clap(about = "A Multi Agent System CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    host: Option<String>,
    #[clap(long, short = 'c')]
    config: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start Agent
    Start,
    /// Stop Agent
    Stop,
    /// Pause Agent
    Pause,
    /// Get Agent Status
    Status,
    /// Chat with Agent
    Command {
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
    let name = match args.name {
        Some(name) => name,
        None => {
            let mut generator = Generator::default();
            generator.next().unwrap()
        },
    };


    let host = args.host.unwrap_or("localhost".to_string());
    let config = args.config.unwrap_or("config.toml".to_string());


    let rakun_service = rakun::service::RakunService::new(name, host, config, 8080);

    match args.command {
        Commands::Start => {
            rakun_service.start();
        }
        Commands::Stop => {
            rakun_service.stop();
        }
        Commands::Pause => {
            println!("Pause Agent");
        }
        Commands::Status => {
            println!("Get Agent Status");
        }
        Commands::Command { message } => {
            println!("Chat with Agent: {}", message);
        }
    }
}