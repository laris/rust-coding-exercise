//use rocket::http::hyper::body::HttpBody;
use super::{/*Database, */model};
use crate::data::{DataError, DatabasePool};
use crate::ShortCode;
use crate::web::api::ApiKey;
use sqlx::Row;

type Result<T> = std::result::Result<T, DataError>;

pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<()> {
    let shortcode = shortcode.as_str();
    Ok(sqlx::query!(
        "UPDATE clips SET hits = hits + ? WHERE shortcode = ?",
        hits,
        shortcode
    )
        .execute(pool)
        .await
        .map(|_| ())?
    )
}
pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
    ) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)    
}

pub async fn new_clip<M: Into<model::NewClip>> (
    model: M,
    pool: &DatabasePool)
    -> Result<model::Clip> {
        let model = model.into();
        let _ = sqlx::query!(
            r#"INSERT INTO clips (
                clip_id,
                shortcode,
                content,
                title,
                posted,
                expires,
                password,
                hits)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            model.clip_id,
            model.shortcode,
            model.content,
            model.title,
            model.posted,
            model.expires,
            model.password,
            0
        )
        .execute(pool)
        .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
            content = ?,
            expires = ?,
            password = ?,
            title = ?
           WHERE shortcode = ? 
        "#,
        model.content,
        model.expires,
        model.password,
        model.title, 
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();
    let _ = sqlx::query!("INSERT INTO api_keys (api_key) VALUES (?)", bytes)
        .execute(pool)
        .await
        .map(|_| ())?;
    Ok(api_key)
}

pub enum RevocationStatus {
    Revoked,
    NotFound,
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.clone().into_inner();
    Ok(sqlx::query!("DELETE FROM api_keys WHERE api_key == ?", bytes)
        .execute(pool)
        .await
        .map(|result|
            match result.rows_affected() {
            0 => RevocationStatus::NotFound,
            1 => RevocationStatus::Revoked,
            _ => RevocationStatus::NotFound,
        })?,
    )
}
pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys WHERE api_key = ?")
        .bind(bytes)
        .fetch_one(pool)
        .await
        .map(|row| {
            let count: u32 = row.get(0);
            count > 0
        })?,
    )
}

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64> {
    Ok(
        sqlx::query!(
            r#"DELETE FROM clips WHERE strftime('%s', 'now') > expires"#)
            .execute(pool)
            .await?
            .rows_affected()
    )
}

#[cfg(test)]
pub mod test {
    use crate::data::test::*;
    use crate::data::*;
    use crate::test::async_runtime;

    fn model_get_clip(shortcode: &str) -> model::GetClip {
        model::GetClip {
            shortcode: shortcode.into()
        }
    }

    fn model_new_clip(shortcode: &str) -> model::NewClip {
        use chrono::Utc;
        model::NewClip {
            clip_id: DbId::new().into(),
            content: format!("content for clip '{shortcode}"),
            title: None,
            shortcode: shortcode.into(),
            posted: Utc::now().timestamp(),
            expires: None,
            password: None,
        }
    }

    fn clip_new_and_get() {
        let rt = async_runtime();
        let db = new_db(rt.handle());
        let pool = db.get_pool();

        let clip = rt.block_on(async move {
            super::new_clip(model_new_clip("1"), &pool.clone()).await
        });
        assert!(clip.is_ok());
        let clip = clip.unwrap();
        assert!(clip.shortcode == "1");
        assert!(clip.content == format!("content for clip '1'"));
    }
}