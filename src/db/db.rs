use futures::stream::TryStreamExt;
use mongodb::options::ClientOptions;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client};
#[derive(Clone)]
pub struct DataBaseF {
    client: Client,
}
use log::debug;

use crate::data_types::product::Ticket;
const TICKET_DATABASE_NAME: &'static str = "paragone";
const TICKET_COLLECTION_NAME: &'static str = "tickets";
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

    pub async fn get_all_tickets(&self) -> Result<Vec<Ticket>, Error> {
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
