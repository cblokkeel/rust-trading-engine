mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
use crate::matching_engine::engine::{MatchingEngine, PlaceLimitOrderParams, TradingPair};

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order);

    // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let trading_pair = TradingPair::new(String::from("BTC"), String::from("USD"));

    engine.add_new_market(trading_pair.clone());
    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(PlaceLimitOrderParams::new(
        trading_pair,
        10_000.0,
        buy_order
    )).unwrap();


    println!("engine={:?}", engine);
}
