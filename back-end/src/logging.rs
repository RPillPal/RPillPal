use tracing::Instrument;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[macro_export]
macro_rules! log_error {
    ($e:expr) => {
        if let Err(e) = $e {
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
    ($context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = format!("{:?}", ::anyhow::anyhow!(e).context($context));
            tracing::error!(error.message = %format!("{}", &e), "{:?}", e);
        }
    };
}

pub fn init_tracing(honeycomb_api_key: Option<String>) -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let log_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::try_new("info").unwrap())
        .add_directive("actix_web=debug".parse().unwrap())
        .add_directive("actix_server=debug".parse().unwrap())
        .add_directive("back-end=debug".parse().unwrap());

    let sub = Registry::default()
        .with(log_filter)
        .with(tracing_subscriber::fmt::Layer::default());

    if let Some(api_key) = honeycomb_api_key {
        tracing::info!("honeycomb api key is set, initializing honeycomb layer");
        let config = libhoney::Config {
            options: libhoney::client::Options {
                api_key,
                dataset: "rpillpal".to_string(),
                ..libhoney::client::Options::default()
            },
            transmission_options: libhoney::transmission::Options::default(),
        };

        let honeycomb_layer = tracing_honeycomb::Builder::new_libhoney("rpillpal", config).build();

        let sub = sub.with(honeycomb_layer);
        tracing::subscriber::set_global_default(sub)?;
    } else {
        tracing::info!("no honeycomb api key is set");
        let sub = sub.with(tracing_honeycomb::new_blackhole_telemetry_layer());
        tracing::subscriber::set_global_default(sub)?;
    }

    Ok(())
}

pub async fn send_honeycomb_deploy_marker(api_key: &str) {
    let client = reqwest::Client::new();
    log_error!(
        client
            .post("https://api.honeycomb.io/1/markers/rpillpal")
            .header("X-Honeycomb-Team", api_key)
            .body(format!(
                r#"{{"message": "{}", "type": "deploy"}}"#,
                crate::VERSION
            ))
            .send()
            .await
    );
}
pub async fn init_cpu_logging() {
    use cpu_monitor::CpuInstant;
    use std::time::Duration;

    tokio::spawn(
        async {
            loop {
                let start = CpuInstant::now();
                tokio::time::sleep(Duration::from_millis(20000)).await;
                let end = CpuInstant::now();
                if let (Ok(start), Ok(end)) = (start, end) {
                    let duration = end - start;
                    let percentage = duration.non_idle() * 100.0;
                    tracing::info!(cpu_usage = format!("{:.2}%", percentage));
                }
            }
        }
        .instrument(tracing::info_span!("cpu-usage")),
    );
}
