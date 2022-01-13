mod matching;

use crate::member::Participant;
use std::collections::BTreeMap;
use std::time::SystemTime;

pub mod order_book {
    use super::*;
    use order::{Order, OrderAction};
    use std::hash::Hash;
    /**
     * PriceLevel represents the integral and fractional parts of a price.
     */
    #[derive(Ord, PartialOrd, Debug, Hash, Eq, PartialEq)]
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
    @Asks: (best price == lowest price)
     [POP_FIRST] (market)
     [ITER PRICELEVEL] [FIRST TIMESTAMP] = MATCH (Limit)
    Price time: low till high (oldest timestamps gets priority) [POP_LAST]
    */
    pub struct OrderBook<'a> {
        bids: BTreeMap<PriceLevel, BTreeMap<SystemTime, Order<'a>>>,
        asks: BTreeMap<PriceLevel, BTreeMap<SystemTime, Order<'a>>>,
    }

    impl<'a> OrderBook<'a> {
        pub fn new() -> OrderBook<'static> {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, mut order: Order<'a>) {
            let time = SystemTime::now();
            if matches!(order.action_type, OrderAction::BUY) {
                order.owner.orders.push(
                    self.bids
                        .entry(order.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(time, order).unwrap()
                        .as_ref()
                );
            } else {
                order.owner.orders.insert(
                    0,
                    self.asks
                        .entry(*order.price_level)
                        .or_insert_with(BTreeMap::new)
                        .insert(time, **order)
                        .as_ref(),
                )
            }
        }

        // pub fn cancel_order(&mut self, order: Order) {
        //     self.asks.get(&order.price_level)
        //         .unwrap_or_else(println!("Price Level of order is non existant."))
        //         .get_key_value()
        // }

        // pub fn update_order(&mut self, order: Order) {
        //     if order.action_type == OrderAction::BUY {
        //         self.
        //     }
        // }
    }

    /*
     The Order represents a single component of the OrderBook
     The OrderBook stores "Orders" based on Price_levels
     So a Canecelation Order must state the Canceled Orders Value.
    */

    pub mod order {
        use super::Participant;
        use crate::order_book::order_book::PriceLevel;

        pub enum OrderType {
            MARKET,
            LIMIT,
        }

        pub enum OrderAction {
            BUY,
            SELL,
            UPDATE,
            CANCEL,
        }
        // #[derive(Copy, Clone)]
        pub struct Order<'a> {
            pub price_level: PriceLevel,
            pub order_size: f64,
            pub action_type: OrderAction,
            pub order_type: OrderType,
            pub owner: Participant<'a>,
        }

        impl<'a> Order<'a> {
            pub fn new(
                price_level: PriceLevel,
                order_size: f64,
                owner: Participant<'a>,
                order_type: OrderType,
                action_type: OrderAction,
            ) -> Order<'a> {
                Order {
                    price_level,
                    order_size,
                    action_type,
                    order_type,
                    owner,
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
