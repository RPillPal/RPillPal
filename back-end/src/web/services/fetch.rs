use crate::{
    db::{user::User, MongoDB},
    error::PillError,
};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

use futures::TryStreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchRequest {
    name: Option<String>,
}

pub async fn fetch_user(user: Option<&str>, db: Data<MongoDB>) -> Result<Vec<User>, PillError> {
    if let Some(user) = user {
        let doc = db
            .patients_collection
            .find_one(
                mongodb::bson::doc! {
                    "name": user
                },
                None,
            )
            .await
            .map_err(|e| {
                println!("{e}");
                PillError::NotImplemented
            })?;
        // .map_err(|e| PillError::MongoError(e.to_string()))?;
        doc.map_or(Ok(vec![]), |doc| Ok(vec![doc]))
    } else {
        Ok(db
            .patients_collection
            .find(None, None)
            .await
            // .map_err(|e| PillError::MongoError(e.to_string()))?
            .map_err(|e| {
                println!("{e}");
                PillError::NotImplemented
            })?
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![]))
    }
}

pub async fn fetch(
    data: Data<crate::db::MongoDB>,
    _req: HttpRequest,
    body: Json<FetchRequest>,
) -> Result<HttpResponse, PillError> {
    let docs = fetch_user(body.name.as_deref(), data.clone())
        .await
        .map_err(|e| {
            println!("Error fetching user: {e}");
            e
        })?;

    Ok(HttpResponse::Ok().json(docs))
}
