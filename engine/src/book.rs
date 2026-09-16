use std::collections::{BTreeMap, VecDeque};
use crate::types::{Order, Price, Side};
use std::collections::btree_map::Entry;

#[derive(Default)]
struct OrderBook {
    bids: BTreeMap<Price, VecDeque<Order>>,
    asks: BTreeMap<Price, VecDeque<Order>>,
}

impl OrderBook {
    fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    fn add_resting_order(&mut self, order: Order){
        let price = order.price.expect("price is required for resting orders");
        let side = order.side;

    match side {
            Side::Buy => self.bids.entry(price).or_default().push_back(order),
            Side::Sell => self.asks.entry(price).or_default().push_back(order),
        }
    }

    fn remove_if_empty(&mut self, side: Side, price: Price) {
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        if let Entry::Occupied(entry) = book.entry(price) {
            if entry.get().is_empty() {
                entry.remove();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderType, OrderId};

    #[test]
    fn best_prices_report_correct_extremes(){
        let mut book = OrderBook::default();
        book.add_resting_order(Order::new(1, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 1));
        book.add_resting_order(Order::new(2, Side::Buy, OrderType::Limit, Some(Price(101)), 10, 2));
        book.add_resting_order(Order::new(3, Side::Sell, OrderType::Limit, Some(Price(102)), 10, 3));
        book.add_resting_order(Order::new(4, Side::Sell, OrderType::Limit, Some(Price(103)), 10, 4));

        assert_eq!(book.best_bid(), Some(Price(101)));
        assert_eq!(book.best_ask(), Some(Price(102)));
    }

    #[test]
    fn same_price_orders_keep_arrival_order(){
        let mut book = OrderBook::default();
        book.add_resting_order(Order::new(1, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 1));
        book.add_resting_order(Order::new(2, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 2));
        book.add_resting_order(Order::new(3, Side::Sell, OrderType::Limit, Some(Price(101)), 10, 3));
        
        let first_bid = book.bids.get(&Price(100)).unwrap().front().unwrap();
        let second_bid = book.bids.get(&Price(100)).unwrap().back().unwrap();
        assert_eq!(first_bid.id, OrderId(1));
        assert_eq!(second_bid.id, OrderId(2));
    }

    #[test]
    fn remove_if_empty_removes_price_level(){
        let mut book = OrderBook::default();
        book.add_resting_order(Order::new(1, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 1));
        book.bids.get_mut(&Price(100)).unwrap().pop_front();
        book.remove_if_empty(Side::Buy, Price(100));
        assert!(!book.bids.contains_key(&Price(100)));
    }

    #[test]
    fn remove_if_empty_does_not_remove_non_empty_price_level(){
        let mut book = OrderBook::default();
        book.add_resting_order(Order::new(1, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 1));
        book.add_resting_order(Order::new(2, Side::Buy, OrderType::Limit, Some(Price(100)), 10, 2));
        book.remove_if_empty(Side::Buy, Price(100));
        assert!(book.bids.contains_key(&Price(100)));
    }
}
