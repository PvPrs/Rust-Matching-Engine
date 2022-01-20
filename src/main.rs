pub mod net;
pub mod order_book;

use std::net::SocketAddr;
use std::sync::mpsc::sync_channel;
use tokio::sync::Mutex;
use std::sync::{Arc};
use std::{thread, time};
use std::iter::from_fn;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use futures::task::Spawn;

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

    let mut matching_engine: MatchingEngine = MatchingEngine::new();
    while let order = rx.recv().await.unwrap() {
        let res = matching_engine.handle_order(&order, 0.0);
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    }
}
