use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::OrderBook;
use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport;
use crate::order_book::order_book::order::{OrderData, OrderType};

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

        /**
        Turn this into a Generic function, Same functionality used multiple times.
        */
        pub fn handle_order(&mut self, order: &Order, x: f64) -> ExecutionReport {
            match order {
                Order::Buy { order: mut buyer, filled: mut filled} => match buyer.order_type {
                    OrderType::MARKET => {
                        for mut map in self.book.asks.clone() {
                            for (_, mut sell_order) in map.1 {
                                match sell_order {
                                    Order::Sell { order: mut seller, filled: mut seller_fill} => {
                                        if buyer.qty < seller.qty {
                                            seller_fill = buyer.qty;
                                            return self.book.cancel_order(order.clone(), true);
                                        } else {
                                            filled = seller.qty;
                                            return self
                                                .book
                                                .cancel_order(sell_order.clone(), true);
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
                                    if seller.qty > buyer.qty {
                                        filled = buyer.qty;
                                        return self.book.cancel_order(buy_order.clone(), true);
                                    } else {
                                        buyer_fill = seller.qty;
                                        return self.book.cancel_order(order.clone(), true);
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
            ExecutionReport::CancelOrder("No market orders available.".to_string(), order.clone())
        }
    }

    pub mod execution_report {
        use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport::OrderUpdate;
        use crate::order_book::order_book::order::Order;

        #[derive(Debug)]
        pub enum ExecutionReport {
            PartialFill(String, Order),
            Filled(String, Order),
            CancelOrder(String, Order),
            OrderUpdate(String, Order),
            NotFound(String, Order),
        }

        impl ExecutionReport {}
    }
}



// Test Module
