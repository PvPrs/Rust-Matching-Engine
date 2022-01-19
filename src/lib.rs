mod member;
mod order_book;

#[cfg(test)]
mod tests {
    use crate::member::Participant;
    use crate::order_book::order_book::order::{Order, OrderAction, OrderType};
    use crate::order_book::order_book::{OrderBook, PriceLevel};
    use std::rc::Rc;

    #[test]
    fn fn_test_add_orders() {
        let mut book: OrderBook = OrderBook::new();
        let mut participant: Participant = Participant::new(1);
        let mut order: Order = Order::new(
            PriceLevel::new(500 as f64),
            100 as f64,
            OrderType::LIMIT,
            OrderAction::BUY,
        );

        participant
            .orders
            .push(Rc::from(book.add_order(order.to_owned())));

        assert_eq!(2, 2);
    }
}
