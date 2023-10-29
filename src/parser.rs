use mongodb::{Client, options::{ClientOptions, ResolverConfig, FindOneOptions}};
use serenity::{model::prelude::ChannelId, client::Cache};
use std::env;
use crate::redis_handler;
use futures::stream::StreamExt;

use mongodb::bson::{self, doc, Document};

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

    println!("Checking wheter user {} exists", user_id);

    exists.is_some()
}


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

    println!("Checking if {}'s current user name: {} exists", user_id, user_name);

    exists.is_some()

}


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


pub async fn server_exists(user_id: i64, server_id: i64) -> bool {


    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id, "servers.server_id": server_id};

    println!("Checking whether {} exists", filter);

    let exists = db.find_one(
        Some(filter), None).await.unwrap();

    exists.is_some()

}


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

    println!("Trying to add a new server \n\n");
    match db.update_one(filter, update, None).await {
        Ok(result) => println!("Matched {} and modified {} documents", result.matched_count, result.modified_count),
        Err(e) => eprintln!("Update failed: {}", e),
    }
}


pub async fn channel_exists(user_id: i64, channel: ChannelId) -> bool {
   
    let channel_id = channel.0 as i64;

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    println!("Checking whether channel exists");

    let exists = db.find_one(
        Some(doc! {
            "user_id": user_id,
            "servers.channels.channel_id": channel_id,
        }), None).await.unwrap();

    exists.is_some()
}

pub async fn insert_new_channel(user_id: i64, server_id: i64, channel: ChannelId, cache: &Cache) {

    let channel_id = channel.0 as i64;
    let channel_name = channel.name(cache).await.unwrap();

    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    println!("Attempting to insert new channel in an already existing server");

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    let filter = doc! {"user_id": user_id, "servers.server_id": server_id};

    println!("Checking whether {} exists", filter);

    let exists = db.find_one(
        Some(filter), None).await.unwrap();

    if exists.is_some() {
        println!("Encontrado el documento en el que insertarlo");
    } else {
        println!("No se ha encontrado el documento al que pertenece el canal");
    }
    let filter = doc! {"user_id": user_id, "servers.server_id": server_id};
    let new_channel = doc! {"channel_id": channel_id, "channel_names": [channel_name], "minutes": 0};
    let update = doc! {
        "$push": {
            "servers.$.channels": new_channel
        }
    };

    let result = db.update_one(filter, update, None).await.expect("Couldn't add new channel to the db");

    println!("{} document(s) updated", result.modified_count);

}


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

    println!("Checking whether channel name exists");

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



pub async fn update_seconds(user_id: i64, channel: ChannelId, ts: u64) {

    let channel_id = channel.0 as i64;
    
    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    let client = Client::with_options(options.unwrap()).unwrap();

    let db: mongodb::Collection<Document> = client.database("prueba2").collection("prombo");

    println!("Obtaining total seconds for {} in channel {}", user_id, channel_id);

    let filter = doc! {"user_id": user_id, "servers.channels.channel_id": channel_id};

    let options = FindOneOptions::builder()
                    .projection(bson::doc! { "minutes": 1, "_id": 0 })
                    .build();

    let total_seconds =  db.find_one(Some(filter), None).await.unwrap();

    if total_seconds.is_some() {
        println!("Encontrado el canal para obtener segundos");
    } else {
        println!("No encontrado el canal");
    }

    let total_seconds = total_seconds.unwrap();

    match find_minutes(&total_seconds, channel_id) {
        Some(minutes) => println!("Minutes: {}", minutes),
        None => println!("Channel not found"),
    }
    
    //println!("The total seconds are: {}", total_seconds.unwrap());

    let mut con = redis_handler::establish_connection().unwrap();
    let ts_inicial: u64 = redis_handler::get_ts(&mut con, user_id).unwrap().parse().unwrap();

    println!("Ha estado en la sala {} durante {} segundos", channel_id, ts-ts_inicial);

    let mut new_time: i32 = (ts-ts_inicial) as i32;
    new_time = new_time + find_minutes(&total_seconds, channel_id).unwrap();

    update_minutes(&db, channel_id, user_id, new_time).await;
}


fn find_minutes(data: &Document, channel_id: i64) -> Option<i32> {
    if let Some(servers) = data.get_array("servers").ok() {
        for server in servers {
            if let Some(server_doc) = server.as_document() {
                if let Some(channels) = server_doc.get_array("channels").ok() {
                    for channel in channels {
                        if let Some(channel_doc) = channel.as_document() {
                            if channel_doc.get_i64("channel_id").ok() == Some(channel_id) {
                                return channel_doc.get_i32("minutes").ok();
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

async fn update_minutes(collection: &mongodb::Collection<Document>, channel_id: i64, user_id: i64, new_minutes: i32) {
    // 1. Fetch the Document
    let filter = doc! {
        "user_id": user_id,
        "servers.channels.channel_id": channel_id
    };
    let mut cursor = collection.find(filter, None).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                // 2. Modify the Document
                let mut modified_document = modify_document(document, channel_id, new_minutes);

                // 3. Update the Document
                let filter = doc! {
                    "_id": modified_document.get_object_id("_id").unwrap().clone(),
                };
                let update = doc! {
                    "$set": {
                        "servers": modified_document.get_array("servers").unwrap().clone(),
                    }
                };
                match collection.update_one(filter, update, None).await {
                    Ok(result) => println!("Matched {} and modified {} documents", result.matched_count, result.modified_count),
                    Err(e) => eprintln!("Update failed: {}", e),
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn modify_document(mut document: Document, channel_id: i64, new_minutes: i32) -> Document {
    let servers = document.get_array_mut("servers").unwrap();
    for server in servers {
        if let bson::Bson::Document(server_doc) = server {
            let channels = server_doc.get_array_mut("channels").unwrap();
            for channel in channels {
                if let bson::Bson::Document(channel_doc) = channel {
                    if channel_doc.get_i64("channel_id").unwrap() == channel_id {
                        channel_doc.insert("minutes", new_minutes);
                    }
                }
            }
        }
    }
    document
}