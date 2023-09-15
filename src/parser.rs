use mongodb::{Client, options::{ClientOptions, ResolverConfig}, bson::Document};
use std::{env, thread};
use std::error::Error;
use tokio;
use mongodb::bson::doc;


#[tokio::main]
pub async fn connect() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:

    let new_doc = doc! {
        "userid": 8813,
        "names": ["prueba", "prueba2"],
        "totalHours": 32,
        "servers": [{
            "server_id": 71112,
            "server_name": "theQ2",
            "channels": [{
                "channel_id": 515231,
                "channel_name": "sala pubica",
                "hours": 12
            }]
        }]
    };
    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
    ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;

    let client = Client::with_options(options.unwrap()).unwrap();


    // Print the databases in our MongoDB cluster

    let prueba = client.database("prueba2").collection("prombo");
    let user: Document = prueba.find_one(
        doc! {
            "userid": 8813
        },
        None
    ).await?.unwrap();
    println!("user: {}", user);
        
    Ok(())
}
