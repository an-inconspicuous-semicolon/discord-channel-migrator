use super::IdentifierType;
use crate::identification::get_id_name;
use crate::identification::get_id_type;
use serenity::all::{ChannelType, Http, Token};
use std::str::FromStr;

fn get_token() -> Token {
    // Requires to be run on a bot that is on the isc dev server (e.g. it must be run by me)
    Token::from_str(&std::env::var("BOT_TOKEN").expect("No bot token was given for the tests"))
        .expect("An invalid bot token was given to the tests")
}

#[tokio::test]
async fn test_text_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417995589529899008; // dcm-text channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(channel_type, IdentifierType::Channel(ChannelType::Text)),
        "The text channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_voice_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417995650846687393; // dcm-voice channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(channel_type, IdentifierType::Channel(ChannelType::Voice)),
        "The voice channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_news_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417996239609270476; // dcm-news channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(channel_type, IdentifierType::Channel(ChannelType::News)),
        "The news channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_news_thread_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417996269158273095; // dcm-news-thread in the dcm-news channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(
            channel_type,
            IdentifierType::Channel(ChannelType::NewsThread)
        ),
        "The news thread channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_private_thread_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417996053365391512; // dcm-private-thread in the dcm-text channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(
            channel_type,
            IdentifierType::Channel(ChannelType::PrivateThread)
        ),
        "The private thread channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_public_thread_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417995966052696254; // dcm-public-thread in the dcm-forum channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(
            channel_type,
            IdentifierType::Channel(ChannelType::PublicThread)
        ),
        "The public thread channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_stage_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417997115296321678; // dcm-stage channel channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(channel_type, IdentifierType::Channel(ChannelType::Stage)),
        "The stage channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_forum_channel_identification() {
    let token = get_token();
    let http = Http::new(token);
    let channel_id = 1417995706995703870; // dcm-forum channel in the isc dev server.
    let channel_type = get_id_type(&http, channel_id.into()).await.unwrap();
    assert!(
        matches!(channel_type, IdentifierType::Channel(ChannelType::Forum)),
        "The forum channel was incorrectly identified as a {}!",
        get_id_name(channel_type)
    );
}

#[tokio::test]
async fn test_user_identification() {
    let token = get_token();
    let http = Http::new(token);
    let id = 487960126201004042; // An Inconspicuous Semicolon.
    let id_type = get_id_type(&http, id.into()).await.unwrap();
    assert!(
        matches!(id_type, IdentifierType::User),
        "The user \"An Inconspicuous Semicolon\" was incorrectly identified as a {}!",
        get_id_name(id_type)
    );
}

#[tokio::test]
async fn test_guild_identification() {
    let token = get_token();
    let http = Http::new(token);
    let id = 1353806081058537603; // ISC's Testing & Dev Server.
    let id_type = get_id_type(&http, id.into()).await.unwrap();
    assert!(
        matches!(id_type, IdentifierType::Guild),
        "The guild \"ISC's Testing & Dev Server\" was incorrectly identified as a {}!",
        get_id_name(id_type)
    );
}
