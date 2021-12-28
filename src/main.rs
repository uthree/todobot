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
#[commands(ping, prefix, todo, list)]
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
                c.dynamic_prefix(|_, msg| Box::pin(async move {
                        userdata::init_if_not_exist(&msg.author.id);
                        Some(userdata::load(&msg.author.id).command_prefix)
                    })
                )
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

#[command]
#[num_args(1)]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix = args.single::<String>()?;
    let prefix2 = prefix.clone();
    userdata::init_if_not_exist(&msg.author.id);
    let mut data = userdata::load(&msg.author.id);
    data.command_prefix = prefix;
    userdata::save(&data, &msg.author.id);
    msg.reply(ctx, format!("Changed prefix: `{}`", prefix2)).await?;
    Ok(())
}

#[command]
#[min_args(1)]
#[max_args(2)]
#[aliases("todo", "new", "add")]
async fn todo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = userdata::load(&msg.author.id);
    let taskname = args.single::<String>()?;
    let task = userdata::Task::new(taskname.clone(), "".to_string());
    data.add_task(task);
    userdata::save(&data, &msg.author.id);
    msg.reply(ctx, format!("Added task `{}`", &taskname)).await?;
    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let data = userdata::load(&msg.author.id);
    let tasks = data.get_tasks();
    let mut message = String::new();
    for task in tasks {
        if task.status == userdata::TaskStatus::Waiting {
            message.push_str(":blue_circle:");
        }
        if task.status == userdata::TaskStatus::Doing {
            message.push_str(":yellow_circle:");
        }
        if task.status == userdata::TaskStatus::Complete {
            message.push_str(":green_circle:");
        }
        if task.status == userdata::TaskStatus::Cancelled {
            message.push_str(":red_circle:");
        }
        message.push_str(&format!("**{}** \n{}", task.name, task.description));

    }
    msg.reply(ctx, message).await?;
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


