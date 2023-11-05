use crate::{
    db::user::{EmbeddedUser, UpdateRequest},
    error::PillError,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchRequest {
    name: Option<String>,
}

#[get("/fetch")]
pub async fn fetch(data: Data<crate::db::MongoDB>) -> Result<HttpResponse, PillError> {
    let user = data.fetch_user_in_db(None).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/fetch/{user}")]
pub async fn fetch_user(
    db: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    let name = info.into_inner();
    let docs = db.fetch_user_in_db(Some(&name)).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().json(docs))
}

#[get("/pill_data/{user}")]
pub async fn pill_data(
    db: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    let name = info.into_inner();
    let user = &db.fetch_user_in_db(Some(&name)).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?[0];

    Ok(HttpResponse::Ok().json(EmbeddedUser::from(user)))
}

#[post("/update")]
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
