use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::OrderBook;
use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport;
use crate::order_book::order_book::order::{OrderData, OrderType};
use chrono::

pub mod matching_engine {
    use super::*;

    pub struct MatchingEngine {
        pub book: OrderBook,
    }

    impl MatchingEngine {
        pub fn new() -> MatchingEngine {
            MatchingEngine {
                book: OrderBook::new(),
            }
        }

        // Turn this into a Generic function
        // Same functionality used multiple times <Buy, Sell>
        pub fn handle_order(&mut self, order: &Order, x: f64) -> ExecutionReport {
            match order {
                Order::Buy { order: mut buyer, filled: mut filled} =>
                    match buyer.order_type {
                    OrderType::MARKET => {
                        for mut map in self.book.asks.clone() {
                            for (_, mut sell_order) in map.1 {
                                match sell_order {
                                    Order::Sell { order: mut seller, filled: mut seller_fill} => {
                                        return if buyer.qty < seller.qty {
                                            seller_fill = buyer.qty;
                                            self.book.cancel_order(order.clone(), true)
                                        } else {
                                            filled = seller.qty;
                                            self.book.cancel_order(sell_order.clone(), true)
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    OrderType::LIMIT => {
                        // self.book.asks.get(orde)
                        return self.book.add_order(order.clone());
                    }
                    _ => (),
                },
                Order::Sell { order: mut seller, filled: mut filled } => {
                    for mut map in self.book.bids.clone().iter().rev() {
                        for (_, mut buy_order) in map.1 {
                            match buy_order {
                                Order::Buy { order: mut buyer, filled: mut buyer_fill } => {
                                    return if seller.qty > buyer.qty {
                                        filled = buyer.qty;
                                        self.book.cancel_order(buy_order.clone(), true)
                                    } else {
                                        buyer_fill = seller.qty;
                                        self.book.cancel_order(order.clone(), true)
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
                // if seller.order_type == OrderType::LIMIT {
                //     seller.size -= filled;
                //     self.book.add_order(order);
                Order::Cancel(data, ..) => {
                    return self.book.cancel_order(order.clone(), false);
                }
                // Order::Update(data, .. ) => self.book.update_order(order.clone())
                _ => (),
            }
            ExecutionReport::NotFound()
        }
    }

    pub mod execution_report {
        use chrono::{DateTime, Utc};
        use crate::order_book::order_book::order::Order;
        use serde::{Deserialize, Serialize};

        pub struct ReportData {
            matches: Vec<Order>,
            time_stamp: DateTime<Utc>
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ExecutionReport {
            
            PartialFill(ReportData),
            Filled(ReportData),
            CancelOrder(ReportData),
            OrderUpdate(ReportData),
            NotFound(),
        }

        impl ExecutionReport {}
    }
}



// Test Module
