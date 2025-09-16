mod channel;
mod transferrer;

use crate::channel::{DestinationType, get_destination_type, get_source_type};
use clap::Parser;
use serenity::all::{ChannelId, GenericId, Http, Token};

#[derive(clap::Parser)]
struct Args {
    #[clap(
        long,
        help = "The token for the discord bot that will perform the transfer."
    )]
    token: Token,
    #[clap(long, help = "The channel that will be transferred.")]
    source: ChannelId,
    #[clap(
        long,
        help = "The channel/guild that the channel will be transferred to."
    )]
    destination: GenericId,
}

#[tokio::main]
async fn main() {
    println!("Hello, World!");
    let args = Args::parse();
    let http = Http::new(args.token);

    let source_type = get_source_type(&http, args.source)
        .await
        .unwrap_or_else(|error| terminate_on_serenity_error(error));

    let destination_type = get_destination_type(&http, args.destination.get().into())
        .await
        .unwrap_or_else(|error| terminate_on_serenity_error(error));

    let destination_type_name = match destination_type {
        DestinationType::Guild => "guild",
        DestinationType::Channel(channel_type) => channel_type.name(),
    };

    println!("source channel type: {}", source_type.name());
    println!("destination channel type: {}", destination_type_name);
}

fn terminate_on_serenity_error(error: serenity::Error) -> ! {
    eprintln!("Received an error whilst doing a critical operation: {error}");
    std::process::exit(1);
}
