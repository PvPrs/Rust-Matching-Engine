use std::collections::HashMap;

pub struct Participant {
    id: u32,
    name: String,
    assets: HashMap<String, u64>
}