pub mod matching {
    use crate::member::Participant;
    use crate::order_book::order_book::order::{Order, OrderAction, OrderType};
    use crate::order_book::order_book::OrderBook;
    use std::ops::Index;

    /*
       The Matching engine is supposed to parse the Order Book
       because the OrderBook is sorted by default, the first occurance
       will always be the best match. either in reverse(descending) traverse or ascending
       when it comes to market orders.

       for limit
    */
    pub struct Match<'a> {
        order: &'a Order,
        book: &'a OrderBook,
    }

    impl<'a> Match<'a> {
        pub fn new(mut order: &'a Order, mut book: &'a OrderBook) -> Match<'a> {
            Match { order, book }
        }

        pub fn match_market(&mut self) -> Option<(Order, Order)> {
            todo!("Merge with limit Matcher function(Same functionality with continuing iter)");

            if matches!(self.order.order_type, OrderType::MARKET) {
                if matches!(self.order.order_action, OrderAction::BUY) {
                    return Some((
                        self.order.clone(),
                        self.book.asks.iter().next().expect("Empty PriceLevel.")
                            .1.iter().next().expect("Empty Order.")
                            .1.clone())
                    )
                } else if matches!(self.order.order_action, OrderAction::SELL) {
                    return Some((
                            self.order.clone(),
                            self.book.bids.iter().rev().next().expect("Empty PriceLevel.")
                            .1.iter().next().expect("Empty Order.")
                            .1.clone())
                    )
                }
            }
            return Option::None
        }

        pub fn match_limit(&mut self) -> Option<(Order, Order)> {
                if matches!(self.order.order_action, OrderAction::BUY) {
                    return Some((
                        self.order.clone(),
                        self.book.asks.iter().next().expect("Empty PriceLevel.")
                            .1.iter().next().expect("Empty Order.")
                            .1.clone())
                    )
                } else if matches!(self.order.order_action, OrderAction::SELL) {
                    return Some((
                        self.order.clone(),
                        self.book.bids.iter().rev().next().expect("Empty PriceLevel.")
                            .1.iter().next().expect("Empty Order.")
                            .1.clone())
                    )
                }
            return Option::None
        }

        pub fn swap_assets(&mut self) {}
    }
}
