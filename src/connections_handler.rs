use serenity::{model::voice::VoiceState, client::Cache};
use crate::parser;

pub fn connected(state: VoiceState, cache: &Cache) {
    
    let user_id = state.user_id.0;
    let channel = state.channel_id.unwrap();
    let server_id = state.guild_id.unwrap().0;
    let server_name = state.guild_id.unwrap().name(cache).unwrap();
    let member = state.member.unwrap();
    let user_name = member.user.name;

    if !parser::user_exists(user_id as i64) {

        println!("User {} doesn't exist in the db", user_id);

        parser::create_user(user_id, &user_name, &server_name, server_id, channel, cache);

    } else if parser::server_exists(user_id as i64, server_id as i64) {
        
        parser::insert_new_server(user_id as i64, server_id as i64, &server_name, channel, cache);

    } else if parser::channel_exists(user_id as i64, channel) {

        parser::insert_new_channel(user_id as i64, server_id as i64, channel, cache);

    } else if parser::channel_name_exists(user_id as i64, channel, cache) {

        parser::insert_new_channel_name(user_id as i64, server_id as i64, channel, cache);

    }
    if !parser::user_name_exists(user_id as i64, &user_name) {
        parser::insert_new_user_name(user_id as i64, &user_name)
    }
}

pub fn disconnected(state: VoiceState) {

}

pub fn moved(state: VoiceState) {

}
