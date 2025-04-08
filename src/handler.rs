use std::{env, sync::LazyLock};
use serenity::{all::{Context, CreateMessage, EventHandler, GuildChannel, Message}, async_trait};
use crate::actions::{messages::{error::GlobalMessageError, misc::immortal::MessageImmortal, threads::{help_on_create::ThreadMessageHelpOnCreate, logs::LogAnalyzerError}, IntoCreateMessage}, threads};

static CHANNEL_ID_HELP: LazyLock<u64> = LazyLock::new(|| env::var("CHANNEL_ID_HELP").unwrap().parse().unwrap());

pub struct MessageHandler;

impl MessageHandler {
  async fn send_message<T: IntoCreateMessage>(&self, ctx: Context, channel: GuildChannel) {
    let message = T::create();

    let result = channel.send_message(&ctx, message)
      .await;

    if let Err(_) = result {
      #[allow(unused)]
      channel.send_message(ctx, GlobalMessageError::create())
        .await;
    }
  }
}

#[async_trait]
impl EventHandler for MessageHandler {
  // handles the message creation
  // todo переписать этот пи.пец
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.author.bot {
      return;
    }

    if msg.content == ">bestlyricsinthaworld" {
      msg.channel_id.send_message(ctx, MessageImmortal::create())
        .await;

      return
    }

    let channel = msg.channel(&ctx)
      .await;

    if let Ok(channel) = channel {
      let parent = channel.guild().unwrap().parent_id;

      if let Some(parent) = parent {
        match parent {
          id if id == *CHANNEL_ID_HELP => {
            if msg.attachments.len() != 1 {
              return
            }

            let bind = msg.clone();
            // safe unwrap, source: trust me
            let item = bind.attachments.get(0)
              .unwrap();

            if !item.filename.ends_with(".rflog") || item.size == 0 {
              return
            }

            let channel = msg.channel_id;

            if let Err(err) = threads::logs::start_analyze(&ctx, msg, channel, item).await {
              #[allow(unused)]
              channel.send_message(&ctx, LogAnalyzerError::create(err.to_string()))
              .await;
            }
          },
          _ => return
        }
      } else {
        msg.channel_id.send_message(ctx, CreateMessage::new().content("не сегодня")).await;
      }
    } else {
      msg.channel_id.send_message(ctx, CreateMessage::new().content("не сегодня")).await;
    }

    // match parent {
    //   id if id == *CHANNEL_ID_HELP => {
    //     // println!("{msg:?}");
    //     if msg.attachments.len() != 1 {
    //       return
    //     };

    //     println!("attaches: {:?}", msg.attachments)
    //   },
    //   _ => return
    // }
  }

  // todo it doubles
  async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
    println!("Created");
    // posted in this channel
    let id = thread.parent_id.unwrap().get();

    match id {
      // channel with help
      id if id == *CHANNEL_ID_HELP => {
        self.send_message::<ThreadMessageHelpOnCreate>(ctx, thread)
          .await;
      },
      _ => return
    }
  }
}