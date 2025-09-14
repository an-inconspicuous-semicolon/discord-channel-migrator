use serenity::all::{Attachment, CreateMessage, Message, MessageId, User};

#[derive(Clone)]
pub struct ForwardedMessage {
    pub _original_id: Option<MessageId>,
    pub _author: User,
    pub content: String,
    pub attachments: Vec<Attachment>,
    pub _reference: Option<MessageId>,
}

impl ForwardedMessage {
    pub fn sanitise_content(content: String) -> String {
        if content.is_empty() {
            "-# _[empty message]_".into()
        } else {
            content
        }
    }
}

impl From<Message> for ForwardedMessage {
    fn from(value: Message) -> Self {
        Self {
            _original_id: Some(value.id),
            _author: value.author,
            content: Self::sanitise_content(value.content.into()),
            attachments: value.attachments.into(),
            _reference: value.referenced_message.map(|msg| msg.id),
        }
    }
}

impl<'a> From<ForwardedMessage> for CreateMessage<'a> {
    fn from(value: ForwardedMessage) -> CreateMessage<'a> {
        CreateMessage::new().content(value.content)
    }
}
