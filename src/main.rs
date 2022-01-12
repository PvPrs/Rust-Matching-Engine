pub mod order_book;

use std::io::{Read, stdin};
use order_book::order_book::{OrderBook};

fn main() {
	let mut book: OrderBook = OrderBook::new();
	let mut buffer = String::new();

	while 1 {
		print
		stdin().read_line()
	}
}