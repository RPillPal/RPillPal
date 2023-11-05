use crate::{
    db::{
        user::{EmbeddedUser, UpdateRequest, User},
        MongoDB,
    },
    error::PillError,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

use futures::TryStreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchRequest {
    name: Option<String>,
}

async fn fetch_user_in_db(user: Option<&str>, db: Data<MongoDB>) -> Result<Vec<User>, PillError> {
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

#[get("/fetch")]
pub async fn fetch(data: Data<crate::db::MongoDB>) -> Result<HttpResponse, PillError> {
    let user = fetch_user_in_db(None, data.clone()).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/fetch/{user}")]
pub async fn fetch_user(
    data: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    let name = info.into_inner();
    let docs = fetch_user_in_db(Some(&name), data.clone())
        .await
        .map_err(|e| {
            println!("Error fetching user: {e}");
            e
        })?;

    Ok(HttpResponse::Ok().json(docs))
}

#[get("/pill_data/{user}")]
pub async fn pill_data(
    data: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    let name = info.into_inner();
    let user = &fetch_user_in_db(Some(&name), data.clone())
        .await
        .map_err(|e| {
            println!("Error fetching user: {e}");
            e
        })?[0];

    // only send user.name, user.prescription.name, user.prescription.num_pills, and user.pin

    Ok(HttpResponse::Ok().json(EmbeddedUser::from(user)))
}

#[post("/update/{user}")]
pub async fn update(
    db: Data<crate::db::MongoDB>,
    body: Json<UpdateRequest>,
) -> Result<HttpResponse, PillError> {
    db.update_user(body.0).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().finish())
}
