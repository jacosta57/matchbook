#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Side {Buy, Sell}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum OrderType {Limit, Market}

#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Copy)] 
struct Price(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Quantity(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct OrderId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Sequence(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct Order {
    id: OrderId,
    side: Side,
    order_type: OrderType,
    pub price: Option<Price>,
    quantity: Quantity,
    sequence: Sequence,
}

impl Order {
    pub fn new(id: u64, side: Side, order_type: OrderType, price: Option<Price>, quantity: u64, sequence: u64) -> Self {
        Order {
            id: OrderId(id),
            side,
            order_type,
            price,
            quantity: Quantity(quantity),
            sequence: Sequence(sequence),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Trade {
    maker_order_id: OrderId,
    taker_order_id: OrderId,
    price: Price,
    quantity: Quantity,
}
