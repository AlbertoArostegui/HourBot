use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use serenity::model::{guild::Member, user::User};
use serenity::prelude::*;
use serenity::model::timestamp::Timestamp;

use std::io::prelude::*;
use std::fs::File;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Maricon").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn voice_state_update(&self, _ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
       
        let server = _new.guild_id.unwrap();
        let new_chann = _new.channel_id.expect("se fue");
        let member = _new.member.unwrap();
        let user = member.user;
        let name = &user.name;
        let time = Timestamp::now().unix_timestamp();
        

        println!("Actualizacion en {}", server);
        println!("{}, {} se movio a {}", user, name, new_chann);
        println!("{}", time);
    }
}

#[tokio::main]
async fn main() {

    let mut file = File::open(".token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Token file expected to be within the directory");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}