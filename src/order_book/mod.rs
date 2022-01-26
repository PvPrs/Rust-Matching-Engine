pub mod matching_engine;

use crate::order_book::matching_engine::matching_engine::execution_report::Events;
use crate::order_book::order_book::order::{OrderData, OrderType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

pub mod order_book {
    use super::*;
    use crate::order_book::order_book::order::Order;
    use crate::order_book::order_book::order::Order::Sell;

    pub mod order {
        use super::*;

        /// Represents a "Order Type" to be immediately filled or placed in a Order Book
        /// An Order is the most minimal stand-alone order, interpreted as a http packet and serialized into a struct.
        /// The 4 States: Buy, Sell, Cancel & Update.
        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub enum Order {
            /// Represents a Buy Order Type, wrapping the OrderData and state of filled.
            Buy {
                order: OrderData,
                #[serde(skip_serializing)]
                filled: f64,
            },
            /// Represents a Sell Order Type, wrapping the OrderData and state of filled.
            Sell {
                order: OrderData,
                #[serde(skip_serializing)]
                filled: f64,
            },
            /// Represents a 'Update order type', An 'update' order resets the timestamp
            /// to the current time-stamp. allowing for a Cancel & New Order function call.
            Update(OrderData),
            /// Represents a Cancel Order type, referring to its previous order ID.
            /// Canceling an order will do a 'binary-tree search' over the entire 'Order Book'
            /// Looking for the given ID to be removed from the book.
            Cancel(OrderData),
            None,
        }

        /// Every 'Order' Type has a 'Order Type'
        /// Buy and Sell Orders can either have a 'Market' [Best current offer] or 'Limit' [Market with a max price]
        /// Update could only actually update, a LIMIT order.
        #[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
        pub enum OrderType {
            MARKET,
            LIMIT,
            UPDATE,
            CANCEL
        }

        /// Todo!("Check if can be removed")
        impl Display for OrderType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        /// 'OrderData' Represents the content(Data) of a single 'Order'
        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        pub struct OrderData {
            /// Represents the Order_ID every, Incrementing in an ascending order.
            pub id: u64,
            /// Represents the Prev_Order_ID for Updates & Cancels, Incrementing in an ascending order.
            #[serde(rename = "prev_id")]
            pub prev_id: u64,
            /// Represents the price level in integral and fractional parts of type 'PriceLevel'
            #[serde(rename = "price_level")]
            pub price_level: PriceLevel,
            /// represents the quantity or size of the order.
            pub qty: f64,
            /// order_type represents sort of 'order type' [OrderType]
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
    }

    /// 'Buy' orders are ordered differently in comparison to 'Sell'
    /// So we create 2 separate structs with distinguishable 'Ordering'
    #[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
    pub struct Buy { price_level: PriceLevel }

    /// 'Sell' orders are ordered in a 'descending' manner
    #[derive(Debug, PartialOrd, PartialEq, Eq)]
    pub struct Ask { price_level: PriceLevel }

    impl Ord for Ask {
        fn cmp(&self, other: &Self) -> Ordering {
            self::Ordering::reverse(Ordering::Less)
        }
    }

    /// 'OrderBook' represents the actual book storing the orders on two sides
    /// Asks and Bids, Both these maps are sorted based on their Key<'PriceLevel'>
    /// containing a sorted 'BTreeMap<ID, Order>' as its value.
    /// ID increments over time, meeting the 'Price-Time' requirements.
    #[derive(Debug)]
    pub struct OrderBook {
        pub bids: BTreeMap<Buy, BTreeMap<u64, Order>>,
        pub asks: BTreeMap<Ask, BTreeMap<u64, Order>>,
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
                    if filled == data.qty { return Events::NotFound(order); }
                    self.bids
                        .entry(Buy { price_level: data.price_level })
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    Events::New(order)
                }
                Order::Sell { order: data, filled } => {
                    if filled == data.qty { return Events::NotFound(order); }
                    self.asks
                        .entry(Ask { price_level: data.price_level })
                        .or_insert_with(BTreeMap::new)
                        .insert(data.id, order);
                    Events::OrderUpdate(order)
                }
                _ => Events::NotFound(order),
            }
        }

        pub fn cancel_order(&mut self, order: Order, filled: bool) -> Events {
            match order {
                Order::Buy { order: data, .. } => {
                    self.bids.get(&Buy { price_level: data.price_level }).unwrap().clone().remove(
                        match data.order_type {
                            OrderType::UPDATE => &data.prev_id,
                            _ => &data.id,
                        },
                    );
                    Events::Filled(order, data)
                }
                Order::Sell { order: data, .. } => {
                    if let Some(mut price_levels) = &self.asks.get(&Ask { price_level: data.price_level }) {
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

    /// Due tot the fact that Floating point numbers cause trouble
    /// we have split those up into its 'integral' and 'fractional' parts.
    /// More info about floating points:
    /// https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html
    /// https://floating-point-gui.de/
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

}

// Test Module
// use rand::{thread_rng, Rng};
