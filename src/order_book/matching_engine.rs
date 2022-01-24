use crate::order_book::matching_engine::matching_engine::execution_report::Events;
use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::order::{OrderData, OrderType};
use crate::order_book::order_book::OrderBook;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

pub mod matching_engine {
    use super::*;
    use crate::order_book::matching_engine::execution_report::Events::Filled;
    use crate::order_book::matching_engine::matching_engine::execution_report::ReportData;

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
        pub fn match_order(&mut self, order: &Order) -> Events {
            // todo!("Include a vector to pass to is_match to add opposite side matches
            // instead of returning, return later on with a single\")
            match order {
                // Market Buy Order handling, Looks for match in asks.
                Order::Buy { order: mut buyer, mut filled} => match buyer.order_type {
                    OrderType::MARKET => {
                        for mut map in self.book.asks.clone() {
                            for (_, mut other) in map.1 {
                                return self.is_match(order, &other);
                            }
                        }
                    }
                    OrderType::LIMIT => {
                        let mut result = Events::NotFound(*order);
                        match self.book.asks.get(&buyer.price_level) {
                            None => self.book.add_order(order.clone()),
                            Some(res) => res.iter().for_each(|(participant, other)| {
                                result = self.is_match(order, other);
                            }),
                        }
                        self.book.add_order(order.clone());
                        return result;
                    }
                    _ => (),
                },
                Order::Sell { order: mut seller, mut filled} => match seller.order_type {
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
                        let mut result = Events::NotFound(*other);
                        match self.book.bids.get(&seller.price_level) {
                            None => self.book.add_order(order.clone()),
                            Some(res) => res.iter().for_each(|(participant, other)| {
                                result = self.is_match(other, order);
                            }),
                        }
                        self.book.add_order(order.clone());
                        return result;
                    }
                    _ => {}
                },
                Order::Cancel(data, ..) => return self.book.cancel_order(order.clone(), false),
                Order::Update(data, ..) => return self.book.update_order(order.clone()),
                _ => (),
            }
            Events::NotFound(*order)
        }

        // Checks if a order represents a match with an opposing order
        // @Return -> boolean to allow executor/caller to add to list of events.
        pub fn is_match(&mut self, order: &Order, other: &Order) -> Events {
            let (&mut data, &mut filled) = match order {
                Order::Buy { order: mut order_data, filled} |
                Order::Sell { order: mut order_data, filled}
                => (order_data, filled),
                _ => (Order::None),
            };

            let (&mut other_data, &mut other_filled) = match other {
                Order::Buy { order: mut order_data, filled } |
                Order::Sell { order: mut order_data, filled}
                => (order_data, filled),
                _ => Order::None,
            };

            if *filled > *other_filled {
                *filled = other_data.qty;
                other_data
                self.book.cancel_order(other.clone(), true);
                return Filled(*other, other_data);
            } else if data.qty < other_data.qty {
                *other_filled = data.qty;
                self.book.cancel_order(order.clone(), true)
            } else if data.qty == other_data.qty {
                self.book.cancel_order(order.clone(), true);
                self.book.cancel_order(other.clone(), true)
            }
        }
    }
}

// Represents the Execution Report as response on every order action
// The module is initialized through enumerating constructors representing "Order Updates"
pub mod execution_report {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Events {
        New(Order),
        PartialFill(Order, OrderData),
        Filled(Order, OrderData),
        CancelOrder(Order),
        OrderUpdate(Order),
        NotFound(Order),
    }
}

// Test Module
