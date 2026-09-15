#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Side {Buy, Sell}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum OrderType {Limit, Market}

#[derive(PartialOrd, Ord, Debug, Clone, PartialEq, Eq, Copy)] 
pub struct Price(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct Quantity(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct OrderId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct Sequence(u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
pub struct Order {
    id: OrderId,
    pub side: Side,
    order_type: OrderType,
    pub price: Option<Price>,
    quantity: Quantity,
    sequence: Sequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)] 
struct Trade {
    maker_order_id: OrderId,
    taker_order_id: OrderId,
    price: Price,
    quantity: Quantity,
}