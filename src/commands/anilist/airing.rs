use chrono::{DateTime, Local, Weekday};
use std::ops::Add;
use time::Duration;

use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::anilist::client;
use crate::core::utils::{next_day, weekday_to_string};
use crate::models::anilist::airing_schedule::AiringSchedule;

#[command]
#[aliases("air", "airs")]
#[usage = "[weekday]"]
#[description = "Show airing anime for a given/current day"]
fn airing(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let to_midnight = |datetime: DateTime<Local>| datetime.date().and_hms(0, 0, 0);

    let (start, day) = match args.message().parse::<Weekday>() {
        Ok(day) => (to_midnight(next_day(day)), weekday_to_string(day)),
        Err(_) => (to_midnight(Local::now()), "Today".to_owned()),
    };

    let results: Vec<AiringSchedule> =
        client::search_airing_schedule(start.timestamp(), start.add(Duration::days(1)).timestamp());

    if results.is_empty() {
        return Err(CommandError::from("Error checking the airing schedule"));
    }

    let airing_shows = results
        .iter()
        .map(|item| item.to_url())
        .collect::<Vec<String>>()
        .join("\n");

    let _ = message.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.color(3447003)
                .title(format!("Airing Schedule for {}", day))
                .description(airing_shows)
                .footer(|f| {
                    f.icon_url("https://anilist.co/img/icons/favicon-32x32.png")
                        .text("Powered by AniList")
                })
        })
    });

    Ok(())
}
