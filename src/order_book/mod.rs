use std::collections::BTreeMap;
use std::time::SystemTime;
use crate::entity::{Participant};

pub mod OrderBook {
    use crate::order_book::OrderBook::Order::OrderData;
    use super::*;
    use self::Order::PriceLevel;

    mod Order {
        use super::*;

        pub struct OrderData { owner: Participant, amount: f64, }
        impl OrderData {
            pub fn new(owner: Participant, amount: f64) -> OrderData {
                OrderData {
                    owner,
                    amount,
                }
            }
        }
    /*
    Bids: sorted from high till low (best price == highest price)
    Asks: sorted from low till high (best price == lowest price)
    Price time: low till high (oldest timestamps gets priority)
    */
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

        pub fn add_order(&mut self, price_level: f64, participant: Participant) {
            let time = SystemTime::now();
            if !self.bids.contains_key(&price_level) {
                self.bids.insert(price_level, BTreeMap::new());
            } else {
                self.bids.get
            }
        }

        pub fn rm_order() {}
    }
}
