pub mod member;
pub mod order_book;

use crate::member::Participant;
use crate::order_book::order_book::order::{Order, OrderAction, OrderType};
use crate::order_book::order_book::PriceLevel;
use core::num::fmt::Part;
use order_book::order_book::OrderBook;
use std::io::{stdin, Read};

fn main() {
    let mut book: OrderBook = OrderBook::new();
    let participant: Participant = Participant::new(1);
    let &mut order: Order = Order::new(
        PriceLevel::new(5000 as f64),
        100 as f64,
        participant,
        OrderType::LIMIT,
        OrderAction::BUY,
    );
    book.add_order(order);

    while 1 {
        // 	println("Please Enter your price.");
        // 	stdin(
        // 	stdin().read_line(&mut buffer)
    }
}
