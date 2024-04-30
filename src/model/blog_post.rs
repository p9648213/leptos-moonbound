use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Post {
        Post {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
