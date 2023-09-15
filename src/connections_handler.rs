use serenity::model::voice::VoiceState;




pub fn connect(state: VoiceState) {
    let member = state.member.unwrap();
    let user = member.user.name;

    println!("Se ha conectado {}", user);
}

pub fn disconnect(state: VoiceState) {

}

pub fn moved(state: VoiceState) {

}