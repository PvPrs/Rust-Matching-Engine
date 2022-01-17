pub mod member;
pub mod order_book;
mod net;

use std::net::SocketAddr;
use rand::{Rng, thread_rng};

use crate::member::Participant;
use crate::net::net::http_connect;
use crate::order_book::order_book::order::{Order, OrderData, OrderType};
use crate::order_book::order_book::PriceLevel;
use crate::order_book::matching_engine::matching_engine::MatchingEngine;

#[tokio::main]
async fn main() {
    // let mut matching_engine: MatchingEngine = MatchingEngine::new();
    // let mut participant: Participant = Participant::new(1);
    // let mut order_id = 1;
    // let mut rng = thread_rng();
     http_connect(SocketAddr::from(([127, 0, 0, 1], 43594))).await;

    // loop {
    //     let mut price: f64 =  rng.gen_range(0 as f64, 1000 as f64);
    //     let mut order_size: f64 =  rng.gen_range(0 as f64, 1000 as f64);
    //     let mut order: OrderData = OrderData {
    //         id: order_id,
    //         prev_id: 1,
    //         price_level: PriceLevel::new(price),
    //         size: order_size,
    //         order_type: OrderType::LIMIT,
    //     };
    //     let mut price: f64 =  rng.gen_range(0 as f64, 1000 as f64);
    //     let mut order_size: f64 =  rng.gen_range(0 as f64, 1000 as f64);
    //     let res = matching_engine.handle_order(&Order::Buy(order, 800.0));
    //     order_id += 1;
    //     let mut order2: OrderData = OrderData {
    //         id: order_id,
    //         prev_id: 0,
    //         price_level: PriceLevel::new(price),
    //         size: order_size,
    //         order_type: OrderType::MARKET,
    //     };
    //     let res_2 = matching_engine.handle_order(&Order::Sell(order2,800.00));
    //
    //             println!("{}", format!("{:?}\n\n{:?}\n",
    //             res, res_2));
    //
    // }
}
