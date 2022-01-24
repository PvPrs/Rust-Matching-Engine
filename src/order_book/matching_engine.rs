use crate::order_book::matching_engine::matching_engine::execution_report::Events;
use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::order::{OrderData, OrderType};
use crate::order_book::order_book::OrderBook;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;

pub mod matching_engine {
    use futures::future::Either;
    use super::*;
    use crate::order_book::matching_engine::execution_report::Events::Filled;

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
        pub fn match_order(&mut self, incoming_order: &Order) -> Events {
            match incoming_order {
                // Market Buy Order handling, Looks for match in asks.
                Order::Buy { order, filled } |
                Order::Sell { order, filled } => {
                    let book_side = if incoming_order == (Order::Buy { }) {
                        Either::Left(self.book.asks.clone())
                    } else {
                        Either::Right(self.book.bids.clone().iter().rev())
                    };
                    match order.order_type {
                        OrderType::MARKET => {
                            for price_levels in book_side {
                                for (_, mut other) in price_levels.1 {
                                    return self.is_match(incoming_order, &other);
                                }
                            }
                        }
                        OrderType::LIMIT => {
                            let mut result = Events::NotFound(*incoming_order);
                            match if incoming_order == Order::Buy {
                                self.book.asks.get(&order.price_level)
                            } else {
                                self.book.bids.get(&order.price_level) }
                            {
                                None => self.book.add_order(incoming_order.clone()),
                                Some(res) => res.iter().for_each(|(participant, other)| {
                                    result = self.is_match(incoming_order, other);
                                }),
                            }
                            self.book.add_order(incoming_order.clone());
                            return result;
                        }
                        _ => (),
                    }
            },
                Order::Cancel(data, ..) => return self.book.cancel_order(incoming_order.clone(), false),
                Order::Update(data, ..) => return self.book.update_order(incoming_order.clone()),
                _ => (),
            }
            Events::NotFound(*incoming_order)
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
