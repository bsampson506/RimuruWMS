use sqlx::postgres::{PgPoolOptions, PgPool, PgConnectOptions};
use std::time::Duration;
use crate::infrastructure::app_error::AppError;
use crate::infrastructure::helpers::parse_env_var;

pub async fn create_db_pool() -> Result<PgPool, AppError> {
    let max_connections = parse_env_var::<u32>("DB_MAX_CONNECTIONS")?;
    let acquire_timeout = parse_env_var::<u64>("DB_ACQUIRE_TIMEOUT")?;
    let options = database_configuration().await?;

    let database = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .connect_with(options)
        .await?;

    Ok(database)
}

async fn database_configuration() -> Result<PgConnectOptions, AppError> {
    let config = DatabaseConfiguration::from_environment()?;
    Ok(PgConnectOptions::new()
        .host(config.host.as_str())
        .port(config.port)
        .database(config.database.as_str())
        .username(config.username.as_str())
        .password(config.password.as_str()))
}

#[derive(Debug)]
struct DatabaseConfiguration {
    host: String,
    port: u16, //PGConnectOptions expects an u16, future-proofing I guess.
    database: String,
    username: String,
    password: String,
}

impl DatabaseConfiguration {
    pub fn from_environment() -> Result<DatabaseConfiguration, AppError> {
        let validated_host = parse_env_var::<String>("DB_HOST")?;
        let validated_port = parse_env_var::<u16>("DB_PORT")?;
        let validated_database = parse_env_var::<String>("DB_DATABASE")?;
        let validated_username = parse_env_var::<String>("DB_USERNAME")?;
        let validated_password = parse_env_var::<String>("DB_PASSWORD")?;
        Ok(DatabaseConfiguration {
            host: validated_host,
            port: validated_port,
            database: validated_database,
            username: validated_username,
            password: validated_password,
        })
    }
}





