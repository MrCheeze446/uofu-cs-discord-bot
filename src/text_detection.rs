use std::io::Read;

use crate::{config::ReactRole, data::AppState};
use color_eyre::eyre::{Context, OptionExt, Result};
use poise::serenity_prelude::{self as serenity};
use serenity::Message;
pub async fn text_detection(
    ctx: &serenity::Context,
    data: &AppState,
    message: &Message,
) -> Result<()> {
    if message.is_own(ctx) {
        return Ok(());
    }

    let author_id: u64 = message.author.id.into();

    let author_has_role = data
        .config
        .read()
        .await
        .bot_react_role_members
        .iter()
        .find(|member| matches!(member, ReactRole { user_id, .. } if *user_id == author_id))
        .map(|member| member.react);

    if let Some(false) = author_has_role {
        return Ok(());
    }

    let author_has_role = message
        .author
        .has_role(
            ctx,
            message.guild_id.ok_or_eyre("should have guild id")?,
            data.config.read().await.bot_react_role_id,
        )
        .await
        .context("Couldn't get roles")?;

    data.config
        .write()
        .await
        .bot_react_role_members
        .push(ReactRole {
            user_id: author_id,
            react: author_has_role,
        });

    if let Some(message_response) = data.find_response(&message.content, &message.link()).await {
        data.run_action(&message_response, message, ctx).await?;
    }

    Ok(())
}
