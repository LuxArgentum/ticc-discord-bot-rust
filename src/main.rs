use poise::serenity_prelude as serenity;

use crate::redis_client::RedisClient;

mod moderator_utils;
mod redis_client;
mod social_commands;
mod user_information_commands;

struct Data {
    redis_client: RedisClient,
}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

enum Testing {
    Testing(serenity::GuildId),
    NotTesting,
}

#[tokio::main]
async fn main() {
    run_server().await;
}

async fn connect_redis() -> RedisClient {
    dotenv::dotenv().ok();
    let redis_url = std::env::var("REDISCLOUD_URL").expect("REDISCLOUD_URL must be set");
    let redis_client = RedisClient::new(&redis_url)
        .await
        .expect("Failed to create Redis client");

    redis_client
}

async fn run_server() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let testing = match std::env::var("GUILD_ID") {
        Ok(id) => {
            let id: u64 = id.parse().expect("not a 64-bit unsigned integer");
            Testing::Testing(id.into())
        }
        Err(_) => Testing::NotTesting,
    };

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                user_information_commands::age(),
                user_information_commands::legal_birthday(),
                social_commands::quiet_time(),
                moderator_utils::register(),
                moderator_utils::announcement(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    println!("bruh, what happened?");
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                                println!("Found a RoleParseError: {:?}", error);
                            } else {
                                println!("Not a RoleParseError :(");
                            }
                        }
                        other => poise::builtins::on_error(other).await.unwrap(),
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                match testing {
                    Testing::Testing(guild_id) => {
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            guild_id,
                        )
                        .await?;
                    }
                    Testing::NotTesting => {}
                }
                Ok(Data {
                    redis_client: connect_redis().await,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
