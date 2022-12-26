use crate::DataBaseF;
use crate::Question;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use log::{debug, error, info, log_enabled, Level};
use std::sync::Mutex;
/*
#[post("/question")]
pub async fn add_new_question(db: Data<DataBaseF>, user: Json<Question>) -> impl Responder {
    let q2 = Question {
        id: None,
        name: user.name.to_owned(),
        location: user.location.to_owned(),
        title: user.title.to_owned(),
    };
    let user_db = db.create_question(q2).await;
    match user_db {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/all3")]
pub async fn list_all3(_client: Data<mongodb::Client>) -> HttpResponse {
    debug!("CALL FOR ALL");
    HttpResponse::Ok().into()
}
*/
#[get("/all2")]
pub async fn list_all2() -> HttpResponse {
    debug!("CALL FOR ALL");
    HttpResponse::Ok().into()
}
#[get("/all")]
pub async fn list_all(db: Data<Mutex<DataBaseF>>) -> impl Responder {
    let all_questions = db.lock().unwrap().get_all_questions().await;
    debug!("CALL FOR ALL");
    match all_questions {
        Ok(q) => HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
