pub mod member;
pub mod net;
pub mod order_book;

use std::net::SocketAddr;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::net::net::listen_serve;
use crate::order_book::matching_engine::matching_engine::MatchingEngine;
use crate::order_book::order_book::order::{Order, OrderData, OrderType};
use crate::order_book::order_book::PriceLevel;

#[tokio::main]
async fn main() {

    let order_data = Arc::new(Mutex::new(OrderData {
        id: 0,
        prev_id: 0,
        price_level: PriceLevel::new(0.0),
        qty: 0.0,
        order_type: OrderType::MARKET,
    }));

    let data = Arc::clone(&order_data);
    thread::spawn(|| async move {
        listen_serve(SocketAddr::from(([127, 0, 0, 1], 43594)), Arc::clone(&data)).await;
    });

    let mut matching_engine: MatchingEngine = MatchingEngine::new();
    loop {
        let order = *Arc::clone(&order_data).lock().unwrap();
        let res = matching_engine.handle_order(&Order::Sell(order.clone(), 0.0));
        println!("{}", format!("{:?}\n\n", res));
    }
}
