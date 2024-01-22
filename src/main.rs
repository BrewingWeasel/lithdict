use std::collections::HashMap;
use std::fs::read_to_string;

use actix_web::http::header::ContentType;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use html2text::from_read;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[get("/define/{word}")]
async fn get_def(word: web::Path<String>) -> impl Responder {
    let v = get_def_conts(&word).await;
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(v)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_def))
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub view_html: String,
}

async fn get_def_conts(word: &str) -> String {
    let client = reqwest::Client::new();

    // TODO: don't  read this all every single time
    let conts = read_to_string("uuids").unwrap();
    let mut words = HashMap::new();
    for i in conts.lines() {
        let (cur_word, uuid) = i.split_once('\t').unwrap();
        words.insert(cur_word, uuid);
    }

    let uuid = if let Some(u) = words.get(word) {
        *u
    } else {
        return String::new();
    };

    client
        .get(format!(
            "https://ekalba.lt/action/vocabulary/record/{uuid}?viewType=64"
        ))
        .send()
        .await
        .unwrap()
        .json::<Root>()
        .await
        .unwrap()
        .details
        .view_html

    // let html = result.details.view_html.as_bytes();
    // from_read(html, usize::max_value())
    //     .lines()
    //     .fold(String::new(), |mut acc, v| {
    //         acc.push_str(v);
    //         acc
    //     })
}
