//mod orderbook {
use super::structs::*;
use std::collections::{HashMap,BinaryHeap};
use std::cmp::min;
use std::error;
use super::errors::*;
use std::str::FromStr;

    impl OrderBook {
        pub fn new() -> OrderBook {
            return OrderBook {
                orders: HashMap::new(),
                buys: BinaryHeap::new(),
                sells: BinaryHeap::new(),
                trades: Vec::new()
            };
        }

        pub fn submit(&mut self, o: Order) -> bool {
            if self.orders.contains_key(&o.id) {
                println!("ID already used. Order not added.");
                return false;
            }
            match o.order_type {
                OrderType::Sell => self.sells.push(o.clone()),
                OrderType::Buy => self.buys.push(o.clone()),
            };
            self.orders.insert(o.id, o);
            true
        }

        pub fn trade_wrapper(&mut self) -> bool {
            if self.sells.len() == 0 || self.buys.len() == 0 {
                return false;
            }
            while self.sells.len() != 0 && self.buys.len() != 0
                && self.sells.peek().unwrap().price <= self.buys.peek().unwrap().price {
                self.trade();
            }
            if self.sells.is_empty() {
                self.buys.drain();
            }
            true

        }

        pub fn trade(&mut self) {
            let mut s = self.sells.pop().unwrap();
            let mut b = self.buys.pop().unwrap();
            if s.price <= b.price {
                let q = min(s.quantity, b.quantity);
                let t = Trade { buy_id: b.id, sell_id: s.id, price: s.price, quantity_traded: q };
                self.trades.push(t.clone());
                println!("Trade {} BTC @ {} USD between {} and {}", t.quantity_traded, t.price, t.buy_id, t.sell_id);
                if s.quantity != t.quantity_traded {
                    s.quantity -= t.quantity_traded;
                    self.sells.push(s.clone());
                }
                if b.quantity != t.quantity_traded {
                    b.quantity -= t.quantity_traded;
                    self.buys.push(b.clone());
                }
            }
        }
    }

    impl FromStr for Order {
        type Err = Box<dyn error::Error>;
        fn from_str(s: &str) -> Result<Order, Self::Err> {
            if s.len() == 0 {
                return Err(OrderError.into());
            }
            let s = s.split(' ').collect::<Vec<&str>>();
            if s.len() != 7 || s[0] == "" {
                return Err(OrderError.into());
            }
            if s[3] != "BTC" || s[4] != "@" || s[6] != "USD" {
                return Err(OrderError.into())
            }
            let id = s[0].trim_matches('.').to_string().parse::<usize>()?;
            let price = s[5].to_string().parse::<u32>()?;
            let quantity = s[2].to_string().parse::<u32>()?;
            Ok(Order {
                id: id,
                order_type: match s[1] {
                    "Buy" => OrderType::Buy,
                    "Sell" => OrderType::Sell,
                    &_ => return Err(OrderError.into())
                },
                price: price,
                quantity: quantity
            })
        }
    }
//}
