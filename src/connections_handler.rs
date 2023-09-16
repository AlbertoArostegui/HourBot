use serenity::{model::voice::VoiceState, client::Cache};
use crate::parser;

pub fn connected(state: VoiceState, cache: &Cache) {
    
    let user_id = state.user_id;
    let channel = state.channel_id.unwrap();
    let server_id = state.guild_id.unwrap().0;

    if !parser::user_exists(user_id.0 as i64) {

        println!("User {} doesn't exist in the db", user_id);

        let member = state.member.unwrap();
        let user_name = member.user.name;
        let server_name = state.guild_id.unwrap().name(cache).unwrap();
        
        

        parser::create_user(user_id.0, &user_name, &server_name, server_id, channel, cache);

    } else {
        
        println!("User {} already exists in the db", user_id);
        let member = state.member.unwrap();
        let user_name = member.user.name;

        if !parser::user_name_exists(user_id.0 as i64, &user_name) {
            parser::insert_new_user_name(user_id.0 as i64, &user_name)
        }

        if !parser::channel_name_exists(user_id.0 as i64, channel, cache) {

        }

        
        //let channel_name = state.channel_id.unwrap().name(cache).await.unwrap();

    }
    


}

pub fn disconnected(state: VoiceState) {

}

pub fn moved(state: VoiceState) {

}

pub fn pre_checks(user_id: u64, user_name: &str, channel_name: &str) {
    let existe = parser::user_exists(user_id as i64);
}
