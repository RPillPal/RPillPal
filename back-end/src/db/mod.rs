use mongodb::options::{ClientOptions, Credential, ServerAddress};

use crate::parse_required_env_var;

pub mod user;

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: mongodb::Client,

    pub patients_db: mongodb::Database,

    pub patients_collection: mongodb::Collection<user::User>,
}

impl MongoDB {
    pub async fn new(host: &str, port: u16) -> anyhow::Result<Self> {
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

        let uri = "mongodb+srv://hackathon:rpi123@cluster0.tgssxvw.mongodb.net";
        let uri = format!("mongodb+srv://{username}:{password}@cluster0.tgssxvw.mongodb.net");
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
}
