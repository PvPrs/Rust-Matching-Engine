use std::collections::BTreeMap;
use std::time::SystemTime;
use crate::entity::{Participant};

pub mod OrderBook {
    use super::*;
    use self::Order::PriceLevel;

    

    mod Order {
        use super::*;

        #[derive(Debug, Hash, Eq, PartialEq)]
        pub(crate) struct PriceLevel { integral: u64, decimal: u64, }

        struct OrderData { owner: Participant, amount: f64, }

        impl PriceLevel {
            pub fn new(value: f64) -> PriceLevel {
                PriceLevel {
                    integral: value.trunc() as u64,
                    decimal: (value.fract() * 100.0) as u64,
                }
            }
        }
    }

    pub struct OrderBook {
        bids: BTreeMap<PriceLevel, SortedMap<SystemTime, OrderData>>,
        asks: BTreeMap<PriceLevel, SortedMap<SystemTime, OrderData>>,
    }

    impl OrderBook {
        pub fn new() -> LimitOrderBook {
            OrderBook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            }
        }

        pub fn add_order(&mut self, price_level : PriceLevel) {
            if (self.asks)
        }

        pub fn rm_order() {}
    }
}
