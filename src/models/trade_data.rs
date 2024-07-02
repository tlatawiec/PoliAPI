use crate::models::trade::Trade;
use std::collections::HashSet;

pub struct Data {
  latest_trade: Option<Trade>,
  time_since_last_trade: u64,
  trades: HashSet<Trade>,
}

impl Data {
  pub fn new() -> Data {
    Data {
      latest_trade: None, 
      time_since_last_trade: 99999,
      trades: HashSet::new(),
    }
  }


  fn insert_trade(&mut self, trade: Trade) {
    self.trades.insert(trade);
  }
}
