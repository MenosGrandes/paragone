use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{data_types::product::Product, db::db::DataBaseF};

use log::debug;
#[post("/add_product")]
pub async fn add_new_product(db: Data<DataBaseF>, product_json: Json<Product>) -> impl Responder {
    let product = product_json.into_inner();
    let product_in_db = db.create_product(product).await;
    match product_in_db {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/get_all_products")]
pub async fn list_all_products(db: Data<DataBaseF>) -> impl Responder {
    let all_tickets = db.get_all_products().await;
    match all_tickets {
        Ok(q) => HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NameQuery
{
    pub name : String
}

#[get("/get_product_by")]
pub async fn get_product_by_name(db: Data<DataBaseF>, params : actix_web::web::Query<NameQuery>) -> impl Responder {
    let a = params.into_inner();
    let all_tickets = db.get_product_by_name(a.name).await;
    match all_tickets {
        Ok(q) => HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
