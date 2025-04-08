use anyhow::Result;
use rflog_parser::log::Log;
use serenity::all::{Attachment, ChannelId, Context, Message};
use crate::{actions::messages::{threads::logs::{LogAnalyzerErrors, LogAnalyzerInfoLine, LogAnalyzerLogInformation, LogStartAnalyzing}, user::UserPing, IntoCreateMessage}, utils::http::get_body};

#[derive(Default, Debug)]
pub struct SortedLogs<'a> {
  pub out: Vec<&'a Log>,
  pub err: Vec<&'a Log>
}

#[allow(unused)]
pub async fn start_analyze(ctx: &Context, message: Message, channel: ChannelId, item: &Attachment) -> Result<()> {
  channel.send_message(ctx, LogStartAnalyzing::create())
    .await?;

  let body = get_body(&item.url)
    .await?;

  let log = rflog_parser::parse(body)?;

  channel.send_message(ctx, LogAnalyzerInfoLine::create(&log.info_line))
    .await;

  let mut sorted = SortedLogs::default();

  log.logs.iter().for_each(|log| {
    match log.kind {
      rflog_parser::log::LogKind::Out => sorted.out.push(log),
      rflog_parser::log::LogKind::Err => sorted.err.push(log),
    }
  });

  channel.send_message(ctx, LogAnalyzerLogInformation::create(&sorted))
    .await;

  if sorted.err.len() != 0 {
    channel.send_message(ctx, LogAnalyzerErrors::create(&sorted))
      .await;
  }

  channel.send_message(ctx, UserPing::create(message.author.id, "не получилось найти ошибки/их решение"))
    .await;

  Ok(())
}