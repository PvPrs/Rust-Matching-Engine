use crate::order_book::order_book::order::Order;
use crate::order_book::order_book::OrderBook;

pub mod matching_engine {
    use super::*;
    use crate::order_book::matching_engine::matching_engine::execution_report::ExecutionReport;
    use crate::order_book::order_book::order::{OrderData, OrderType};
    /*
       The Matching engine is supposed to parse the Order Book
       because the OrderBook is sorted by default, the first occurance
       will always be the best match. either in reverse(descending) traverse or ascending
       when it comes to market orders.

       for limit
    */
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
        pub fn handle_order(&mut self, order: &Order) -> ExecutionReport {
            match order {
                Order::Buy(mut buyer, mut filled) => match buyer.order_type {
                    OrderType::MARKET => {
                        for mut map in self.book.asks.clone() {
                            for (_, mut sell_order) in map.1 {
                                match sell_order {
                                    Order::Sell(mut seller, mut seller_fill) => {
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
                Order::Sell(mut seller, mut filled) => {
                    for mut map in self.book.bids.clone().iter().rev() {
                        for (_, mut buy_order) in map.1 {
                            match buy_order {
                                Order::Buy(mut buyer, mut buyer_fill) => {
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
