use crate::threads::ThreadReader;
use clap::Parser;
use futures::StreamExt;
use serenity::all::{ChannelId, GuildThread, Http, Token};

mod messages;
mod threads;

#[derive(clap::Parser)]
struct Args {
    #[clap(
        long,
        help = "The token for the discord bot that will perform the transfer."
    )]
    token: Token,
    #[clap(long, help = "The channels that contain the threads to transfer.")]
    source: ChannelId,
    #[clap(long, help = "The channel that the threads will be transferred to.")]
    destination: ChannelId,
}

#[tokio::main]
async fn main() {
    println!("Hello, World!");
    let args = Args::parse();

    let http = Http::new(args.token.clone());
    println!(
        "Will move threads from {} to {}.",
        args.source, args.destination
    );

    let threads = match enumerate_channel_threads(&http, args.source).await {
        Ok(threads) => threads,
        Err(error) => {
            eprintln!("Failed to enumerate the threads from the source: {error}");
            std::process::exit(1);
        }
    };

    let readers = threads
        .into_iter()
        .map(ThreadReader::new)
        .collect::<Vec<_>>();

    futures::stream::iter(readers.into_iter())
        .then(async |reader| reader.forward_messages(&http, args.destination).await)
        .collect::<Vec<_>>()
        .await;
}

pub async fn enumerate_channel_threads(
    http: &Http,
    channel_id: ChannelId,
) -> Result<Vec<GuildThread>, anyhow::Error> {
    let mut threads = vec![];
    let mut timestamp = None;

    println!("Searching for threads in {}", channel_id);
    loop {
        let data = match http
            .get_channel_archived_public_threads(channel_id, timestamp, None)
            .await
        {
            Ok(data) => data,
            Err(error) => return Err(error.into()),
        };

        if data.threads.is_empty() {
            break;
        }
        println!("Received {} threads.", data.threads.len());

        let oldest = data.threads.last().unwrap();
        timestamp = Some(oldest.thread_metadata.archive_timestamp.unwrap());
        threads.extend(data.threads);
    }
    println!("Successfully found {} threads.", threads.len());

    threads.reverse();
    Ok(threads)
}
