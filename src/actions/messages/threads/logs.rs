use rflog_parser::info_line::RFLogInfoLine;
use serenity::all::{Color, CreateEmbed, CreateMessage};
use crate::actions::{messages::IntoCreateMessage, threads::logs::SortedLogs};

pub struct LogStartAnalyzing;

impl IntoCreateMessage for LogStartAnalyzing {
  fn create() -> CreateMessage {
    let embed = CreateEmbed::new()
      .title("<a:ranifiesta:1358873672953106735> Анализ лога")
      .color(Color::from_rgb(255, 153, 0))
      .description("Я начал анализировать ваши логи.\nЭто может занять какое-то время.\n\nКогда лог будет проанализирован, я упомяну вас в этом чате.");

    CreateMessage::new()
      .add_embed(embed)
  }
}

pub struct LogAnalyzerError;

impl LogAnalyzerError {
  pub fn create(err: String) -> CreateMessage {
    let embed = CreateEmbed::new()
      .title("⚠️ Ошибка")
      .color(Color::from_rgb(220, 0, 0))
      .description(format!("Ошибка при обработке лога: {err}"));

    CreateMessage::new()
      .add_embed(embed)
  }
}

pub struct LogAnalyzerInfoLine;

impl LogAnalyzerInfoLine {
  pub fn create(line: &RFLogInfoLine) -> CreateMessage {
    let embed = CreateEmbed::new()
      .title("Информация")
      .color(Color::from_rgb(0, 200, 255))
      .description(format!(
        "Лог был успешно распаршен.
        \n**``RFLogInfoLine``**:
        ``Версия лаунчера`` **{}**
        ``Игрок`` **{}**
        ``Клиент`` **{}**
        ``Операционная система``: **{}** ({})
        ", line.launcher_version.to_string(), line.player_nick, line.game_client, line.os, line.os_version));

    CreateMessage::new()
      .add_embed(embed)
  }
}

pub struct LogAnalyzerLogInformation;

impl LogAnalyzerLogInformation {
  pub fn create(logs: &SortedLogs) -> CreateMessage {
    let errs_len = logs.err.len();
    let outs_len = logs.out.len();

    let embed = CreateEmbed::new()
      .title("Получено из логов")
      .color(Color::from_rgb(82, 50, 168))
      .description(format!(
        "``Всего строк`` **{}**
        ``Количество нормальных логов`` **{}**
        ``Количество ошибок`` **{}**
        \n*Мы не смогли найти ошибки/решение ошибок в этом лог файле*
        ", errs_len + outs_len, outs_len, errs_len));

    CreateMessage::new()
      .add_embed(embed)
  }
}

pub struct LogAnalyzerErrors;

impl LogAnalyzerErrors {
  pub fn create(logs: &SortedLogs) -> CreateMessage {
    let count = logs.err.len().min(5);

    let mut content = String::new();

    for err in &logs.err[logs.err.len() - count..] {
      let mut prefix = String::new();

      if err.executor.len() != 0 {
        prefix += &format!("[{}]", err.executor);
        if err.thread.len() != 0 {
          prefix += &format!(" {}", err.thread);
        }
      }

      content += &format!("\n``L{}``\n```log\n{}", err.log_line, if prefix.len() != 0 {": "} else {""});

      for (i, log) in err.data.iter().enumerate() {
        if i == 0 {
          content += &format!("{}", log);

          continue;
        }

        content += &format!("\n\t{}", log);
      }

      content += "```";
    };

    let embed = CreateEmbed::new()
      .title(&format!("❗ Последние {count} ошибок"))
      .color(Color::from_rgb(82, 50, 168))
      .description(content);

    CreateMessage::new()
      .add_embed(embed)
  }
}