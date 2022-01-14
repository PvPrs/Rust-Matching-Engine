use std::rc::Rc;
use crate::order_book::order_book::order::Order;

#[derive(Debug, Clone)]
pub struct Participant {
    id: u32,
    pub orders: Vec<Rc<Order>>,

}

impl Participant {
    pub fn new(id: u32) -> Participant{
        Participant { id, orders: vec![] }
    }
}
