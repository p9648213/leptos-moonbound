use std::sync::Arc;

use crate::model::blog_post::Post;
use chrono::Local;
use leptos::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
use leptos_actix::extract;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;

    sqlx::query("")
        .bind(&id)
        .bind(&dt)
        .bind(&image_url)
        .bind(&title)
        .bind(&text)
        .bind(&pool.acquire())
        .await?;

    Ok(String::from("placeholder"))
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    let pool: Data<Pool<Sqlite>> = extract().await?;

    Ok(Post {
        id: "1".to_string(),
        dt: Local::now().naive_local(),
        title: "Ocean".to_string(),
        image_url: "https://bit.ly/3t0bA61".to_string(),
        text: "I spent some time at the beach today. It was amazing".to_string(),
    })
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    Ok(vec![
        Post {
            id: "1".to_string(),
            dt: Local::now().naive_local(),
            title: "Ocean".to_string(),
            image_url: "https://bit.ly/3t0bA61".to_string(),
            text: "I spent some time at the beach today. It was amazing".to_string(),
        },
        Post {
            id: "2".to_string(),
            dt: Local::now().naive_local(),
            title: "Desert".to_string(),
            image_url: "https://bit.ly/3t8HGMG".to_string(),
            text: "I spent some time at the beach today. It was amazing".to_string(),
        },
        Post {
            id: "3".to_string(),
            dt: Local::now().naive_local(),
            title: "Garden".to_string(),
            image_url: "https://bit.ly/3RfUxop".to_string(),
            text: "I spent some time at the beach today. It was amazing".to_string(),
        },
        Post {
            id: "4".to_string(),
            dt: Local::now().naive_local(),
            title: "Andromeda".to_string(),
            image_url: "https://bit.ly/47PKLQQ".to_string(),
            text: "I spent some time at the beach today. It was amazing".to_string(),
        },
    ])
}
