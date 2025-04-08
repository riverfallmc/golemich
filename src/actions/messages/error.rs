use serenity::all::{Color, CreateEmbed, CreateMessage};
use super::IntoCreateMessage;

pub struct GlobalMessageError;

impl IntoCreateMessage for GlobalMessageError {
  fn create() -> CreateMessage {
    CreateMessage::new()
      .add_embed(
        CreateEmbed::new()
          .title("⚠️ Ошибка")
          .color(Color::from_rgb(220, 0, 0))
          .description("При выполнении части кода произошла ошибка.\nВаш запрос не был выполнен корректно.")
      )
  }
}