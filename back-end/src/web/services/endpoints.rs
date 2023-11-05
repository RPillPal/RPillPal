use crate::{
    db::{
        device::Device,
        user::{AddRequest, EmbeddedUser, UpdateRequest},
    },
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

#[post("/update_pills")]
pub async fn update_pills(
    db: Data<crate::db::MongoDB>,
    body: Json<AddRequest>,
) -> Result<HttpResponse, PillError> {
    db.update_user_pills(body.0).await.map_err(|e| {
        println!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().finish())
}

// Check if the device by its device_id is inside the db.current_online_devices, else
// add it to the db.current_online_devices
#[post("/get_devices")]
pub async fn post_devices(
    db: Data<crate::db::MongoDB>,
    body: Json<Device>,
) -> Result<HttpResponse, PillError> {
    {
        let mut current_online_devices = db.current_online_devices.lock().unwrap();
        if !current_online_devices.contains(&body.0) {
            current_online_devices.push(body.0);
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/get_devices")]
pub async fn get_devices(db: Data<crate::db::MongoDB>) -> Result<HttpResponse, PillError> {
    let current_online_devices = db.current_online_devices.lock().unwrap();

    Ok(HttpResponse::Ok().json(current_online_devices.clone()))
}
