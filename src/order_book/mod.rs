mod matching;

use crate::entity::Participant;
use std::collections::BTreeMap;
use std::time::SystemTime;

pub mod order_book {
    use super::*;
    use order::{OrderData, OrderAction};
    /*
    Bids: sorted from high till low (best price == highest price)
    Asks: sorted from low till high (best price == lowest price)
    Price time: low till high (oldest timestamps gets priority)
    */
    #[derive(Ord)]
    pub struct OrderBook {
        bids: BTreeMap<f64, BTreeMap<SystemTime, OrderData>>,
        asks: BTreeMap<f64, BTreeMap<SystemTime, OrderData>>,
    }

    impl OrderBook {
        pub fn new() -> LimitOrderBook {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, order: OrderData) -> Option<OrderData> {
            let time = SystemTime::now();
            if order.action_type == OrderAction::BUY {
                self.bids
                    .entry(order.price_level)
                    .or_insert_with(BTreeMap::new)
                    .insert(order.time, order)
            } else {
                self.asks
                    .entry(order.price_level)
                    .or_insert_with(BTreeMap::new)
                    .insert(order.time, order)
            }
        }

        pub fn cancel_order(&mut self, order: OrderData) {
        }

        pub fn update_order(&mut self, order: OrderData) {
        }
    }

    pub(super) mod order {
        use super::Participant;

        pub(super) enum OrderType {
            MARKET,
            LIMIT,

        }
        pub(super) enum OrderAction {
            BUY,
            SELL,
            UPDATE,
            CANCEL
        }

        pub(super) struct OrderData {
            pub order_id: u64,
            pub prev_order_id: u64,
            pub price_level: f64,
            pub order_size: f64,
            pub action_type: OrderAction,
            pub order_type: OrderType,
            pub owner: Participant,
        }

        impl OrderData {
            pub fn new(owner: Participant, amount: f64) -> OrderData {
                OrderData {
                    price_level,
                    order_size,
                    action_type,
                    order_type,
                    order_id,
                    prev_order_id,
                    owner,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_order() {
        let res = order_book::OrderBook::add_order()
    }
}
