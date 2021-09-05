use std::io;
use std::io::prelude::*;

mod build;
use crate::build::structs::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut ob = OrderBook::new();
    for line in stdin.lock().lines() {
       if let Ok(o) = line?.to_string().parse::<Order>() {
           ob.submit(o);
           ob.trade_wrapper();
       }
       else {
            println!("Order should be in the format:  <id>. <OrderType> <Quantity> BTC @ <price> USD")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_format() {
        let s = "1. Buy 50 BTC @ 1000 USD";
        assert!(!s.parse::<Order>().is_err());
        let s = "1000. Sell 50 BTC @ 1000 USD";
        assert!(!s.parse::<Order>().is_err());

        let s = "1x. Buy 50 BTC @ 1000 USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. By 50 BTC @ 1000 USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. Buy -2 BTC @ 1000 USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. Buy 50 XRP @ 1000 USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. Buy 50 BTC > 1000 USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. Buy 50 BTC @ FOUR USD";
        assert!(s.parse::<Order>().is_err());
        let s = "1. Buy 50 BTC @ 1000 GBP";
        assert!(s.parse::<Order>().is_err());
    }

    #[test]
    fn min_sells() {
        let mut ob = OrderBook::new();
        let o = Order {id: 1, order_type: OrderType::Sell, price: 10, quantity: 100};
        ob.submit(o.clone());

        assert!(ob.buys.is_empty());
        assert!(!ob.sells.is_empty());

        assert_eq!(Some(&o),ob.sells.peek());

        let p = Order {id: 2, order_type: OrderType::Sell, price: 5, quantity: 100};
        ob.submit(p.clone());
        assert_eq!(Some(&p),ob.sells.peek());
        let q = Order {id: 3, order_type: OrderType::Sell, price: 20, quantity: 100};
        ob.submit(q.clone());
        assert_eq!(Some(&p),ob.sells.peek());
        ob.sells.pop();
        assert_eq!(Some(&o),ob.sells.peek());
        ob.sells.pop();
        assert_eq!(Some(&q),ob.sells.peek());
        assert!(ob.buys.is_empty());
        assert!(!ob.sells.is_empty());
    }
    #[test]
    fn max_buys() {
        let mut ob = OrderBook::new();
        let o = Order {id: 1, order_type: OrderType::Buy, price: 10, quantity: 100};
        ob.submit(o.clone());

        assert!(!ob.buys.is_empty());
        assert!(ob.sells.is_empty());

        assert_eq!(Some(&o),ob.buys.peek());

        let p = Order {id: 2, order_type: OrderType::Buy, price: 5, quantity: 100};
        ob.submit(p.clone());
        assert_eq!(Some(&o),ob.buys.peek());
        let q = Order {id: 3, order_type: OrderType::Buy, price: 20, quantity: 100};
        ob.submit(q.clone());
        assert_eq!(Some(&q),ob.buys.peek());
        ob.buys.pop();
        assert_eq!(Some(&o),ob.buys.peek());
        ob.buys.pop();
        assert_eq!(Some(&p),ob.buys.peek());
        assert!(!ob.buys.is_empty());
        assert!(ob.sells.is_empty());
    }
    #[test]
    fn trade_1() {
        let mut ob = OrderBook::new();
        assert!(ob.trades.is_empty());
        let o = Order {id: 1, order_type: OrderType::Sell, price: 5000, quantity: 100};
        ob.submit(o.clone());
        ob.trade_wrapper();
        let q = Order {id: 2, order_type: OrderType::Buy, price: 6000, quantity: 50};
        ob.submit(q.clone());
        ob.trade_wrapper();

        let t = Trade {buy_id: 2, sell_id: 1, price: 5000, quantity_traded: 50};
        ob.trade_wrapper();
        assert!(!ob.trades.is_empty());
        assert_eq!(Some(t), ob.trades.pop());
    }
    #[test]
    fn trade_2() {
        let mut ob = OrderBook::new();
        assert!(ob.trades.is_empty());
        let s1 = Order {id: 1, order_type: OrderType::Sell, price: 5001, quantity: 100};
        ob.submit(s1.clone());
        ob.trade_wrapper();
        let s2 = Order {id: 2, order_type: OrderType::Sell, price: 5000, quantity: 25};
        ob.submit(s2.clone());
        ob.trade_wrapper();
        let b1 = Order {id: 3, order_type: OrderType::Buy, price: 6000, quantity: 50};
        ob.submit(b1.clone());
        ob.trade_wrapper();

        let t1 = Trade {buy_id: 3, sell_id: 2, price: 5000, quantity_traded: 25};
        let t2 = Trade {buy_id: 3, sell_id: 1, price: 5001, quantity_traded: 25};
        assert!(!ob.trades.is_empty());
        assert_eq!(Some(t2), ob.trades.pop());
        assert_eq!(Some(t1), ob.trades.pop());
    }
    #[test]
    fn trade_3() {
        let mut ob = OrderBook::new();
        assert!(ob.trades.is_empty());
        let s1 = Order {id: 1, order_type: OrderType::Sell, price: 5000, quantity: 75};
        ob.submit(s1.clone());
        ob.trade_wrapper();
        let b1 = Order {id: 2, order_type: OrderType::Buy, price: 6000, quantity: 50};
        ob.submit(b1.clone());
        ob.trade_wrapper();
        let b2 = Order {id: 3, order_type: OrderType::Buy, price: 6000, quantity: 50};
        ob.submit(b2.clone());
        ob.trade_wrapper();

        let t1 = Trade {buy_id: 2, sell_id: 1, price: 5000, quantity_traded: 50};
        let t2 = Trade {buy_id: 3, sell_id: 1, price: 5000, quantity_traded: 25};
        assert!(!ob.trades.is_empty());
        assert_eq!(Some(t2), ob.trades.pop());
        assert_eq!(Some(t1), ob.trades.pop());
    }
    #[test]
    fn trade_4() {
        let mut ob = OrderBook::new();
        assert!(ob.trades.is_empty());
        let s1 = Order {id: 1, order_type: OrderType::Sell, price: 5000, quantity: 75};
        ob.submit(s1.clone());
        ob.trade_wrapper();
        let b1 = Order {id: 2, order_type: OrderType::Buy, price: 6000, quantity: 100};
        ob.submit(b1.clone());
        ob.trade_wrapper();
        let s2 = Order {id: 3, order_type: OrderType::Sell, price: 5000, quantity: 75};
        ob.submit(s2.clone());
        ob.trade_wrapper();
        let b2 = Order {id: 4, order_type: OrderType::Buy, price: 6000, quantity: 50};
        ob.submit(b2.clone());
        ob.trade_wrapper();

        let t1 = Trade {buy_id: 2, sell_id: 1, price: 5000, quantity_traded: 75};
        let t2 = Trade {buy_id: 4, sell_id: 3, price: 5000, quantity_traded: 50};
        ob.trade_wrapper();
        assert!(!ob.trades.is_empty());
        assert_eq!(Some(t2), ob.trades.pop());
        assert_eq!(Some(t1), ob.trades.pop());
    }
}
