use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport;
use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::order::{OrderData, OrderType};
use crate::order_book::order_book::OrderBook;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

pub mod matching_engine {
    use crate::order_book::matching_engine::matching_engine::execution_report::ReportData;
    use super::*;

    pub struct MatchingEngine {
        pub book: OrderBook,
    }

    impl MatchingEngine {
        pub fn new() -> MatchingEngine {
            MatchingEngine {
                book: OrderBook::new(),
            }
        }

        // Match_order identifies @param: order
        // @Return -> ExecutionReport representing the events of execution for said order.
        pub fn match_order(&mut self, order: &Order, x: f64) -> ExecutionReport {
            // todo!("Include a vector to pass to is_match to add opposite side matches
            // instead of returning, return later on with a single\")
            let mut match_events = ReportData::new();
            match match_events {
                ReportData { mut events, .. } =>
                    {
                    match order {
                        // Market Buy Order handling, Looks for match in asks.
                        Order::Buy { order: mut buyer,  mut filled, } =>
                            match buyer.order_type {
                            OrderType::MARKET => {
                                for mut map in self.book.asks.clone() {
                                    for (_, mut other) in map.1 {
                                        if self.is_match(order, &other) {
                                            events.push(other)
                                        }
                                    }
                                }
                            }
                            OrderType::LIMIT => {
                                let prices = self.book.asks.get(&buyer.price_level);
                                match prices {
                                    None => return self.book.add_order(order.clone()),
                                    Some(res) => res.iter().for_each(|(participant, other)| match other {
                                        Order::Sell { .. } => {
                                            if self.is_match(&order.clone(), other) {
                                                events.push(*other)
                                            }
                                        }
                                        _ => (),
                                    }),
                                }
                                return self.book.add_order(order.clone());
                            }
                            _ => (),
                        },
                        Order::Sell { order: mut seller, mut filled, } => {
                            match seller.order_type {
                                OrderType::MARKET => {
                                    for mut map in self.book.bids.clone().iter().rev() {
                                        for (_, mut other) in map.1 {
                                            if self.is_match(other, order) {
                                                events.push(*other)
                                            };
                                        }
                                    }
                                }
                                OrderType::LIMIT => {
                                    let prices = self.book.asks.get(&buyer.price_level);
                                    match prices {
                                        None => return self.book.add_order(order.clone()),
                                        Some(res) => res.iter().for_each(|(participant, other)| match other {
                                            Order::Sell { .. } => {
                                                if self.is_match(&order.clone(), other) {
                                                    events.push(*other)
                                                }
                                            }
                                            _ => (),
                                        }),
                                    }
                                    return self.book.add_order(order.clone());
                                }
                                _ => {}
                            }
                        }
                        Order::Cancel(data, ..) =>
                            { return self.book.cancel_order(order.clone(), false) }
                        Order::Update(data, .. ) =>
                            return self.book.update_order(order.clone()),
                        _ => (),
                    }
                }
            }
            ExecutionReport::NotFound("".to_string(), *order)
        }

        // Checks if a order represents a match with an opposing order
        // @Return -> boolean to allow executor/caller to add to list of events.
        pub fn is_match(&mut self, order: &Order, other: &Order) -> &Order {
            let (mut filled, mut other_filled) = (0.0, 0.0);
            let (mut order_qty, mut other_qty) = (0.0, 0.0);
            match order {
                Order::Buy { order: mut order_data, filled } => { order_qty = order_data.qty; }
                Order::Sell { order: mut order_data, filled } => { order_qty = order_data.qty }
                _ => (),
            }
            let x = match other {
                Order::Buy { order: mut order_data, filled } => { other_qty = order_data.qty; order_data }
                Order::Sell {order: mut order_data, filled} => { other_qty = order_data.qty; order_data }
                _ => (),
            };
            println!("{:?}", x.price_level);
            if filled > other_filled {
                filled = other_data.qty;
                return self.book.cancel_order(other.clone(), true);
            } else if data.qty < other_data.qty {
                other_fill = data.qty;
                return self.book.cancel_order(order.clone(), true);
            } else if data.qty == other_data.qty {
                self.book.cancel_order(order.clone(), true);
                return self.book.cancel_order(other.clone(), true);
            }
                _ => ExecutionReport::NotFound("lol".to_string(), order.clone()),
            }
            ExecutionReport::NotFound("Order not Found.".to_string(), buyer.clone())
        }
    }

    // Represents the Execution Report as response on every order action
    // The module is initialized through enumerating constructors representing "Order Updates"
    pub mod execution_report {
        use super::*;

        pub struct ReportData {
            events: Vec<Order>,
            time_stamp: DateTime<Utc>
        }

        impl ReportData {
            pub fn new() -> ReportData {
                ReportData {
                    events: Vec::new();
                    time_stamp: Utc::now();
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ExecutionReport {
            PartialFill(String, Order),
            Filled(String, Order),
            CancelOrder(String, Order),
            OrderUpdate(String, Order),
            NotFound(String, Order),
        }

        impl ExecutionReport {}
    }
}

// Test Module
