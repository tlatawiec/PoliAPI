use crate::models::politician::Politician;
use rusqlite::{
  Result,
  ToSql,
  types::{
    ToSqlOutput,
    ValueRef,
    FromSql,
    FromSqlError,
    FromSqlResult,
  },
};
use std::fmt;

pub struct Trade {
  pub politician: Politician,	    // politician executing the trade
  pub trade_issuer: String,         // trade issuer
  pub publish_date: String,         // date trade was published
  pub traded_date: String,          // date trade was conducted
  pub price: Price,		    // price per share
  pub size: String,                 // size of trade
  pub reporting_gap: String,	    // gap between trade and publishing
  pub buy: String,                  // buy or sell (true - buy | false - sell)
}

pub enum Price {
  Value(f64),
  NA,
}

impl fmt::Display for Price {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Price::Value(val) => write!(f, "{:.2}", val),
      Price::NA => write!(f, "N/A"),
    }
  }
}

impl ToSql for Price {
  fn to_sql(&self) -> Result<ToSqlOutput> {
    match *self {
      Price::Value(val) => Ok(ToSqlOutput::from(format!("{:.2}", val))),
      Price::NA => Ok(ToSqlOutput::from("N/A")),
    }
  }
}

impl FromSql for Price {
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    match value {
      ValueRef::Real(val) => Ok(Price::Value(val as f64)),
      ValueRef::Text(val) => {
	if val == b"N/A" {
	  Ok(Price::NA)
	} else {
	  Err(FromSqlError::InvalidType)
	}
      }
      _ => Err(FromSqlError::InvalidType),
    }
  }
}

impl Trade {
  // trade constructor
  pub fn new(politician: Politician, trade_issuer: String, publish_date: String, traded_date: String, reporting_gap: String, size: String, price: String, buy: String) -> Trade {
    
    let price = match price.parse::<f64>() {
      Ok(value) => Price::Value(value),
      Err(_) => {
	Price::NA
      },
    };

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


