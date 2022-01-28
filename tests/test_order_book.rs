extern crate matching_engine;

pub use matching_engine::order_book::{Order};

#[cfg(test)]
mod tests {
	use matching_engine::order_book::order_book::order::{Order, OrderData, OrderType};
	use matching_engine::order_book::order_book::{Buy, Book, PriceLevel};
	use crate::order_book::order_book::{OrderBook, PriceLevel};

	#[test]
	fn test_add_buy_order() {
		let mut book: Book = Book::new();
		let mut price = PriceLevel::new(6500.0);
		let mut order: Order = Order::Buy {
			order: OrderData {
				id: 1,
				prev_id: 0,
				price_level: price,
				qty: 2500.0,
				order_type: OrderType::MARKET
			},
			filled: 0.0
		};
		let _events = book.add_order(order);

		assert_eq!(book.order_book.get(&Buy { price_level: price }), order);
	}

	#[test]
	fn remove_order() {

	}

	#[test]
	fn modify_order() {

	}
}