use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::Result as Fallible;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    account: String,
    url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Account {
    avatar: String,
    url: String,
    username: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct TootContext {
    account: Account,
    content: String,
    created_at: String,
}

pub async fn get_toot_context(url: String) -> Fallible<Vec<TootContext>> {
    let client = Client::new();

    Ok(client
        .get(url)
        .send()
        .await?
        .json::<Vec<TootContext>>()
        .await?)
}

#[get("/")]
async fn list() -> impl Responder {
    let data = fs::read_to_string("./db/subscriptions.json")
        .expect("Unable to read subscription file");

    let subscriptions: Vec<Subscription> = match serde_json::from_str(&data) {
        Ok (subscriptions) => subscriptions,
        Err (error) => {
            println!("ERROR deserializing subscriptions {}", error.to_string());
            Vec::new()
        },
    };

    let mut resp = Vec::new();

    for subscription in subscriptions {
        let url = format!("{}/statuses", subscription.url);
        println!("Fetching {}", url);
        let mut toots = match get_toot_context(url).await {
            Ok (toots) => toots,
            Err (error) => {
                println!("ERROR getting toots {}", error.to_string());
                Vec::new()
            }
        };
        resp.append(&mut toots); //forces using mut above, not nice
    }

    resp.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let output = resp
        .into_iter()
        .filter(|t| t.content != "") // do not display boosts etc
        .map(|t| {
            // haha my frontend
            format!("
            <div style=\"border: 1px solid #000; background-color: #eef; padding: 5px; margin: 5px\">
                <div style=\"display: flow-root\">
                    <img src=\"{}\" align=\"left\" width=\"64\" height=\"64\" />
                    <a href=\"{}\" target=\"_blank\">{}</a><br />
                    {}
                </div>
                <div>
                    {}
                </div>
            </div>
            ",
            t.account.avatar,
            t.account.url,
            t.account.username,
            t.created_at,
            t.content)
        })
        .collect::<String>();
        
    HttpResponse::Ok().body(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}