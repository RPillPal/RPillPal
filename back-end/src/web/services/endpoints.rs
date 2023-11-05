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
#[tracing::instrument(skip(db))]
pub async fn fetch(db: Data<crate::db::MongoDB>) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    tracing::info!("Fetching all users");
    let user = db.fetch_user_in_db(None).await.map_err(|e| {
        tracing::error!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/fetch/{user}")]
#[tracing::instrument(skip(db))]
pub async fn fetch_user(
    db: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    tracing::info!("Fetching {info}'s user information'");
    let name = info.into_inner();
    let docs = db.fetch_user_in_db(Some(&name)).await.map_err(|e| {
        tracing::error!("Error fetching user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().json(docs))
}

#[get("/pill_data/{user}")]
#[tracing::instrument(skip(db))]
pub async fn pill_data(
    db: Data<crate::db::MongoDB>,
    info: Path<String>, // Extract path parameter
) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    tracing::info!("Fetching pill data for {info}");
    let name = info.into_inner();
    let users = db.fetch_user_in_db(Some(&name)).await.map_err(|e| {
        tracing::error!("Error fetching user: {e}");
        e
    })?;

    let user = users
        .first()
        .ok_or(PillError::MongoError("No user found".into()))?;

    Ok(HttpResponse::Ok().json(EmbeddedUser::from(user)))
}

#[post("/update")]
#[tracing::instrument(
    skip(db, body), fields(name = %body.name, num_pills = %body.num_pills, time_dispensed = %body.time_dispensed)
)]
pub async fn update(
    db: Data<crate::db::MongoDB>,
    body: Json<UpdateRequest>,
) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    db.update_user(body.0).await.map_err(|e| {
        tracing::error!("Error updating user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/update_pills")]
#[tracing::instrument(
    skip(db, body), fields(name = %body.name, num_added = %body.num_added)
)]
pub async fn update_pills(
    db: Data<crate::db::MongoDB>,
    body: Json<AddRequest>,
) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    db.update_user_pills(body.0).await.map_err(|e| {
        tracing::error!("Error updating user: {e}");
        e
    })?;

    Ok(HttpResponse::Ok().finish())
}

// Check if the device by its device_id is inside the db.current_online_devices, else
// add it to the db.current_online_devices
#[post("/get_devices")]
#[tracing::instrument(
    skip(db, body), fields(device_id = %body.device_id, last_heartbeat = %body.last_heartbeat)
)]
pub async fn post_devices(
    db: Data<crate::db::MongoDB>,
    body: Json<Device>,
) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    tracing::info!(
        "Updating {device_id}'s heartbeat to {last_heartbeat}",
        device_id = body.device_id,
        last_heartbeat = body.last_heartbeat
    );

    {
        let mut current_online_devices = db.current_online_devices.lock().await;
        if let Some(device) = current_online_devices
            .iter_mut()
            .find(|d| d.device_id == body.device_id)
        {
            device.last_heartbeat = body.last_heartbeat;
        } else {
            current_online_devices.push(body.0);
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/get_devices")]
#[tracing::instrument(skip(db))]
pub async fn get_devices(db: Data<crate::db::MongoDB>) -> Result<HttpResponse, PillError> {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    tracing::info!("Getting devices");
    let current_online_devices = db.current_online_devices.lock().await;

    Ok(HttpResponse::Ok().json(current_online_devices.clone()))
}

// Cleanup function that loops every 30 seconds to find dead devices
pub async fn clean_up_devices(db: Data<crate::db::MongoDB>) {
    match tracing_honeycomb::register_dist_tracing_root(tracing_honeycomb::TraceId::new(), None) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Error while registering dist tracing root: {e:?}");
        }
    }
    loop {
        let mut current_online_devices = db.current_online_devices.lock().await;
        // current unix timestamp
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut removed_devices = Vec::new();
        current_online_devices.retain(|d| {
            if current_time - (d.last_heartbeat as u64) < 60 {
                true
            } else {
                removed_devices.push(d.device_id.clone());
                false
            }
        });
        drop(current_online_devices); // Release the lock
        removed_devices
            .iter()
            .for_each(|d| tracing::info!("Device `{d}` was removed"));
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}
