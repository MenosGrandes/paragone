use std::u8;

use serde::{Deserialize, Serialize};

use crate::db::unique_id::UniqueDbId;
pub type DateTime = chrono::DateTime<chrono::Utc>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub name: String,
    pub product_type: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Weight {
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub value: u8,
    pub cheeper: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPriceWeight {
    pub product: Product,
    pub price: Price,
    pub weight: Weight,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<UniqueDbId>,
    pub purchase_date: String,
    pub store: Store,
    pub name: String,
    pub products: Vec<ProductPriceWeight>,
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

