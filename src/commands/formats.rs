use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::cache::{GodboltCache, ConfigCache};
use serenity::builder::CreateEmbed;

use crate::utls::constants::{ICON_HELP, COLOR_OKAY};
use crate::utls::discordhelpers::embeds;

#[command]
pub async fn formats(ctx: &Context, msg: &Message, _args : Args) -> CommandResult {
    let data = ctx.data.read().await;
    let gbolt = data.get::<GodboltCache>().unwrap().read().await;
    let prefix = {
        let botinfo_lock = data
            .get::<ConfigCache>()
            .expect("Expected BotInfo in global cache")
            .clone();
        let botinfo = botinfo_lock.read().await;
        botinfo.get("BOT_PREFIX").unwrap().clone()
    };

    let mut emb = CreateEmbed::default();
    emb.thumbnail(ICON_HELP);
    emb.color(COLOR_OKAY);
    emb.title("Formatters:");
    emb.description(format!("Below is the list of all formatters currently supported, an valid example request can be `{}format rust`, or `{}format clang mozilla`", prefix, prefix));
    for format in &gbolt.formats {
        let mut output = String::new();
        output.push_str("Styles:\n");
        if format.styles.is_empty() {
            output.push_str("    *(None)*\n");
        }
        for style in &format.styles {
            output.push_str(&format!("    *- {}*\n", style));
        }
        emb.field(&format.format_type, &output, false);
    }

    let mut emb_msg = embeds::embed_message(emb);
    msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    return Ok(());
}