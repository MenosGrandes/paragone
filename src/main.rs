mod data_types;
mod db;
pub mod endpoints;

use actix_web::{App, HttpServer};

use actix_web::web::Data;
use data_types::product::{Price, Product, ProductAndPrice, Store, Ticket, Weight};
use db::db::DataBaseF;
use endpoints::endpoints::list_all_tickets;
use endpoints::endpoints::add_new_ticket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let data_base = DataBaseF::init().await.expect("Cannot create DATABASE");

    let p: Product = Product::new(None, "aaa".to_string());
    let p2: Product = Product::new(None, "bbb".to_string());
    let pandp: ProductAndPrice = ProductAndPrice::new(p, Price::new(23, false), Weight::new(12));
    let pandp2: ProductAndPrice = ProductAndPrice::new(p2, Price::new(33, true), Weight::new(1));
    let s: Store = Store::new(None, "Lidl".to_string());

    let time: i32 = 1;
    let ticket: Ticket = Ticket::new(None, vec![pandp, pandp2], time, s, "Dupa".to_string());
    println!("{}",serde_json::to_string(&ticket)?);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(data_base.clone()))
            .service(add_new_ticket)
            .service(list_all_tickets)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
