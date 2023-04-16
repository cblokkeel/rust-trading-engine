use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {
                let price = Price::new(price);
                match self.asks.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            },
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            },
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar: u64 = 100000;
        let integral: u64 = price as u64;
        let fractional: u64 = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }

    fn get_price(self) -> f64 {
        (self.integral as f64) + (self.fractional as f64) / (self.scalar as f64)
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
   fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
      Order {
          bid_or_ask,
          size,
      }
   }
}

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    println!("{:?}", orderbook);
}
