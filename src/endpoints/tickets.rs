use crate::data_types::product::Ticket;
use crate::DataBaseF;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[post("/add_ticket")]
pub async fn add_new_ticket(db: Data<DataBaseF>, user: Json<Ticket>) -> impl Responder {
    let ticket = user.into_inner();
    let user_db = db.create_ticket(ticket).await;
    match user_db {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/get_all_tickets")]
pub async fn list_all_tickets(db: Data<DataBaseF>) -> impl Responder {
    let all_tickets = db.get_all_tickets().await;
    match all_tickets {
        Ok(q) => HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
