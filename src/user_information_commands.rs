use poise::serenity_prelude;
use poise::serenity_prelude::UserId;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{Context, Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command, ephemeral)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity_prelude::User>,
) -> serenity_prelude::Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created at {}",
        u.global_name.as_ref().unwrap(),
        u.created_at()
    );
    ctx.say(response).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    user_id: UserId,
    age: Option<u8>,
    legal_birthday: Option<LegalBirthday>,
    spiritual_birthday: Option<SpiritualBirthday>,
}

impl UserData {
    pub fn new(name: String, user_id: UserId) -> Self {
        UserData {
            name,
            user_id,
            age: None,
            legal_birthday: None,
            spiritual_birthday: None,
        }
    }

    fn update_name(&mut self, name: String) {
        self.name = name;
    }

    fn update_age(&mut self, age: u8) {
        self.age = Some(age);
    }

    fn update_legal_birthday(&mut self, legal_birthday: LegalBirthday) {
        self.legal_birthday = Some(legal_birthday);
    }

    fn update_spiritual_birthday(&mut self, spiritual_birthday: SpiritualBirthday) {
        self.spiritual_birthday = Some(spiritual_birthday);
    }
}

#[derive(Serialize, Deserialize)]
struct LegalBirthday {
    month: u8,
    day: u8,
    year: Option<u32>,
}

impl LegalBirthday {
    pub fn new(month: u8, day: u8, year: Option<u32>) -> Self {
        LegalBirthday { month, day, year }
    }
}

#[derive(Serialize, Deserialize)]
struct SpiritualBirthday {
    month: u8,
    day: u8,
    year: Option<u32>,
}

impl SpiritualBirthday {
    fn new(month: u8, day: u8, year: Option<u32>) -> Self {
        SpiritualBirthday { month, day, year }
    }
}

#[poise::command(slash_command)]
pub async fn legal_birthday(
    ctx: Context<'_>,
    #[description = "Month"] month: u8,
    #[description = "Day"] day: u8,
    #[description = "Year"] year: u32,
) -> serenity_prelude::Result<(), Error> {
    let legal_birthday: LegalBirthday = LegalBirthday::new(month, day, Some(year));
    let name: String = ctx.author().to_string();
    let user_id: UserId = ctx.author().id;

    let mut user_data: UserData = UserData::new(name, user_id);
    user_data.update_legal_birthday(legal_birthday);
    let response: String = if user_data.legal_birthday.as_ref().unwrap().year.is_some() {
        format!(
            "Hey {}! Your birthday is saved as {}/{}/{}!",
            user_data.name,
            user_data.legal_birthday.as_ref().unwrap().month,
            user_data.legal_birthday.as_ref().unwrap().day,
            user_data.legal_birthday.as_ref().unwrap().year.unwrap()
        )
    } else {
        format!(
            "Hey {}! Your birthday is saved as {}/{}!",
            user_data.name,
            user_data.legal_birthday.as_ref().unwrap().month,
            user_data.legal_birthday.as_ref().unwrap().day,
        )
    };
    write_to_redis(&ctx, user_data);
    ctx.reply(response).await?;
    Ok(())
}

fn write_to_redis(ctx: &Context<'_>, user_data: UserData) {
    // todo!("Writing to Redis is not yet implemented")
    let redis_client = &ctx.data().redis_client;
    let user_data_json =
        serde_json::to_string(&user_data).expect("Serde failed to convert UserData into a json");
    // redis_client.set(user_data.user_id.to_string(), user_data_json);
    let mut conn = redis_client
        .connection
        .lock()
        .unwrap()
        .get_connection()
        .unwrap();
    let _: () = conn.set("user", user_data.user_id.to_string()).expect("Redis set failed");
}
