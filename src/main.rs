mod userdata;

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
use serenity::model::prelude::{channel::Message, gateway::Ready, id::UserId};

use std::{env};
use dotenv::dotenv;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Botが起動したときに走る処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c|
            {
                c.prefix(">");
                c.dynamic_prefix(|ctx, msg| {
                    let user_id = msg.author.id;
                    let userdata = userdata::load(&user_id);
                    userdata.command_prefix.clone()
                })
            }
        )
        .group(&GENERAL_GROUP)
        .help(&HELP_COMMAND)
        ;

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


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{}", msg.author.id);
    msg.reply(ctx, "Pong!").await?;
    userdata::init_if_not_exist(&msg.author.id);
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


