use std::time::Duration;
use migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(settings: crate::settings::Settings, test_pool: Option<sea_orm::DatabaseConnection>) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
            match sea_orm::Database::connect(
                sea_orm::ConnectOptions::new(db_url)
                    .max_connections(100)
                    .min_connections(5)
                    .connect_timeout(Duration::from_secs(8))
                    .acquire_timeout(Duration::from_secs(8))
                    .idle_timeout(Duration::from_secs(8))
                    .max_lifetime(Duration::from_secs(8))
                    .sqlx_logging(true)
                    .sqlx_logging_level(log::LevelFilter::Info)
                    .to_owned()
                )
                    .await
                    {
                        Ok(pool) => pool,
                        Err(e) => {
                            tracing::event!(target: "sea-orm", tracing::Level::ERROR, "Cannot establish DB connection: {:#?}", e);
                            panic!("Cannot establish DB connection")
                        }
                    }
        };

        Migrator::up(connection_pool.into_schema_manager_connection(), None).await.unwrap();

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let lisenter = std::net::TcpListener::bind(&address)?;
        let port = lisenter.local_addr().unwrap().port();
        let server = run(lisenter, connection_pool, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    lisenter: std::net::TcpListener,
    db_pool: sea_orm::DatabaseConnection,
    settings: crate::settings::Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let pool = actix_web::web::Data::new(db_pool);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(crate::routes::health_check)
            .configure(crate::routes::routes_config)
            .app_data(pool.clone())
            .wrap(actix_web::middleware::Logger::default())
    })
        .listen(lisenter)?
        .run();

    Ok(server)
}