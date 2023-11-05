use futures::TryStreamExt;
use mongodb::options::ClientOptions;

use crate::parse_required_env_var;

pub mod user;

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: mongodb::Client,

    pub patients_db: mongodb::Database,

    pub patients_collection: mongodb::Collection<user::User>,
}

impl MongoDB {
    pub async fn new(host: &str) -> anyhow::Result<Self> {
        // read .config into Config struct

        let username: String = match parse_required_env_var("DB_USERNAME") {
            Ok(username) => username,
            Err(e) => return Err(e),
        };
        let password: String = match parse_required_env_var("DB_PASSWORD") {
            Ok(password) => password,
            Err(e) => return Err(e),
        };

        // let mut options = mongodb::options::ClientOptions::parse_with_resolver_config(
        //     "mongodb+srv://hackathon:rpi123@cluster0.tgssxvw.mongodb.net",
        //     mongodb::options::ResolverConfig::google(),
        // )
        // .await
        // .unwrap();

        let uri = format!("mongodb+srv://{username}:{password}@{host}");
        let options = ClientOptions::parse(uri).await.unwrap();
        let client = mongodb::Client::with_options(options)?;

        // options.connect_timeout = Some(std::time::Duration::from_secs(5));

        // let options = mongodb::options::ClientOptions::builder()
        //     .hosts(vec![ServerAddress::Tcp {
        //         host: host.to_owned(),
        //         port: Some(port),
        //     }])
        //     .credential(
        //         Credential::builder()
        //             .username(username)
        //             .password(password)
        //             .build(),
        //     )
        //     .connect_timeout(Some(std::time::Duration::from_secs(5)))
        //     .build();
        //
        // let client = mongodb::Client::with_options(options)?;

        // ping the database to check if we can connect
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        println!("Connected to MongoDB!");

        Ok(Self {
            client: client.clone(),

            patients_db: client.database("patients"),
            patients_collection: client.database("patients").collection("patients"),
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
        let filter = doc! {"name": doc.name};
        let update = if let Some(time) = doc.time_dispensed {
            doc! {
                "$set": {
                    "prescription.0.numPills": doc.num_pills
                },
                "$set": {
                    "prescription.0.lastTaken": time
                }
            }
        } else {
            doc! {
                "$set": {
                    "prescription.0.numPills": doc.num_pills
                }
            }
        };
        self.patients_collection
            .update_one(filter, update, None)
            .await
    }
}
