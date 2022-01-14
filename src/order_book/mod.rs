mod matching;

use std::collections::BTreeMap;
use std::time::SystemTime;

pub mod order_book {
    use super::*;
    use order::{Order, OrderAction};
    use std::rc::Rc;

    /**
     * PriceLevel represents the integral and fractional parts of a price.
     */
    #[derive(Copy, Clone, Ord, PartialOrd, Debug, Hash, Eq, PartialEq)]
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
        pub bids: BTreeMap<PriceLevel, BTreeMap<SystemTime, Order>>,
        pub asks: BTreeMap<PriceLevel, BTreeMap<SystemTime, Order>>,
    }

    impl OrderBook {
        pub fn new() -> OrderBook {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, position: Order) -> Order {
            if matches!(position.order_action, OrderAction::BUY) {
                self.bids
                    .entry(position.price_level)
                    .or_insert_with(BTreeMap::new)
                    .insert(position.timestamp, position.to_owned());
                position
            } else {
                self.asks
                    .entry(position.price_level)
                    .or_insert_with(BTreeMap::new)
                    .insert(position.timestamp, position.to_owned());
                position
            }
        }

        pub fn cancel_order(&mut self, order: Order) {
            if !matches!(order.order_action, OrderAction::BUY) {
                self.bids
                    .get(&order.price_level)
                    .expect("Order not found.")
                    .clone()
                    .remove(&order.timestamp);
            } else {
                self.asks
                    .get(&order.price_level)
                    .expect("Order not found.")
                    .clone()
                    .remove(&order.timestamp);
            }
        }


        pub fn update_order(&mut self, order: Order, price: Option<PriceLevel>, size: Option<f64>) {
        }
    }

    /*
     The Order represents a single component of the OrderBook
     The OrderBook stores "Orders" based on Price_levels
     So a Canecelation Order must state the Canceled Orders Value.
    */

    pub mod order {
        use crate::order_book::order_book::PriceLevel;
        use std::time::SystemTime;

        #[derive(Debug, Copy, Clone)]
        pub enum OrderType {
            MARKET,
            LIMIT,
        }

        #[derive(Debug, Copy, Clone)]
        pub enum OrderAction {
            BUY,
            SELL,
            UPDATE,
            CANCEL,
        }
        #[derive(Copy, Debug, Clone)]
        pub struct Order {
            pub timestamp: SystemTime,
            pub price_level: PriceLevel,
            pub order_size: f64,
            pub order_action: OrderAction,
            pub order_type: OrderType,
        }

        impl Order {
            pub fn new(
                price_level: PriceLevel,
                order_size: f64,
                order_type: OrderType,
                action_type: OrderAction,
            ) -> Order {
                Order {
                    price_level,
                    order_size,
                    order_action: action_type,
                    order_type,
                    timestamp: SystemTime::now(),
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::order_book::order_book::OrderBook;
//     use super::*;
//     #[test]
//     fn test_add_order() {
//         order: OrderBook.
//         let res = order_book::OrderBook::add_order()
//     }
// }
