use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::ClientOptions;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client};
#[derive(Clone)]
pub struct DataBaseF {
    client: Client,
}
use log::debug;

use crate::data_types::product::{Product, Store, Ticket};
const TICKET_DATABASE_NAME: &'static str = "paragone";
const TICKET_COLLECTION_NAME: &'static str = "tickets";
const PRODUCT_COLLECTION_NAME: &'static str = "products";
const STORES_COLLECTION_NAME: &'static str = "stores";

impl DataBaseF {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        for db_name in client.list_database_names(None, None).await? {
            debug!("{}", db_name);
        }
        Ok(DataBaseF { client })
    }
}

impl DataBaseF {
    pub async fn create_store(&self, new_store: Store) -> Result<InsertOneResult, Error> {
        let product = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(STORES_COLLECTION_NAME)
            .insert_one(new_store, None)
            .await
            .ok()
            .expect("Error creating STORE");
        Ok(product)
    }

    pub async fn create_product(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        let product = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(PRODUCT_COLLECTION_NAME)
            .insert_one(new_product, None)
            .await
            .ok()
            .expect("Error creating ticket");
        Ok(product)
    }
    pub async fn create_ticket(&self, new_ticket: Ticket) -> Result<InsertOneResult, Error> {
        let new_doc = Ticket::copy_ctor(new_ticket);
        let ticket = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(TICKET_COLLECTION_NAME)
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating ticket");
        Ok(ticket)
    }

    pub async fn get_all_stores(&self) -> Result<Vec<Store>, Error> {
        let mut cursors = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(STORES_COLLECTION_NAME)
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of STORES");
        let mut stores: Vec<Store> = Vec::new();
        while let Some(ticket) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            stores.push(ticket)
        }
        Ok(stores)
    }
    pub async fn get_all_products(&self) -> Result<Vec<Product>, Error> {
        let mut cursors = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(PRODUCT_COLLECTION_NAME)
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of PRODUCTS");
        let mut products: Vec<Product> = Vec::new();
        while let Some(ticket) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            products.push(ticket)
        }
        Ok(products)
    }
    pub async fn get_product_by_name(&self, product_name: String) -> Result<Vec<Product>, Error> {
        let filter = doc! { "name": product_name.to_owned() };
        let mut cursors = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(PRODUCT_COLLECTION_NAME)
            .find(filter, None)
            .await
            .ok()
            .expect("Error getting list of PRODUCTS");
        let mut products: Vec<Product> = Vec::new();
        while let Some(ticket) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            products.push(ticket)
        }
        Ok(products)
    }
    pub async fn get_all_tickets(&self) -> Result<Vec<Ticket>, Error> {
        let pipeline = vec![
            doc! { "$lookup":{"from":"stores","localField" : "store_id","foreignField" : "_id","as":"store_info"}},
        ];
        let mut r = self.client.database(TICKET_DATABASE_NAME).collection::<Document>(TICKET_COLLECTION_NAME).aggregate(pipeline,None).await.expect("BLAD");
        while let Some(t) =  r.try_next().await.ok().expect("ERROR")
        {
            debug!("{}",t);//MENOSGRANDES UPDATE 
                           //https://github.com/mongodb/mongo-rust-driver/issues/514
        }
        let mut cursors = self
            .client
            .database(TICKET_DATABASE_NAME)
            .collection(TICKET_COLLECTION_NAME)
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of TICKETS");
        let mut tickets: Vec<Ticket> = Vec::new();
        while let Some(ticket) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            tickets.push(ticket)
        }
        Ok(tickets)
    }
}
