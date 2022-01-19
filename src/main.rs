pub mod member;
pub mod net;
pub mod order_book;

use std::net::SocketAddr;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::iter::from_fn;
use std::thread::sleep;

use crate::net::net::listen_serve;
use crate::order_book::matching_engine::matching_engine::MatchingEngine;
use crate::order_book::order_book::order::{Order, OrderData, OrderType};
use crate::order_book::order_book::PriceLevel;

#[tokio::main]
async fn main() {
    let order = Arc::new(Mutex::new(Order::None));
    let order_rc = Arc::clone(&order);

    tokio::spawn(async move {
        listen_serve(SocketAddr::from(([127, 0, 0, 1], 43594)), Arc::clone(&order_rc)).await
    });

    let mut matching_engine: MatchingEngine = MatchingEngine::new();
    loop {
        let order_data = *Arc::clone(&order).lock().unwrap();
       // println("{}")
        let res = matching_engine.handle_order(&order_data, 0.0);
        println!("{}", format!("{:?}\n\n", res));
        thread::sleep(time::Duration::from_secs(5))
    }
}
