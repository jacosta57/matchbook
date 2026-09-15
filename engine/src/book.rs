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
    #[test]
    fn best_prices_report_correct_extremes(){

    }

    #[test]
    fn same_price_orders_keep_arrival_order(){
        
    }
}
