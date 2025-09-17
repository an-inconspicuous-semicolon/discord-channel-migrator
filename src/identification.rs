use crate::channel;
use crate::channel::IdentifierType;
use anyhow::anyhow;
use serenity::all::{GenericId, Http, HttpError};

pub fn get_id_name(destination_type: IdentifierType) -> String {
    match destination_type {
        IdentifierType::User => "user".to_string(),
        IdentifierType::Guild => "guild".to_string(),
        IdentifierType::Channel(channel_type) => format!("{} channel", channel_type.name()),
    }
}

pub async fn get_id_type(http: &Http, id: GenericId) -> Result<IdentifierType, anyhow::Error> {
    // First, attempt to see if it is a user that we are aware of
    if let Err(error) = http.get_user(id.get().into()).await {
        if !is_unknown_entity(&error) {
            // This was not an unknown entity, so we encountered an actual error!
            return Err(error.into());
        };
    } else {
        return Ok(IdentifierType::User);
    }

    // Second, attempt to see if it is a guild that we are aware of
    if let Err(error) = http.get_guild(id.get().into()).await {
        if !is_unknown_entity(&error) {
            return Err(error.into());
        };
    } else {
        return Ok(IdentifierType::Guild);
    }

    // Finally, check if this is a channel that we are aware of. otherwise return an error since we don't know the type
    http.get_channel(id.get().into())
        .await
        .map(channel::get_channel_inner_type)
        .map(IdentifierType::Channel)
        .map_err(|_| anyhow!("Unknown ID type! {id}"))
}

fn is_unknown_entity(error: &serenity::Error) -> bool {
    matches!(&error,
            serenity::all::Error::Http(HttpError::UnsuccessfulRequest(e))
            if (10000u32..20000).contains(&e.error.code.0))
}
