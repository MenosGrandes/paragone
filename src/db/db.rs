use crate::question::question::Question;
use futures::stream::TryStreamExt;
use mongodb::options::ClientOptions;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};
#[derive(Clone)]
pub struct DataBaseF {
    client: Client,
}
use log::{debug, error, info, log_enabled, Level};
impl DataBaseF {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("familyFraud");
        for db_name in client.list_database_names(None, None).await? {
            debug!("{}", db_name);
        }
        Ok(DataBaseF { client })
    }
}
/*
impl DataBaseF {
    pub async fn create_question(&self, new_question: Question) -> Result<InsertOneResult, Error> {
        let new_doc = Question {
            id: None,
            name: new_question.name,
            location: new_question.location,
            title: new_question.title,
        };
        let question = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating Question");
        Ok(question)
    }
*/
impl DataBaseF {
    pub async fn get_all_questions(&self) -> Result<Vec<Question>, Error> {
        let mut cursors = self
            .client.database("familyFraud").collection("Questions")
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of Questions");
        let mut questions: Vec<Question> = Vec::new();
        while let Some(question) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            questions.push(question)
        }
        Ok(questions)
    }
}
