pub mod matching_engine;

use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport;
use crate::order_book::order_book::order::{OrderData, OrderType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub mod order_book {
    use super::*;
    use crate::order_book::order_book::order::Order;

    /**
     * PriceLevel represents the integral and fractional parts of a price.
     */
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
    /*
    @Bids: (best price == highest price)
    [LAST_ENTRY] = MATCH(market)
    [ITER.rev(reverse) through PRICELEVEL] get [FIRST TIMESTAMP] = MATCH (Limit)
    ----------------------------------
    @Asks: (best price == lowest price)
    [POP_FIRST] (market)
    [ITER PRICELEVEL] [FIRST TIMESTAMP] = MATCH (Limit)
    Price time: low till high (oldest timestamps gets priority) [POP_LAST]
    */
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

        pub fn add_order(&mut self, order: Order) -> ExecutionReport {
            match order {
                Order::Buy(data, ..) => {
                    self.bids
                        .entry(data.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    ExecutionReport::OrderUpdate("Added to Order Book.".to_string(), order)
                }
                Order::Sell(data, ..) => {
                    self.asks
                        .entry(data.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    ExecutionReport::OrderUpdate("Added to Order Book.".to_string(), order)
                }
                _ => ExecutionReport::NotFound("Error adding to order book.".to_string(), order),
            }
        }

        pub fn cancel_order(&mut self, order: Order, filled: bool) -> ExecutionReport {
            match order {
                Order::Buy(data, ..) => {
                    self.bids.get(&data.price_level).unwrap().clone().remove(
                        match data.order_type {
                            OrderType::UPDATE => &data.prev_id,
                            _ => &data.id,
                        },
                    );
                    ExecutionReport::Filled(
                        "Order removed from orderbook. Filled buy order.".to_string(),
                        order,
                    )
                }
                Order::Sell(data, ..) => {
                    if let Some(mut price_levels) = &self.asks.get(&data.price_level) {
                        if let Some(id) = price_levels.clone().remove(match data.order_type {
                            OrderType::UPDATE => &data.prev_id,
                            _ => &data.id,
                        }) {
                            ExecutionReport::Filled(
                                "Order removed from orderbook. Filled sell Order".to_string(),
                                order,
                            )
                        } else {
                            ExecutionReport::CancelOrder(
                                "ERROR: Order_id Not Found.".to_string(),
                                order,
                            )
                        }
                    } else {
                        ExecutionReport::NotFound(
                            "ERROR: Price_Level not Found!".to_string(),
                            order,
                        )
                    }
                }
                _ => ExecutionReport::NotFound("Error".to_string(), order),
            }
        }

        pub fn update_order(&mut self, order: Order) -> ExecutionReport {
            match order {
                Order::Update(data) => {
                    self.cancel_order(order, false);
                    self.add_order(order);
                    ExecutionReport::OrderUpdate("Order updated.".to_string(), order)
                }
                _ => ExecutionReport::NotFound("Error".to_string(), order),
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

        #[derive(Debug, Copy, Clone)]
        pub enum Order {
            Buy(OrderData, f64),
            Sell(OrderData, f64),
            Update(OrderData),
            Cancel(OrderData),
        }
    }
}

// Test Module
// use rand::{thread_rng, Rng};
