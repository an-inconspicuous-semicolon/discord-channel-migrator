mod channel;

mod identification;
#[cfg(test)]
mod tests;

use crate::channel::IdentifierType;
use crate::identification::get_id_name;
use anyhow::anyhow;
use clap::Parser;
use identification::get_id_type;
use serenity::all::{ChannelType, GenericId, Http, Token};

#[derive(clap::Parser)]
struct Args {
    #[clap(
        long,
        help = "The token for the discord bot that will perform the transfer."
    )]
    token: Token,
    #[clap(long, help = "The channel that will be transferred.")]
    source: GenericId,
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

    let source_type = get_id_type(&http, args.source)
        .await
        .unwrap_or_else(terminate_on_error);

    let destination_type = get_id_type(&http, args.destination)
        .await
        .unwrap_or_else(terminate_on_error);

    let source_type_name = match source_type {
        IdentifierType::User => terminate_on_error(anyhow!(
            "The source ID appears to be a user. Transferring to or from user DM's are not currently supported!"
        )),
        other => identification::get_id_name(other),
    };

    let destination_type_name = match destination_type {
        IdentifierType::User => terminate_on_error(anyhow!(
            "The source ID appears to be a user. Transferring to or from user DM's are not currently supported!"
        )),
        other => identification::get_id_name(other),
    };

    println!("source channel type: {}", source_type_name);
    println!("destination channel type: {}", destination_type_name);

    validate_source_destination_pairing(source_type, destination_type)
        .unwrap_or_else(terminate_on_error);
}

// We never actually return a T since we always exit,
// but this makes it possible to do `unwrap_or_else(terminate_on_serenity_error)`
// instead of `unwrap_or_else(|error| terminate_on_serenity_error(error))`
fn terminate_on_error<T>(error: anyhow::Error) -> T {
    eprintln!("Received an error whilst doing a critical operation: {error}");
    std::process::exit(1);
}

#[rustfmt::skip] // rustfmt makes the matches call look really ugly :D
fn validate_source_destination_pairing(
    source_type: IdentifierType,
    destination_type: IdentifierType,
) -> Result<(), anyhow::Error>{
    if matches!(
        (&source_type, &destination_type),
        | (IdentifierType::Guild, _)
        | (_, IdentifierType::User)
        | (IdentifierType::Channel(ChannelType::Category), _)
        | (_, IdentifierType::Channel(ChannelType::Category))
        | (IdentifierType::Channel(ChannelType::Directory), _)
        | (_, IdentifierType::Channel(ChannelType::Directory))
        | (IdentifierType::Channel(ChannelType::Forum),
           IdentifierType::Channel(ChannelType::GroupDm))
        | (IdentifierType::Channel(ChannelType::Forum),
           IdentifierType::Channel(ChannelType::NewsThread))
        | (IdentifierType::Channel(ChannelType::Forum),
           IdentifierType::Channel(ChannelType::PublicThread))
        | (IdentifierType::Channel(ChannelType::Forum),
           IdentifierType::Channel(ChannelType::PrivateThread))
    ) {
        return Err(anyhow!(
            "The source and destination types are incompatible! ({} -> {})",
            get_id_name(source_type),
            get_id_name(destination_type)
        ))
    }
    Ok(())
}
