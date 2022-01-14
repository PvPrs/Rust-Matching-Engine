pub mod member;
pub mod order_book;

use crate::member::Participant;
use crate::order_book::order_book::order::{Order, OrderAction, OrderType};
use crate::order_book::order_book::PriceLevel;
use order_book::order_book::OrderBook;
use std::rc::Rc;

fn main() {
    let mut book: OrderBook = OrderBook::new();
    let mut participant: Participant = Participant::new(1);
    let order: Order = Order::new(
        PriceLevel::new(5000 as f64),
        100 as f64,
        OrderType::LIMIT,
        OrderAction::BUY,
    );

    participant
        .orders
        .push(Rc::from(book.add_order(order.to_owned())
        ));

    println!(
        "{:?}",
        book.bids
            .get(&order.price_level)
            .unwrap()
            .get(&order.timestamp)
    );

}
