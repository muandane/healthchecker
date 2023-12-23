use std::env;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service_address = env::var("SERVICE_ADDRESS").unwrap_or_else(|_| String::from("localhost"));
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| String::from("8000"));
    let health_check_path = env::var("HEALTH_CHECK_PATH").unwrap_or_else(|_| String::from("health"));

    let url = format!("http://{}:{}/{}", service_address, service_port, health_check_path);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        println!("Service is up!");
    } else {
        println!("Service is down!");
    }

    Ok(())
}
