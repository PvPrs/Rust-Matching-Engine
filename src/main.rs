pub mod net;
pub mod order_book;

use futures::task::Spawn;
use std::iter::from_fn;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{thread, time};
use tokio::sync::Mutex;

use crate::net::net::listen_serve;
use crate::order_book::matching_engine::matching_engine::MatchingEngine;
use crate::order_book::order_book::order::{Order, OrderData, OrderType};
use crate::order_book::order_book::PriceLevel;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        listen_serve(SocketAddr::from(([127, 0, 0, 1], 43594)), tx.clone()).await
    });

    let mut execution_events: Vec<Event>;
    let mut matching_engine: MatchingEngine = MatchingEngine::new();

    while let incoming_order = rx.recv().await.unwrap() {
        match incoming_order {
            Order::Buy { order, .. } | Order::Sell { order, ..} =>
                execution_events.push(matching_engine.match_order(&incoming_order, 0.0)),;
            _ => { matching_engine.match_order(&incoming_order, 0.0) }
        }
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }
}
