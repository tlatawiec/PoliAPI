use rusqlite::{ Connection, Result };

use crate::models::trade::Trade;
use crate::models::politician::Politician;

// query the trades based off of the name of the politician
pub fn query_trades_by_politician_name(conn: &Connection, politician_name: &str) -> Result<Vec<Trade>> {
  let mut sel_statement = conn.prepare("SELECT politician_name, politician_state,
    politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size,
    reporting_gap, buy FROM trade_db WHERE politician_name = ?1")?;
  
  // iterate over the pulled rows and create trade objects of the trades
  let trade_iter = sel_statement.query_map([politician_name], |row| {
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
        price: row.get(7)?,
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
