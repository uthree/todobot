use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group,
    },
    Args,
    help_commands,
    HelpOptions,
    CommandGroup,
};
use serenity::framework::standard::macros::help;
use std::collections::HashSet;
use serenity::model::prelude::{Message, UserId};

use std::{env};
use std::path::Path;
use std::fs::File;
use dotenv::dotenv;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(">")) // set the bot's prefix to ""
        .group(&GENERAL_GROUP)
        .help(&HELP_COMMAND);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

enum TaskStatus {
    Waiting,
    Doing,
    Complete,
    Cancelled
}

struct Task{
    name : String,
    description : String,
    status : TaskStatus,
}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[help]
async fn help_command(
   context: &Context,
   msg: &Message,
   args: Args,
   help_options: &'static HelpOptions,
   groups: &[&'static CommandGroup],
   owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}


