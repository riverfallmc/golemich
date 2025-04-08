use serenity::all::CreateMessage;

pub mod threads;
pub mod error;
pub mod user;
pub mod misc;

pub trait IntoCreateMessage {
  fn create() -> CreateMessage;
}