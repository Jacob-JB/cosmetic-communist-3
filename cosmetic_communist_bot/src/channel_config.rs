use serenity::all::{Channel, ChannelId, ChannelType, GuildChannel};

use crate::{Context, Error};

#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn set_ping_channel(
    ctx: Context<'_>,
    #[description = "Select a specific channel, leave blank for this channel"] ping_channel: Option<
        Channel,
    >,
) -> Result<(), Error> {
    let (channel_id, guild_id) = match ping_channel {
        Some(Channel::Guild(GuildChannel {
            kind: ChannelType::Text,
            id,
            guild_id,
            ..
        })) => (id, guild_id),
        Some(_) => {
            ctx.reply("Only text channels can be ping channels").await?;
            return Ok(());
        }
        None => match ctx.guild_channel().await.unwrap() {
            GuildChannel {
                kind: ChannelType::Text,
                id,
                guild_id,
                ..
            } => (id, guild_id),
            _ => {
                ctx.reply("Only text channels can be ping channels").await?;
                return Ok(());
            }
        },
    };

    ctx.data()
        .db
        .query(format!(
            "DELETE guildConfig WHERE guildId = \"{}\"; CREATE guildConfig CONTENT {{ guildId: \"{}\", pingChannelId: \"{}\" }};",
            guild_id,
            guild_id,
            channel_id,
        ))
        .await?;

    ctx.reply(format!(
        "Set the ping channel for this server to <#{}>",
        channel_id,
    ))
    .await?;

    Ok(())
}

#[poise::command(slash_command, guild_only, ephemeral)]
pub async fn get_ping_channel(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let mut response = ctx
        .data()
        .db
        .query(format!(
            "SELECT VALUE pingChannelId FROM guildConfig WHERE guildId = \"{}\";",
            guild_id
        ))
        .await?;

    let query_values: Vec<String> = response.take(0).unwrap();

    match query_values.get(0) {
        Some(channel_id) => {
            ctx.reply(format!(
                "The ping channel for this server is <#{}>",
                channel_id
            ))
            .await?;
        }
        None => {
            ctx.reply("This server does not have a ping channel configured")
                .await?;
        }
    }

    Ok(())
}

#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn clear_ping_channel(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    ctx.data()
        .db
        .query(format!(
            "DELETE guildConfig WHERE guildId = \"{}\";",
            guild_id,
        ))
        .await?;

    ctx.reply(format!(
        "This server will no longer be pinged for cosmetics",
    ))
    .await?;

    Ok(())
}
