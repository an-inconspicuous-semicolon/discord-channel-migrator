use serenity::all::{Channel, ChannelId, ChannelType, GenericId, Http, HttpError};

pub enum DestinationType {
    Guild,
    Channel(ChannelType),
}

pub async fn get_source_type(
    http: &Http,
    channel_id: ChannelId,
) -> Result<ChannelType, serenity::Error> {
    http.get_channel(channel_id.into())
        .await
        .map(get_channel_inner_type)
}

pub async fn get_destination_type(
    http: &Http,
    id: GenericId,
) -> Result<DestinationType, serenity::all::Error> {
    if let Err(error) = http.get_guild(id.get().into()).await {
        if matches!(&error, 
            serenity::all::Error::Http(HttpError::UnsuccessfulRequest(e)) 
            if (10000u32..20000).contains(&e.error.code.0))
        // Unknown entity
        {
            // This id was not for a guild, try as a channel instead.
            return get_source_type(http, id.get().into())
                .await
                .map(DestinationType::Channel);
        }
        return Err(error);
    }
    Ok(DestinationType::Guild)
}

fn get_channel_inner_type(channel: Channel) -> ChannelType {
    match channel {
        Channel::Guild(channel) => channel.base.kind,
        Channel::GuildThread(thread) => thread.base.kind,
        Channel::Private(private) => private.kind,
        // The Channel enum is marked as non exhaustive,
        // so we've matched all the variants that _should_ exist,
        // but since there may be more added in the future that would not break this match,
        // We instead panic as we don't know for certain how to deal with this channel type.
        _ => {
            panic!("Encountered a channel type that should not exist!")
        }
    }
}
