use crate::models::politician::Politician;
use std::hash::{Hash, Hasher};

pub struct Trade {
  politician: Politician,	// politician executing the trade
  trade_issuer: String,         // trade issuer
  publish_date: String,         // date trade was published
  traded_date: String,          // date trade was conducted
  price: String,		// price per share
  size: String,                 // size of trade
  reporting_gap: String,	// gap between trade and publishing
  buy: String,                  // buy or sell (true - buy | false - sell)
}

impl Trade {
  // trade constructor
  pub fn new(politician: Politician, trade_issuer: String, publish_date: String, traded_date: String, reporting_gap: String, size: String, price: String, buy: String) -> Trade {

    Trade { 
      politician,
      trade_issuer,   
      publish_date, 
      traded_date, 
      reporting_gap,
      size, 
      price, 
      buy 
    }
  }
    
    // print function for a trade
  pub fn print(&self) {
    self.politician.print();
    println!("\tIssuer: {}\n\tPublished: {}\n\tTraded: {}", self.trade_issuer, self.publish_date, self.traded_date);
    println!("\tPrice: {}\n\tSize: {}\n\tReported After: {} days\n\tType: {}", self.price, self.size, self.reporting_gap, self.buy);
    println!("]\n");
  }
}

// functions to allow for hashing trades
impl PartialEq for Trade {
  fn eq(&self, other: &Self) -> bool {
    self.politician.name == other.politician.name &&
    self.trade_issuer == other.trade_issuer &&
    self.traded_date == other.publish_date &&
    self.price == other.price &&
    self.buy == other.buy
  }
}

impl Eq for Trade {}

impl Hash for Trade {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.politician.name.hash(state);
  }
}
