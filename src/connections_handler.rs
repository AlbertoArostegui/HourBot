use serenity::model::voice::VoiceState;
use crate::parser;

pub fn connected(state: VoiceState) {
    let member = state.member.unwrap();
    let user_id = state.user_id;

    println!("Se ha conectado {}", user_id);
    let existe = parser::user_exists(user_id.0);
    println!("existe: {}", existe);
}

pub fn disconnected(state: VoiceState) {

}

pub fn moved(state: VoiceState) {

}
