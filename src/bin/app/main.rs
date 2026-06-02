mod routes;
mod state;

use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tracing::{Level, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load `.env` file. It's ok if it doesn't exist.
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    // Get the port number from the environment, default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Get the port as a string or default to "3000"
        .parse() // Parse the port string into a u16
        .expect("Failed to parse PORT");

    // Run our app with hyper, listening globally on port 8080
    // NOTE: Must listen to 0.0.0.0 to use in docker container
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!("Server running on {address}...");

    // Get the database url
    let database_url = env::var("DATABASE_URL")?;
    let db_conn = drp_backend::db::establish_connection(database_url)?;

    let app_state = state::AppState::new(db_conn);

    axum::serve(listener, routes::routes(app_state)).await?;

    Ok(())
}
