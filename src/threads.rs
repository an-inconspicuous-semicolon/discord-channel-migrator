use crate::messages::ForwardedMessage;
use futures::StreamExt;
use regex::Regex;
use serenity::all::{
    ChannelId, CreateAttachment, CreateForumPost, CreateMessage, GuildThread, Http, Message,
    ThreadId, UserId,
};

pub struct ThreadReader {
    thread: GuildThread,
}

impl ThreadReader {
    pub fn new(thread: GuildThread) -> Self {
        Self { thread }
    }

    pub async fn forward_messages(&self, http: &Http, new_channel: ChannelId) {
        let messages = self.get_messages(http).await;
        if messages.is_empty() {
            eprintln!("The thread {} had no messages!", self.thread.base.name);
            return;
        }

        let starting_message = messages.first().unwrap();
        let starting_attachments = Self::create_attachments(http, starting_message).await;
        let starting_create: CreateMessage = starting_message.clone().into();

        let new_thread = match http
            .create_forum_post(
                new_channel,
                &CreateForumPost::new(self.thread.base.name.clone(), starting_create),
                starting_attachments,
                Some("Transferring threads"),
            )
            .await
        {
            Ok(thread) => thread,
            Err(error) => {
                eprintln!("Failed to create a new thread! {error}");
                return;
            }
        };

        println!("Successfully create a new thread {}", new_thread.id);

        futures::stream::iter(messages.into_iter().skip(1))
            .then(|msg| Self::forward_message(http, msg, new_thread.id))
            .collect::<Vec<_>>()
            .await;
    }

    pub async fn get_messages(&self, http: &Http) -> Vec<ForwardedMessage> {
        let messages: Vec<_> = match http.get_messages(self.thread.id.into(), None, None).await {
            Ok(messages) => messages.into_iter().rev().collect(),
            Err(error) => {
                eprintln!(
                    "Failed to fetch the messages from the channel {}: {error}",
                    self.thread.id
                );
                return vec![];
            }
        };

        messages
            .into_iter()
            .scan(None, Self::show_authors)
            .flatten()
            .map(Self::filter_message)
            .collect()
    }
}

impl ThreadReader {
    fn filter_message(mut message: ForwardedMessage) -> ForwardedMessage {
        let re = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap();

        let input = message.content;
        let mut out = String::with_capacity(input.len());
        let mut last_end = 0;

        for m in re.find_iter(&input) {
            let start = m.start();
            let end = m.end();

            // If the byte immediately before the match is '<', skip replacement.
            let preceded_by_lt = start > 0 && input.as_bytes()[start - 1] == b'<';

            if preceded_by_lt {
                // copy the intervening text and the matched email unchanged
                out.push_str(&input[last_end..end]);
            } else {
                // copy text before match, then redact
                out.push_str(&input[last_end..start]);
                out.push_str("[email]");
            }

            last_end = end;
        }

        out.push_str(&input[last_end..]);
        message.content = out;
        message
    }

    fn show_authors(
        current_author: &mut Option<UserId>,
        message: Message,
    ) -> Option<Vec<ForwardedMessage>> {
        let mut messages = vec![];

        if current_author.is_none_or(|current| current != message.author.id) {
            let display_name = format!(
                "___**{}**___",
                message
                    .author
                    .display_name()
                    .replace("_", "\\_")
                    .replace("*", "\\*")
            );

            messages.push(ForwardedMessage {
                _original_id: None,
                _author: message.author.clone(),
                content: display_name,
                attachments: vec![],
                _reference: None,
            })
        }
        *current_author = Some(message.author.id);
        messages.push(ForwardedMessage::from(message));
        Some(messages)
    }

    async fn create_attachments<'a>(
        http: &Http,
        msg: &ForwardedMessage,
    ) -> Vec<CreateAttachment<'a>> {
        futures::stream::iter(&msg.attachments)
            .then(async |attachment| {
                CreateAttachment::url(
                    http,
                    attachment.url.to_string(),
                    attachment.filename.clone(),
                )
                .await
            })
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flat_map(|result| match result {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("Failed to create an attachment! {e}");
                    None
                }
            })
            .collect()
    }

    async fn forward_message(http: &Http, message: ForwardedMessage, channel_id: ThreadId) {
        let attachments = Self::create_attachments(http, &message).await;
        let create_msg: CreateMessage = message.clone().into();
        http.send_message(channel_id.into(), attachments, &create_msg)
            .await
            .expect("Failed to send a thread message!");
    }
}
