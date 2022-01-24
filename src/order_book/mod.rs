pub mod matching_engine;

use crate::order_book::matching_engine::matching_engine::execution_report::Events;
use crate::order_book::order_book::order::{OrderData, OrderType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub mod order_book {
    use super::*;
    use crate::order_book::order_book::order::Order;

    #[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Debug, Hash, Eq, PartialEq)]
    pub struct PriceLevel {
        integral: u64,
        decimal: u64,
    }

    impl PriceLevel {
        pub fn new(value: f64) -> PriceLevel {
            PriceLevel {
                integral: value.trunc() as u64,
                decimal: (value.fract() * 100.0) as u64,
            }
        }
    }

    impl Display for PriceLevel {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}.{}", self.integral, self.decimal)
        }
    }

    // Represents a order book sorted by price-time
    // u64 = orderID doubling as time identifier(increased in time).
    #[derive(Debug)]
    pub struct OrderBook {
        pub bids: BTreeMap<PriceLevel, BTreeMap<u64, Order>>,
        pub asks: BTreeMap<PriceLevel, BTreeMap<u64, Order>>,
    }

    impl OrderBook {
        pub fn new() -> OrderBook {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, order: Order) -> Events {
            match order {
                Order::Buy { order: data, filled } => {
                    if filled == data.qty { Events::NotFound(order) }
                    self.bids
                        .entry(data.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    Events::New(order)
                }
                Order::Sell { order: data, filled } => {
                    if filled == data.qty { Events::NotFound(order) }
                    self.asks
                        .entry(data.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    Events::OrderUpdate(order)
                }
                _ => Events::NotFound(order),
            }
        }

        // Cancels order throughout multiple scenario's
        // Order removal on:
        // - Fill               - No orders found[Market]
        // - Cancel / Update
        pub fn cancel_order(&mut self, order: Order, filled: bool) -> Events {
            match order {
                Order::Buy { order: data, .. } => {
                    self.bids.get(&data.price_level).unwrap().clone().remove(
                        match data.order_type {
                            OrderType::UPDATE => &data.prev_id,
                            _ => &data.id,
                        },
                    );
                    Events::Filled(order, data)
                }
                Order::Sell { order: data, .. } => {
                    if let Some(mut price_levels) = &self.asks.get(&data.price_level) {
                        if let Some(id) = price_levels.clone().remove(match data.order_type {
                            OrderType::UPDATE => &data.prev_id,
                            _ => &data.id,
                        }) {
                            Events::Filled(order, data)
                        } else {
                            Events::CancelOrder(order)
                        }
                    } else {
                        Events::NotFound(order)
                    }
                }
                _ => Events::NotFound(order),
            }
        }

        // Price-Time is reset during update
        // Canceling & re-adding said orders.
        pub fn update_order(&mut self, order: Order) -> Events {
            match order {
                Order::Update(data) => {
                    self.cancel_order(order, false);
                    self.add_order(order);
                    Events::OrderUpdate(order)
                }
                _ => Events::NotFound(order),
            }
        }
    }

    pub mod order {
        use super::*;

        pub enum State {
            Partial,
        }

        #[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
        pub enum OrderType {
            MARKET,
            LIMIT,
            UPDATE,
            CANCEL,
        }

        impl Display for OrderType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub struct OrderData {
            pub id: u64,
            #[serde(rename = "prev_id")]
            pub prev_id: u64,
            #[serde(rename = "price_level")]
            pub price_level: PriceLevel,
            pub qty: f64,
            #[serde(rename = "order_type")]
            pub order_type: OrderType,
        }

        impl Display for OrderData {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}\n{}\n{}\n{}\n{}",
                    self.id, self.prev_id, self.price_level, self.qty, self.order_type
                )
            }
        }

        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub enum Order {
            Buy {
                order: OrderData,
                #[serde(skip_serializing)]
                filled: f64,
            },
            Sell {
                order: OrderData,
                #[serde(skip_serializing)]
                filled: f64,
            },
            Update(OrderData),
            Cancel(OrderData),
            None,
        }
    }
}

// Test Module
// use rand::{thread_rng, Rng};
