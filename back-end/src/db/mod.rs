use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::options::ClientOptions;
use tokio::sync::Mutex;

use crate::util::parse_required_env_var;

pub mod device;
pub mod user;

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: mongodb::Client,
    pub patients_db: mongodb::Database,
    pub patients_collection: mongodb::Collection<user::User>,
    pub current_online_devices: Arc<Mutex<Vec<device::Device>>>,
}

impl MongoDB {
    pub async fn new(host: &str) -> anyhow::Result<Self> {
        let username: String = match parse_required_env_var("DB_USERNAME") {
            Ok(username) => username,
            Err(e) => return Err(e),
        };
        let password: String = match parse_required_env_var("DB_PASSWORD") {
            Ok(password) => password,
            Err(e) => return Err(e),
        };

        let uri = format!("mongodb+srv://{username}:{password}@{host}");
        let options = ClientOptions::parse(uri).await.unwrap();
        let client = mongodb::Client::with_options(options)?;

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        tracing::info!("Connected to MongoDB!");

        Ok(Self {
            client: client.clone(),

            patients_db: client.database("patients"),
            patients_collection: client.database("patients").collection("patients"),

            current_online_devices: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn fetch_user_in_db(
        &self,
        user: Option<&str>,
    ) -> Result<Vec<user::User>, mongodb::error::Error> {
        if let Some(user) = user {
            let doc = self
                .patients_collection
                .find_one(
                    mongodb::bson::doc! {
                        "name": user
                    },
                    None,
                )
                .await?;
            doc.map_or(Ok(vec![]), |doc| Ok(vec![doc]))
        } else {
            Ok(self
                .patients_collection
                .find(None, None)
                .await?
                .try_collect()
                .await
                .unwrap_or_else(|_| vec![]))
        }
    }

    pub async fn update_user(
        &self,
        doc: user::UpdateRequest,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let filter = doc! {"name": &doc.name};
        let update = doc! {
            "$set": {
                "prescription.0.numPills": doc.num_pills,
                "prescription.0.lastTaken": doc.time_dispensed,
            },
        };
        let res = self
            .patients_collection
            .update_one(filter, update, None)
            .await?;
        tracing::info!(
            "{name} now has {num_pills} pills left and last took a dose at {time_dispensed}",
            name = doc.name,
            num_pills = doc.num_pills,
            time_dispensed = doc.time_dispensed
        );
        Ok(res)
    }

    pub async fn update_user_pills(
        &self,
        doc: user::AddRequest,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
        let filter = doc! {"name": &doc.name};
        let update = doc! {
            "$inc": {
                "prescription.0.numPills": doc.num_added,
            },
        };
        let res = self
            .patients_collection
            .update_one(filter, update, None)
            .await?;
        tracing::info!(
            "Added {num_added} pills to {name}",
            name = doc.name,
            num_added = doc.num_added
        );
        Ok(res)
    }
}
