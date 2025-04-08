use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use crate::actions::messages::IntoCreateMessage;

pub struct MessageImmortal;

// :)
impl IntoCreateMessage for MessageImmortal {
  fn create() -> serenity::all::CreateMessage {
    let embed = CreateEmbed::new()
      .url("https://youtu.be/cjAO7Y5WmwM?t=1927&si=MZOdNYmdq8BA0t7V")
      .title("IMMORTAL")
      .description("This world is my balls, so I let 'em hang
        When I pull this thang, all you bitches best to know my name
        Doper than you hoes, I'm coming from South Park
        Catch you after dark, I'mma split your fuckin' chest apart
        Hollow tips")
      .footer(CreateEmbedFooter::new("JUNIOR FERRARI"));

    CreateMessage::new()
      .add_embed(embed)
  }
}