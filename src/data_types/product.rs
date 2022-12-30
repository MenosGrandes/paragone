use std::u8;

use serde::{Deserialize, Serialize};

use crate::db::unique_id::UniqueDbId;
pub type DateTime = chrono::DateTime<chrono::Utc>;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductProducer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub name: String,
}

impl ProductProducer {
    pub fn new(name: String) -> Self { Self { id : None, name } }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub name: String,
    pub product_producer: ProductProducer,
}

impl Product {
    pub fn new(name: String, product_producer: ProductProducer) -> Self { Self { id : None, name, product_producer } }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Weight {
    pub value: u8,
}

impl Weight {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub value: u8,
    pub cheeper: bool,
}

impl Price {
    pub fn new(value: u8, cheeper: bool) -> Self {
        Self { value, cheeper }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAndPrice {
    pub product: Product,
    pub price: Price,
    pub weight: Weight,
}

impl ProductAndPrice {
    pub fn new(product: Product, price: Price, weight: Weight) -> Self {
        Self {
            product,
            price,
            weight,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub name: String,
}

impl Store {
    pub fn new(id: Option<UniqueDbId>, name: String) -> Self {
        Self { id, name }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub products: Vec<ProductAndPrice>,
    pub purchase_date: i32,
    pub store: Store,
    pub name: String,
}

impl Ticket {
    pub fn copy_ctor(other: Ticket) -> Self {
        Self {
            id: None,
            products: other.products,
            purchase_date: other.purchase_date,
            store: other.store,
            name: other.name,
        }
    }
}

impl Ticket {
    pub fn new(
        id: Option<UniqueDbId>,
        products: Vec<ProductAndPrice>,
        purchase_date: i32,
        store: Store,
        name: String,
    ) -> Self {
        Self {
            id,
            products,
            purchase_date,
            store,
            name,
        }
    }
}
