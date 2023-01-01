mod data_types;
mod db;
pub mod endpoints;

use actix_web::{App, HttpServer};

use actix_web::web::Data;
use db::db::DataBaseF;
use endpoints::products::{get_product_by_name, list_all_products};
use endpoints::stores::{add_new_store, list_all_stores};
use endpoints::tickets::{add_new_ticket, list_all_tickets};

use crate::endpoints::products::add_new_product;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let data_base = DataBaseF::init().await.expect("Cannot create DATABASE");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(data_base.clone()))
            .service(add_new_ticket)
            .service(list_all_tickets)
            .service(list_all_products)
            .service(add_new_product)
            .service(get_product_by_name)
            .service(add_new_store)
            .service(list_all_stores)
    })
    .bind(("192.168.1.156", 8086))?
    .run()
    .await
}
