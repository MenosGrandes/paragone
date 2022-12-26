mod db;
mod endpoints;
mod question;
use crate::db::db::DataBaseF;
use crate::question::question::Question;
use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use endpoints::endpoints::{list_all, list_all2};
use env_logger::*;
use mongodb::{bson::doc, options::ClientOptions, Client, Cursor};
use std::sync::Mutex;

use futures::stream::TryStreamExt;
use log::{debug, error, info, log_enabled, Level};
#[get("/all3")]
pub async fn list_all3(client: Data<mongodb::Client>) -> HttpResponse {
    let collection: mongodb::Collection<Question> =
        client.database("familyFraud").collection("Questions");
    match collection.find(None, None).await {
        Ok(mut cursors) => {
            let mut questions: Vec<Question> = Vec::new();
            while let Some(question) = cursors
                .try_next()
                .await
                .ok()
                .expect("Error mapping through cursor")
            {
                questions.push(question)
            }
            return HttpResponse::Ok().json(questions);
        }

        Err(_) => HttpResponse::NotFound().body(format!("No question found with username ")),
    };

    debug!("call all3");
    return HttpResponse::Ok().into();
}
#[post("/question")]
pub async fn post_new_question(
    client: Data<mongodb::Client>,
    question: Json<Question>,
) -> HttpResponse {
    debug!("Insert question call");
    let collection: mongodb::Collection<Question> =
        client.database("familyFraud").collection("Questions");

    let q2 = Question {
        id: None,
        name: question.name.to_owned(),
        location: question.location.to_owned(),
        title: question.title.to_owned(),
    };

    let question = collection.insert_one(q2, None).await;

    match question {
        Ok(q) => return HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    };
    debug!("Insert question call");
    return HttpResponse::Ok().body("ENDED");
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017")
        .await
        .expect("a");
    let client: mongodb::Client = Client::with_options(client_options).unwrap();
    let data: Data<mongodb::Client> = Data::new(client.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(list_all2)
            .service(list_all3)
            .service(post_new_question)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
