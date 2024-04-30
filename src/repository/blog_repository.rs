use crate::model::blog_post::Post;
#[cfg(feature = "ssr")]
use actix_web::rt::time::sleep;
#[cfg(feature = "ssr")]
use actix_web::web::Data;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use std::time::Duration;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    let data: Data<Pool<Sqlite>> = extract().await?;
    let pool = data.get_ref().clone();
    let id = id.unwrap_or(Uuid::new_v4().to_string());

    sqlx::query(
        "INSERT INTO post VALUES($1, $2, $3, $4, $5)
                 ON CONFLICT (id) DO UPDATE SET dt=excluded.dt,
                 image_url=excluded.image_url,
                 title=excluded.title,
                 text=excluded.text",
    )
    .bind(&id)
    .bind(&dt)
    .bind(&image_url)
    .bind(&title)
    .bind(&text)
    .execute(&pool)
    .await?;

    Ok(id)
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    let data: Data<Pool<Sqlite>> = extract().await?;
    let pool = data.get_ref().clone();

    sleep(Duration::from_millis(1000)).await;

    let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(res)
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    _oldest: Option<String>,
    _newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    let data: Data<Pool<Sqlite>> = extract().await?;
    let pool = data.get_ref().clone();

    let res: Vec<Post> = sqlx::query_as(
        "SELECT id, dt, image_url, title,
                CASE
                    WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1, ?) || '...'
                    ELSE text
                END AS text
        FROM post
        ORDER BY dt DESC
        LIMIT $2",
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&pool)
    .await?;

    Ok(res)
}
