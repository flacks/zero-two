use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use serenity::utils::parse_channel;

use crate::checks::*;

#[command]
#[checks(Admin)]
#[usage = "[<Channel> |] [<Title> |] text to embed"]
#[description = "Create an embed with a title and content is the current channel or a different channel"]
fn embed(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let full_message: String = args.message().to_string();

    let split_pattern = " | ";

    let mut segments: Vec<&str> = full_message
        .split(split_pattern)
        .filter(|seg| !seg.is_empty())
        .collect();

    if segments.is_empty() {
        return Err(CommandError::from(
            "_Echo echo. Test 1 2 3, test A B C ..._",
        ));
    }

    let channel = match parse_channel(segments[0]) {
        Some(cid) => {
            segments = segments[1..].to_vec();

            ChannelId(cid)
        }
        None => message.channel_id,
    };

    let _ = channel.send_message(&context.http, |m| {
        m.embed(|embed| {
            if segments.len() >= 2 {
                embed.title(segments[0]);
                embed.description(segments[1..].join(split_pattern));
            } else {
                embed.description(segments.join(split_pattern));
            }

            embed
        })
    });

    Ok(())
}
