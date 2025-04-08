use serenity::all::{Color, CreateEmbed, CreateMessage};
use crate::actions::messages::IntoCreateMessage;

pub struct ThreadMessageHelpOnCreate;

impl IntoCreateMessage for ThreadMessageHelpOnCreate {
  fn create() -> CreateMessage {
    let embed = CreateEmbed::new()
      .title("Логи")
      .color(Color::from_rgb(0, 200, 255))
      .description("Если у вас возникли проблемы
      с игрой/её запуском, то пожалуйста, предоставьте
      лог из папки ``<папка клиента>/logs/latest.rflog``.
      <a:steve:1358873681744363571>
      Так я смогу сказать что именно вызывает ошибку.");

    CreateMessage::new()
      .add_embed(embed)
  }
}