use serenity::all::{Channel, ChannelType};

#[derive(Copy, Clone)]
pub enum IdentifierType {
    User,
    Guild,
    Channel(ChannelType),
}

pub(crate) fn get_channel_inner_type(channel: Channel) -> ChannelType {
    let channel_type = match channel {
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
    };

    // The channel can be "valid" yet unknown, so handle that as well.
    if channel_type.name().eq_ignore_ascii_case("unknown") {
        panic!("Encountered a channel type that should not exist!")
    }

    channel_type
}
