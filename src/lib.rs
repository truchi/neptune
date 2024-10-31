pub mod min_max_deque;
pub mod stats;
pub mod symbol;
pub mod window;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::RangeInclusive, sync::Arc};
use symbol::Symbol;
use tokio::sync::RwLock;

pub const KS: RangeInclusive<usize> = 1..=8;
pub const K1: usize = 10;
pub const K2: usize = 100;
pub const K3: usize = 1000;
pub const K4: usize = 10000;
pub const K5: usize = 100000;
pub const K6: usize = 1000000;
pub const K7: usize = 10000000;
pub const K8: usize = 100000000;

pub type Float = f64; // OMG: f32 precision breaks tests!!!

/// Payload for `POST /add_batch`
#[derive(Serialize, Deserialize, Debug)]
pub struct AddBatchPayload {
    pub symbol: String,
    pub values: Vec<Float>,
}

/// Query params for `GET /stats`
#[derive(Serialize, Deserialize, Debug)]
pub struct StatsQuery {
    pub symbol: String,
    pub k: usize,
}

/// Response for `GET /stats`
#[derive(Serialize, Deserialize, Debug)]
pub struct StatsResponse {
    pub min: Float,
    pub max: Float,
    pub last: Float,
    pub avg: Float,
    pub var: Float,
}

/// Server's state.
#[derive(Clone, Default, Debug)]
pub struct App {
    symbols: Arc<RwLock<HashMap<String, Arc<RwLock<Symbol>>>>>,
}

impl App {
    /// Returns `Symbol` for `symbol`. Minimal lock time.
    async fn get(&self, symbol: &str) -> Arc<RwLock<Symbol>> {
        // The entry may already exists, in which case we only need to lock a read.
        let symbols_lock = self.symbols.read().await;

        if let Some(symbol) = symbols_lock.get(symbol) {
            let symbol = Arc::clone(symbol);
            drop(symbols_lock); // Explicit `drop(lock)` is always a good thing in Rust :)

            return symbol;
        }

        drop(symbols_lock); // If you forgot this one, nothing will happen!

        // We have to lock a write...
        let mut symbols_lock = self.symbols.write().await;

        symbols_lock
            .entry(symbol.to_owned())
            .or_insert_with(Default::default);

        let symbol = Arc::clone(symbols_lock.get(symbol).unwrap());
        drop(symbols_lock); // :)

        symbol
    }

    /// `POST /add_batch` handler.
    pub async fn add_batch(State(app): State<Self>, Json(payload): Json<AddBatchPayload>) {
        let symbol = app.get(&payload.symbol).await;
        let mut symbol_lock = symbol.write().await;
        symbol_lock.add_batch(&payload.values);
        drop(symbol_lock);

        // dbg!(&app);
    }

    /// `GET /stats` handler.
    pub async fn stats(
        State(app): State<Self>,
        Query(query): Query<StatsQuery>,
    ) -> Result<Json<StatsResponse>, (StatusCode, &'static str)> {
        // dbg!(&app);

        if !KS.contains(&query.k) {
            return Err((StatusCode::IM_A_TEAPOT, "Invalid `k`"));
        }

        let symbol = app.get(&query.symbol).await;
        let symbol_lock = symbol.read().await;
        let stats = symbol_lock.stats(query.k);
        drop(symbol_lock);

        let Some([min, max, last, avg, var]) = stats else {
            return Err((StatusCode::IM_A_TEAPOT, "No values for this symbol"));
        };

        Ok(Json(StatsResponse {
            min,
            max,
            last,
            avg,
            var,
        }))
    }
}
