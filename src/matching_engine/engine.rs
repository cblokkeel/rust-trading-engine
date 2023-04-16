use std::collections::HashMap;
use crate::matching_engine::orderbook::Order;
use super::orderbook::OrderBook;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }

    pub fn to_string(self) -> String {
       format!("{}_{}", self.base, self.quote)
    }
}

pub struct PlaceLimitOrderParams {
    pair: TradingPair,
    price: f64,
    order: Order,
}

impl PlaceLimitOrderParams {
    pub fn new(pair: TradingPair, price: f64, order: Order) -> PlaceLimitOrderParams {
        PlaceLimitOrderParams {
            pair,
            price,
            order,
        }
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
       MatchingEngine {
           orderbooks: HashMap::new(),
       }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        match self.orderbooks.get_mut(&pair) {
            None => {
                self.orderbooks.insert(pair.clone(), OrderBook::new());
                println!("Opening new orderbook for market {:?}", pair.to_string());
            },
            Some(_) => println!("Market already exists")
        };
    }

    pub fn place_limit_order(&mut self, params: PlaceLimitOrderParams) -> Result<(), String> {
        match self.orderbooks.get_mut(&params.pair) {
            Some(orderbook) => {
                orderbook.add_order(params.price, params.order.clone());
                println!("placed limit order {:?}", params.order);
                Ok(())
            },
            None => {
                Err(format!(
                    "the orderbook for the given trading pair ({}) does not exist",
                    params.pair.to_string(),
                ))
            }
        }
    }
}