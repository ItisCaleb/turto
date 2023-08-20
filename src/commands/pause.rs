use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
};

use tracing::error;

use crate::{
    guild::playing::Playing,
    messages::NOT_PLAYING,
    utils::guild::GuildUtil,
};

#[command]
#[bucket = "music"]
async fn pause(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(ctx).unwrap();
    let guild_id = guild.id;

    // Check if the bot and the user is in a channel or not
    if let Some(bot_voice_channel) = guild.get_user_voice_channel(&ctx.cache.current_user_id()) {
        if Some(bot_voice_channel) != guild.get_user_voice_channel(&msg.author.id) {
            // Notify th user if they are in different voice channel
            msg.reply(
                ctx,
                format!("You are not in {}.", bot_voice_channel.mention()),
            )
            .await?;
            return Ok(());
        }
    } else {
        msg.reply(ctx, NOT_PLAYING).await?;
        return Ok(());
    }

    let playing_lock = ctx
        .data
        .read()
        .await
        .get::<Playing>()
        .expect("Expected Playing in TypeMap")
        .clone();
    {
        let playing = playing_lock.read().await;
        let current_track = match playing.get(&guild_id) {
            Some(track) => track,
            None => {
                msg.reply(ctx, NOT_PLAYING).await?;
                return Ok(());
            }
        };

        if let Err(why) = current_track.pause() {
            error!("Error pausing a track {}: {:?}", current_track.uuid(), why);
        }

        msg.reply(
            ctx,
            format!("⏸️ {}", current_track.metadata().title.clone().unwrap()),
        )
        .await?;
    }
    Ok(())
}
