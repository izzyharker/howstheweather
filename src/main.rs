use anyhow::Context as _;
use serenity::all::{
    CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage, GuildId,
    Interaction,
};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
mod weather;
use std::thread;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(1276935640801607802);

        // add "/hello" command to the bot
        guild_id
            .set_commands(
                &ctx.http,
                vec![CreateCommand::new("weather").description("Give me the current weather")],
            )
            .await
            .unwrap();
    }

    // `interaction_create` runs when the user interacts with the bot
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // check if the interaction is a command
        if let Interaction::Command(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "weather" => thread::spawn(|| weather::weather().to_owned())
                    .join()
                    .expect("Thread panicked."),
                command => unreachable!("Unknown command: {}", command),
            };
            // send `response_content` to the discord server
            let builder: CreateInteractionResponse = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(response_content),
            );
            command
                .create_response(&ctx.http, builder)
                .await
                .expect("Cannot respond to slash command");
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
