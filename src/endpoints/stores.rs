use crate::data_types::product::Store;
use crate::DataBaseF;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[post("/add_store")]
pub async fn add_new_store(db: Data<DataBaseF>, store_json: Json<Store>) -> impl Responder {
    let store = store_json.into_inner();
    let user_db = db.create_store(store).await;
    match user_db {
        Ok(store) => HttpResponse::Ok().json(store),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/get_all_stores")]
pub async fn list_all_stores(db: Data<DataBaseF>) -> impl Responder {
    let all_stores = db.get_all_stores().await;
    match all_stores{
        Ok(q) => HttpResponse::Ok().json(q),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
