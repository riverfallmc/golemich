use serenity::all::{CreateMessage, UserId};

pub struct UserPing;

impl UserPing {
  pub fn create(user: UserId, content: &str) -> CreateMessage {
    CreateMessage::new()
      .content(&format!("<@{}>, {content}", user.get()))
  }
}