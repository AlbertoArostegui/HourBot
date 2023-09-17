use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::Document};
use serenity::{model::prelude::ChannelId, client::Cache};
use std::env;
use tokio;
use mongodb::bson::doc;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
pub async fn user_exists(user_id: i64) -> bool {
    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");
    let exists = db.find_one(
        doc! {
            "user_id": user_id
        },
        None
    ).await.unwrap();

    println!("{}", user_id);

    exists.is_some()
}

#[tokio::main]
pub async fn create_user(user_id: u64, user_name: &str, server_name: &str, server_id: u64, channel: ChannelId, cache: &Cache) {

    let channel_id = channel.0;
    let channel_name = channel.name(cache).await.unwrap();

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let new_doc = doc! {
         "user_id": user_id as i64,
         "names": [user_name],
         "total_minutos": 0,
         "servers": [{
            "server_id": server_id as i64,
            "server_name": server_name,
            "channels": [{
                "channel_id": channel_id as i64,
                "channel_names": [channel_name],
                "minutes": 0
            }]
         }]
    };

    db.insert_one(new_doc, None).await.expect("Couldn't create new user");


}

#[tokio::main]
pub async fn user_name_exists(user_id: i64, user_name: &str) -> bool {

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");
    let exists = db.find_one(
        doc! {
            "user_id": user_id,
            "names": user_name
        },
        None
    ).await.unwrap();

    println!("{}", user_id);

    exists.is_some()

}

#[tokio::main]
pub async fn insert_new_user_name(user_id: i64, user_name: &str) {

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id};
    let update = doc! {
        "$push": {
            "names": user_name
        }
    };

    db.update_one(filter, update, None).await.expect("Counldn't insert new user name");

}

#[tokio::main]
pub async fn server_exists(user_id: i64, server_id: i64) -> bool {


    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let exists = db.find_one(
        doc! {
            "user_id": user_id,
            "servers.server_id": server_id,
        }, None).await.unwrap();

    exists.is_some()

}

#[tokio::main]
pub async fn insert_new_server(user_id: i64, server_id: i64, server_name: &str, channel: ChannelId, cache: &Cache) {

    let channel_id = channel.0;
    let channel_name = channel.name(cache).await;

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id};
    let update = doc! {
        "$push": {
            "servers": doc! {
                "server_id": server_id,
                "server_name": server_name,
                "channels": [ doc! {
                    "channel_id": channel_id as i64,
                    "channel_names": [channel_name],
                    "minutes": 0
                }]
            }            
        }
    };

    db.update_one(filter, update, None).await.expect("Couldn't insert new server");
}

#[tokio::main]
pub async fn channel_exists(user_id: i64, channel: ChannelId) -> bool {
   
    let channel_id = channel.0 as i64;

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let exists = db.find_one(
        doc! {
            "user_id": user_id,
            "servers.channels.channel_id": channel_id,
        }, None).await.unwrap();

    exists.is_some()
}

#[tokio::main]
pub async fn insert_new_channel(user_id: i64, server_id: i64, channel: ChannelId, cache: &Cache) {

    let channel_id = channel.0 as i64;
    let channel_name = channel.name(cache).await.unwrap();

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id, "server_id": server_id};
    let update = doc! {
        "$push": {
            "servers.$.channels": doc! {
                "channel_id": channel_id,
                "channel_names": [channel_name],
                "minutes": 0
            }
        }
    };

    db.update_one(filter, update, None).await.expect("Couldn't add new channel to the db");

}

#[tokio::main]
pub async fn channel_name_exists(user_id: i64, channel: ChannelId, cache: &Cache) -> bool {

    let channel_id = channel.0 as i64;
    let channel_name = channel.name(cache).await.unwrap();

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let exists = db.find_one(
        doc! {
            "user_id": user_id,
            "servers.channels.channel_id": channel_id,
            "servers.channels.channel_names": channel_name
        },
        None
    ).await.unwrap();

    exists.is_some()

}

#[tokio::main]
pub async fn insert_new_channel_name(user_id: i64, server_id: i64, channel: ChannelId, cache: &Cache) {

    let channel_id = channel.0 as i64;
    let channel_name = channel.name(cache).await.unwrap();

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id, "servers.server_id": server_id, "servers.channels.channel_id": channel_id};
    let update = doc! {
        "$push": {
            "servers.$.channels.$.channel_names": channel_name
        }
    };

    db.update_one(filter, update, None).await.expect("Couldn't insert new channel name");

}


#[tokio::main]
pub async fn make_ts(user_id: i64, channel_id: i64) {

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("temp").collection("user_temps");

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let new_doc = doc! {
        "user_id": user_id,
        "channel_id": channel_id,
        "timestamp": since_the_epoch as i64,
    };

    db.insert_one(new_doc, None).await.expect("Couldn't make a timestamp");
}