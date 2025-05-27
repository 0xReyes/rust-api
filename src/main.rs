mod exchanges;
use exchanges::*;
use axum::{extract::State, routing::get, Router, Json, response::IntoResponse};
use axum::http::StatusCode;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;


#[derive(Serialize)]
struct OrderBook {
    binance: Tick,
    mexc: Tick,
    lbank: Tick,
    coinbase: Tick,
}

#[derive(Clone)]
struct AppState {
    client: Client,
    exchanges: Arc<Vec<Box<dyn Exchange>>>,
}

async fn get_order_book_handler(State(state): State<AppState>) -> impl IntoResponse {
    match get_order_book(state).await {
        Ok(json) => json.into_response(),
        Err((status, msg)) => (status, msg).into_response(),
    }
}

async fn get_order_book(state: AppState) -> Result<Json<OrderBook>, (StatusCode, String)> {
    let futures = state.exchanges.iter().map(|ex| {
        let client = &state.client;
        async move {
            let res = client.get(ex.url()).send().await;
            let text = match res {
                Ok(resp) => resp.text().await.map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            }?;
            let tick = ex.parse(&text)?;
            Ok::<(_, _), String>((ex.name(), tick))
        }
    });

    let results = join_all(futures).await;
    let mut map = HashMap::new();

    for res in results {
        if let Ok((name, tick)) = res {
            map.insert(name, tick);
        }
    }

    Ok(Json(OrderBook {
        binance: map.remove("binance").unwrap_or_default(),
        mexc: map.remove("mexc").unwrap_or_default(),
        lbank: map.remove("lbank").unwrap_or_default(),
        coinbase: map.remove("coinbase").unwrap_or_default(),
    }))
}

#[tokio::main]
async fn main() {
    let exchanges: Vec<Box<dyn Exchange>> = vec![
        Box::new(Binance),
        Box::new(MEXC),
        Box::new(Lbank),
        Box::new(Coinbase),
    ];

    let state = AppState {
        client: Client::new(),
        exchanges: Arc::new(exchanges),
    };

    let app = Router::new()
        .route("/orderbook", get(get_order_book_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
