use crate::order_book::order_book::order::Order;

trait Participation {
    // fn new_order() ->
}

pub struct Participant<'a> {
    id: u32,
    pub orders: Vec<Option<Order<'a>>>,
}

impl<'a> Participant<'a> {
    pub fn new(id: u32) -> Participant<'a> {
        Participant { id, orders: vec![] }
    }
}
