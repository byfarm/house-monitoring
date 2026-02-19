use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new().route("/proxy/influxdb/freezer", get(get_temp));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("server failed to bind to port 3000");

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn get_temp() -> impl IntoResponse {
    let api_url = "http://nginx/db/api/v3/query_sql";

    let token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN not set in .env");

    let influx_query = r#"
        SELECT
            temperature AS "Temperature"
        FROM
            "home"
        WHERE
            time >= now() - INTERVAL '15 minutes'
        ORDER BY 
            time DESC
        LIMIT 1
    "#;

    let client = reqwest::Client::new();

    let response = client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("db", "default")])
        .query(&[("q", &influx_query)])
        .send()
        .await;

    match response {
        Ok(res) => {
            let body = res.text().await.unwrap_or_default();

            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/csv".parse().unwrap());
            headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

            (StatusCode::OK, headers, body)
        }
        Err(err) => {
            eprintln!("Influx request failed: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                "Error fetching data".to_string(),
            )
        }
    }
}
