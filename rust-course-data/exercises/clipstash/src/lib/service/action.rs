use crate::data::{query, DatabasePool, Transaction/*, Database*/};
use crate::service::ask;
use crate::{Clip, ServiceError, ShortCode};
use crate::web::api::ApiKey;
use std::convert::TryInto;
//use rocket::Data;

pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>, ServiceError> {
    Ok(pool.begin().await?)
}
pub async fn end_transaction(transaction: Transaction<'_>) -> Result<(), ServiceError> {
    Ok(transaction.commit().await?)
}
pub async fn increase_hit_count(
    shortcode: &ShortCode,
    hits: u32,
    pool: &DatabasePool
) -> Result<(), ServiceError> {
    Ok(query::increase_hit_count(shortcode, hits, pool).await?)
}
pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool)
-> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}
pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool)
-> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}
pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) 
-> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if clip.password == user_password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_owned()))
        }
    } else {
        Ok(clip)
    }
}

pub async fn generate_api_key(pool: &DatabasePool) -> Result<ApiKey, ServiceError> {
    let api_key = ApiKey::default();
    Ok(query::save_api_key(api_key, pool).await?)
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool)
-> Result<query::RevocationStatus, ServiceError> {
    Ok(query::revoke_api_key(api_key, pool).await?)
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool)
-> Result<bool, ServiceError> {
    Ok(query::api_key_is_valid(api_key, pool).await?)
}

pub async fn delete_expired(pool: &DatabasePool) -> Result<u64, ServiceError> {
    Ok(query::delete_expired(pool).await?)
}