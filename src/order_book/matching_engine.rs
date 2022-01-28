use crate::order_book::matching_engine::matching_engine::execution_report::Events;
use crate::order_book::order_book::order::{Order, OrderData, OrderType};
use crate::order_book::order_book::{Book, PriceLevel};

use futures::future::Either;
use std::collections::btree_map::Iter;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod matching_engine {
    use futures::future::Either::{Left, Right};
    use super::*;


    /// 'Matching Engine' the content the matching engine should be parsing.
    pub struct MatchingEngine {
        /// The matching engine parses a 'OrderBook', representing a book of orders(Buy & Ask)
        pub bids: Book,
        pub asks: Book
    }

    impl MatchingEngine {
        pub fn new() -> MatchingEngine {
            MatchingEngine {
                bids: Book::new(),
                asks: Book::new()
            }
        }

        /// 'match_order' represents the gateway of every 'incoming_order'
        /// every 'incoming_order' is identified by its type of 'Order'
        /// and handled in a recursive manner. Every cycle returns a 'Event' resulting in a complete
        /// 'Execution_Report'.
        pub fn match_order(&mut self, incoming_order: &Order) -> Events {
             let side = if let Order::Buy { .. } = incoming_order { &self.bids } else { &self.asks };
            match incoming_order {
                // Market Buy Order handling, Looks for match in asks.
                Order::Buy { order, filled } | Order::Sell { order, filled } => {
                    match order.order_type {
                        OrderType::MARKET | OrderType::LIMIT => {
                            for price_levels in side {
                                if order.order_type == OrderType::LIMIT && price_levels.0 > &order.price_level {
                                    return self.bids.add_order(incoming_order.clone());
                                }
                                for (_, mut other) in price_levels.1 {
                                    return self.is_match(incoming_order, &other);
                                }
                            }
                        }
                        _ => (),
                    }
                }
                Order::Cancel(..) => return self.bids.cancel_order(incoming_order.clone(), false),
                Order::Update(..) => return self.bids.update_order(incoming_order.clone()),
                _ => (),
            }
            Events::NotFound(*incoming_order)
        }

        // Checks if a order represents a match with an opposing order
        // @Return -> boolean to allow executor/caller to add to list of events.
        pub fn is_match(&mut self, order: &Order, other: &Order) -> Events {
            let (&mut data, &mut filled) = match order {
                Order::Buy { order: mut order_data, filled, } |
                Order::Sell { order: mut order_data, filled, } => (order_data, filled),
                _ => (Order::None),
            };

            let (&mut other_data, &mut other_filled) = match other {
                Order::Buy { order: mut order_data, filled } |
                Order::Sell { order: mut order_data, filled } => (order_data, filled),
                _ => Order::None,
            };

            if filled.gt(*other_filled) {
                *filled = other_data.qty;
                self.bids.cancel_order(other.clone(), true);
                return Events::Filled(*other, other_data);
            } else if data.qty < other_data.qty {
                *other_filled = data.qty;
                self.bids.cancel_order(order.clone(), true)
            } else if data.qty == other_data.qty {
                self.bids.cancel_order(order.clone(), true);
                self.bids.cancel_order(other.clone(), true)
            }
        }
    }

    /// Represents the Execution Report as response on every order action
    /// The module is initialized through enumerating constructors representing "Order Updates"
    /// also considered to be events, every event is part of a entire 'Execution Report'.
    pub mod execution_report {
        use super::*;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum Events {
            New(Order),
            PartialFill(Order),
            Filled(Order),
            CancelOrder(Order),
            OrderUpdate(Order),
            NotFound(Order),
        }
    }
}

// Test Module
