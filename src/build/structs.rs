//mod structs {
use std::collections::{HashMap,BinaryHeap};
use std::cmp::Ordering;
use std::hash::{Hash,Hasher};

    #[derive(Debug,Clone)]
    pub enum OrderType {
        Buy,
        Sell,
    }

    #[derive(Debug,Clone)]
    pub struct Order {
        pub id: usize,
        pub order_type: OrderType,
        pub price: u32,
        pub quantity: u32
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Trade {
        pub buy_id: usize,
        pub sell_id: usize,
        pub price: u32,
        pub quantity_traded: u32
    }

    #[derive(Debug)]
    pub struct OrderBook {
        pub orders: HashMap<usize, Order>,
        pub buys: BinaryHeap<Order>,
        pub sells: BinaryHeap<Order>,
        pub trades: Vec<Trade>
    }

    impl PartialEq for Order {
        fn eq(&self, other: &Self) -> bool {
            self.price == other.price
        }
    }

    impl Eq for Order {}

    impl Hash for Order {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.price.hash(hasher)
        }
    }

    impl PartialOrd for Order {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.order_type {
                OrderType::Sell => Some(other.price.cmp(&self.price)),
                OrderType::Buy => Some(self.price.cmp(&other.price))
            }
        }
    }

    impl Ord for Order {
        fn cmp(&self, other: &Self) -> Ordering {
            self.price.cmp(&other.price)
         }
    }

//}
