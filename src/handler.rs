use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn check_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_posts_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let posts = db::fetch_posts(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &posts.into_iter().map(|t| PostResponse::of(t)).collect(),
    ))
}

pub async fn create_post_handler(body: PostRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&PostResponse::of(
        db::create_post(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_post_handler(
    id: i32,
    body: PostUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&PostResponse::of(
        db::update_post(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_posts_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_post(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}