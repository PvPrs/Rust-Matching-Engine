use std::collections::BTreeMap;
use std::time::SystemTime;
use crate::entity::{Participant};

pub mod OrderBook {
    use std::alloc::System;
    use super::*;
    use self::Order::PriceLevel;

    mod Order {
        use super::*;

        struct OrderData { owner: Participant, amount: f64, }
        impl OrderData {
            pub fn new(owner: Participant, amount: f64) -> OrderData {
                OrderData {
                    owner,
                    amount,
                }
            }
        }

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub(crate) struct PriceLevel { integral: u64, decimal: u64, }
        impl PriceLevel {
            pub fn new(value: f64) -> PriceLevel {
                PriceLevel {
                    integral: value.trunc() as u64,
                    decimal: (value.fract() * 100.0) as u64,
                }
            }
        }
    }

    /*
    Bids: sorted from high till low (best price == highest price)
    Asks: sorted from low till high (best price == lowest price)
    Price time: low till high (oldest timestamps gets priority)
    */
    pub struct OrderBook {
        bids: BTreeMap<PriceLevel, BTreeMap<SystemTime, OrderData>>,
        asks: BTreeMap<PriceLevel, BTreeMap<SystemTime, OrderData>>,
    }

    impl OrderBook {
        pub fn new() -> LimitOrderBook {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, price_level : PriceLevel) {
            let time = SystemTime::now();
            if !self.bids.contains_key(&price_level) {
                self.bids.insert(price_level, BTreeMap::new().insert(time, OrderData::new))
            }
        }

        pub fn rm_order() {}
    }
}
