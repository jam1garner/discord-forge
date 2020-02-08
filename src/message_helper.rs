use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::http::AttachmentType;

pub struct MessageHelper {
    message: Message,
    context: Context,
}

impl MessageHelper {
    pub fn new(message: Message, context: Context) -> Self {
        MessageHelper {
            message, context
        }
    }

    pub fn say<S: AsRef<str>>(&self, message: S) -> Message {
        self.message.channel_id.say(&self.context.http, message.as_ref()).unwrap()
    }

    pub fn reply<S: AsRef<str>>(&self, message: S) -> Message {
        self.message.reply(self.context.http.clone(), message.as_ref()).unwrap()
    }

    pub fn broadcast_typing(&self) {
        self.message.channel_id.broadcast_typing(&self.context.http).unwrap()
    }

    pub fn send_file<'a, T, S: AsRef<str>>(&self, file: T, message: S) -> Result<Message, serenity::Error>
        where T: Into<AttachmentType<'a>>, 
    {
        self.send_files(vec![file], message)
    }

    pub fn send_files<'a, T, It, S>(&self, files: It, message: S) -> Result<Message, serenity::Error>
        where T: Into<AttachmentType<'a>>, It: IntoIterator<Item = T>, S: AsRef<str>,
    {
        self.message.channel_id.send_files(
            &self.context.http,
            files,
            |m| m.content(message.as_ref())
        )
    }

    pub fn get_current_application_info(&self) -> CurrentApplicationInfo {
        self.context.http.get_current_application_info().unwrap()
    }

    pub fn member_permissions(&self) -> Permissions {
        self.message.member
            .as_ref()
            .unwrap()
            .roles
            .iter()
            .fold(
                Permissions::empty(),
                |perms, role| perms | role.to_role_cached(&self.context.cache).unwrap().permissions
            )
    }
}

impl std::ops::Deref for MessageHelper {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}
