use serenity::{model::voice::VoiceState, client::Cache};
use crate::parser;
use crate::redis_handler;
use tokio;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn connected(state: VoiceState, cache: &Cache) {

    let mut ts: u64 = 0;

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            ts = n.as_secs();
            println!("UNIX timestamp: {}", ts);
        }
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }

    let user_id = state.user_id.0;
    let channel = state.channel_id.unwrap();
    let server_id = state.guild_id.unwrap().0;
    let server_name = state.guild_id.unwrap().name(cache).unwrap();
    let member = state.member.unwrap();
    let user_name = member.user.name;

    print!("Voice update at {} \n", ts);

    if !parser::user_exists(user_id as i64).await {

        println!("User {} doesn't exist in the db", user_id);

        parser::create_user(user_id, &user_name, &server_name, server_id, channel, cache).await;

    } else if !parser::server_exists(user_id as i64, server_id as i64).await {

        println!("Server doesn't exist in the db, adding");
        
        parser::insert_new_server(user_id as i64, server_id as i64, &server_name, channel, cache).await;

    } else if !parser::channel_exists(user_id as i64, channel).await {

        println!("Channel doesn't exist in the db, adding");

        parser::insert_new_channel(user_id as i64, server_id as i64, channel, cache).await;

    } else if !parser::channel_name_exists(user_id as i64, channel, cache).await {

        println!("New channel name detected for this channel id, adding it to the list");

        parser::insert_new_channel_name(user_id as i64, server_id as i64, channel, cache).await;

    }
    if !parser::user_name_exists(user_id as i64, &user_name).await {
        println!("New user name detected for user id {}, adding", user_id);
        parser::insert_new_user_name(user_id as i64, &user_name).await;
    }

    let mut con = redis_handler::establish_connection().unwrap();
    redis_handler::insert_ts(&mut con, user_id as i64, ts);

}

pub async fn disconnected(state: VoiceState, _cache: &Cache) {

    let mut ts: u64 = 0;

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            ts = n.as_secs();
            println!("UNIX timestamp: {}", ts);
        }
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }
    println!("{}", ts);

    let user_id = state.user_id.0;
    let channel = state.channel_id.unwrap();
    
    parser::update_seconds(user_id as i64, channel, ts).await;

}

pub async fn moved(state: VoiceState, cache: &Cache) {

    let mut ts: u64 = 0;

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            ts = n.as_secs();
            println!("UNIX timestamp: {}", ts);
        }
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }
    println!("{}", ts);

    let user_id = state.user_id.0;
    let channel = state.channel_id.unwrap();
    let server_id = state.guild_id.unwrap().0;
    let server_name = state.guild_id.unwrap().name(cache).unwrap();
    let member = state.member.unwrap();
    let user_name = member.user.name;

    parser::update_seconds(user_id as i64, channel, ts).await;

    if !parser::channel_exists(user_id as i64, channel).await {

        println!("Channel doesn't exist in the db, adding");

        parser::insert_new_channel(user_id as i64, server_id as i64, channel, cache);

    } else if !parser::channel_name_exists(user_id as i64, channel, cache).await {

        println!("New channel name detected for this channel id, adding it to the list");

        parser::insert_new_channel_name(user_id as i64, server_id as i64, channel, cache);

    }
    if !parser::user_name_exists(user_id as i64, &user_name).await {
        println!("New user name detected for user id {}, adding", user_id);
        parser::insert_new_user_name(user_id as i64, &user_name);
    }

    let mut con = redis_handler::establish_connection().unwrap();
    redis_handler::insert_ts(&mut con, user_id as i64, ts);

}
