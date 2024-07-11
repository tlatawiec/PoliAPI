use rusqlite::{ Connection, Result };
use chrono::{ Duration, Utc };

use crate::models::trade::Trade;
use crate::models::trade::Price;
use crate::models::politician::Politician;

fn convert_price(price: &str) -> Price { 
  if price == "N/A" {
    Price::NA
  } else {
    match price.parse::<f64>() {
      Ok(num) => Price::Value(num),
      Err(_) => Price::NA, 
    }
  }
}
// query the trades based off of the name of the politician
pub fn query_trades_by_politician_name(conn: &Connection, politician_name: &str) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare("SELECT politician_name, politician_state,
    politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size,
    reporting_gap, buy FROM trade_db WHERE politician_name = ?1")?;

  // iterate over the pulled rows and create trade objects of the trades
  let trade_iter = stmt.query_map([politician_name], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  // return a vector of the trades
  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades) 
}

pub fn query_trades_by_publish_date(conn: &Connection, weeks: i64) -> Result<Vec<Trade>> {
  let current_date = Utc::now().date_naive();
  let x_weeks_ago = current_date - Duration::days(weeks * 7);

  let mut stmt = conn.prepare("SELECT politician_name, politician_state, politician_position, 
    politician_party, trade_issuer, publish_date, traded_date, price, size, 
    reporting_gap, buy FROM trade_db WHERE publish_date BETWEEN ? AND ?")?;
   
  let trade_iter = stmt.query_map([x_weeks_ago.to_string(), current_date.to_string()], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?; 

  let mut trades = Vec::new();
  
  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}

pub fn query_trades_by_trade_date(conn: &Connection, weeks: i64) -> Result<Vec<Trade>> {
  let current_date = Utc::now().date_naive();
  let x_weeks_ago = current_date - Duration::days(weeks * 7);

  let mut stmt = conn.prepare("SELECT politician_name, politician_state, politician_position, 
    politician_party, trade_issuer, publish_date, traded_date, price, size, 
    reporting_gap, buy FROM trade_db WHERE traded_date BETWEEN ? AND ?")?;
   
  let trade_iter = stmt.query_map([x_weeks_ago.to_string(), current_date.to_string()], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?; 

  let mut trades = Vec::new();
  
  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}

pub fn query_trades_by_price_over(conn: &Connection, price: String) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare(
    "SELECT politician_name, politician_state, politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size, reporting_gap, buy,
      CASE
	WHEN price != 'N/A' THEN CAST(price AS REAL)
	ELSE NULL
      END AS price
    FROM trade_db WHERE price != 'N/A' AND CAST(price as REAL) > ?"
  )?;
  let trade_iter = stmt.query_map([price], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?; 

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}

pub fn query_trades_by_price_under(conn: &Connection, price: String) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare(
    "SELECT politician_name, politician_state, politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size, reporting_gap, buy,
      CASE
	WHEN price != 'N/A' THEN CAST(price AS REAL)
	ELSE NULL
      END AS price
    FROM trade_db WHERE price != 'N/A' AND CAST(price as REAL) < ?"
  )?;

  let trade_iter = stmt.query_map([price], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}

pub fn query_trades_by_price_na(conn: &Connection) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare(
    "SELECT politician_name, politician_state, politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size, reporting_gap, buy
    FROM trade_db WHERE price == 'N/A'"
  )?;

  let trade_iter = stmt.query_map([], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}


pub fn query_trades_by_price_range(conn: &Connection, price_low: f64, price_high: f64) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare(
    "SELECT politician_name, politician_state, politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size, reporting_gap, buy
    FROM trade_db WHERE price != 'N/A' AND CAST(price as REAL) > ? AND CAST(price as REAL) < ?"
  )?;

  let trade_iter = stmt.query_map([price_low, price_high], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}


pub fn query_trades_by_size(conn: &Connection, size: &str) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare(
    "SELECT politician_name, politician_state, politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size, reporting_gap, buy
    FROM trade_db WHERE size == ?"
  )?;

  let trade_iter = stmt.query_map([size], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}

pub fn query_trades_by_issuer_name(conn: &Connection, issuer_name: &str) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare("SELECT politician_name, politician_state,
    politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size,
    reporting_gap, buy FROM trade_db WHERE trade_issuer = ?1")?;

  // iterate over the pulled rows and create trade objects of the trades
  let trade_iter = stmt.query_map([issuer_name], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  // return a vector of the trades
  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades) 
}

pub fn query_trades_by_type(conn: &Connection, bos: &str) -> Result<Vec<Trade>> {
  let mut stmt = conn.prepare("SELECT politician_name, politician_state,
    politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size,
    reporting_gap, buy FROM trade_db WHERE buy = ?1")?;

  // iterate over the pulled rows and create trade objects of the trades
  let trade_iter = stmt.query_map([bos], |row| {
    let price_str: String = row.get(7)?;

    Ok(Trade {
        politician: Politician {
          name: row.get(0)?,
          state: row.get(1)?,
          position: row.get(2)?,
          party: row.get(3)?,
        },
        trade_issuer: row.get(4)?,
        publish_date: row.get(5)?,
        traded_date: row.get(6)?,
        price: convert_price(&price_str),
        size: row.get(8)?,
        reporting_gap: row.get(9)?,
        buy: row.get(10)?,
    })
  })?;

  // return a vector of the trades
  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades) 
}
