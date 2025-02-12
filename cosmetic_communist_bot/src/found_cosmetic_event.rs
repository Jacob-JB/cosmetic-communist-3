use std::sync::Arc;

use log::info;
use serde::Deserialize;
use serenity::{all::*, futures::future::select_all};
use surrealdb::{RecordId, Surreal};

use crate::parse_numerical_record_key;

/// a task that manages a found cosmetic event
pub async fn found_cosmetic_event(
    http: Arc<Http>,
    shard: ShardMessenger,
    db: Surreal<surrealdb::engine::remote::ws::Client>,
    user_record: RecordId,
    cosmetic_record: RecordId,
) {
    let user_discord_id = parse_numerical_record_key(&user_record);

    let user_id = UserId::new(user_discord_id.parse().expect("Failed to parse user id"));

    #[derive(Debug, Deserialize)]
    struct GuildConfig {
        #[serde(rename = "guildId")]
        guild: String,
        #[serde(rename = "pingChannelId")]
        ping_channel: String,
    }

    let mut response = db
        .query(format!(
            "SELECT guildId, pingChannelId FROM guildConfig; SELECT VALUE <-needs<-user.id FROM {}; SELECT VALUE name from {};",
            cosmetic_record,
            cosmetic_record,
        ))
        .await
        .unwrap();

    let channels: Vec<GuildConfig> = response.take(0).unwrap();
    let users: Vec<Vec<RecordId>> = response.take(1).unwrap();
    let cosmetic_name: Vec<String> = response.take(2).unwrap();
    let cosmetic_name = cosmetic_name.into_iter().next().unwrap();

    // In each guild the user is in,
    // send a notification message that the user has
    // found a cosmetic. Ping all players that need that cosmetic.

    let mut button_awaits = Vec::new();

    let mut message_content = format!("<@{}> found a **{}**\n", user_discord_id, cosmetic_name);

    for ping_user_record_id in users.get(0).unwrap() {
        let discord_id = parse_numerical_record_key(&ping_user_record_id);
        message_content += &format!("\n<@{}>", discord_id);
    }

    for GuildConfig {
        guild,
        ping_channel,
    } in channels
    {
        let guild_id = GuildId::new(guild.parse().expect("Failed to parse guild id"));

        // only ping channel if the user that generated the event is in the guild
        let Ok(_) = guild_id.member(&http, user_id).await else {
            continue;
        };

        let ping_channel_id = ChannelId::new(
            ping_channel
                .parse()
                .expect("Failed to parse ping channel id"),
        );
        let Ok(message) = ping_channel_id
            .send_message(
                &http,
                CreateMessage::new()
                    .content(&message_content)
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("claim")
                            .label("Claim")
                            .style(ButtonStyle::Success),
                    ])]),
            )
            .await
        else {
            continue;
        };

        async fn await_button(
            shard: ShardMessenger,
            message: Message,
        ) -> Option<ComponentInteraction> {
            message.await_component_interaction(&shard).await
        }

        button_awaits.push(Box::pin(await_button(shard.clone(), message)));
    }

    if button_awaits.is_empty() {
        return;
    }

    let (Some(interaction), _, _) = select_all(button_awaits.into_iter()).await else {
        info!("no interaction");
        return;
    };

    interaction
        .create_response(
            &http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().components(Vec::new()),
            ),
        )
        .await
        .unwrap();

    interaction
        .message
        .reply(
            &http,
            format!(
                "<@{}> claimed your **{}** <@{}>",
                interaction.user.id, cosmetic_name, user_discord_id
            ),
        )
        .await
        .unwrap();
}
