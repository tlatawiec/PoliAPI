use rusqlite::{ params, Connection, Result };

use crate::models::trade::Trade;

// create the table if it does not exist
pub fn create_table(conn: &Connection) -> Result<()> {
  conn.execute(
    "CREATE TABLE IF NOT EXISTS trade_db (
      id INTEGER PRIMARY KEY,
      politician_name TEXT NOT NULL,
      politician_state TEXT NOT NULL,
      politician_position TEXT NOT NULL,
      politician_party TEXT NOT NULL,
      trade_issuer TEXT NOT NULL,
      publish_date TEXT NOT NULL,
      traded_date TEXT NOT NULL,
      price TEXT NOT NULL,
      size TEXT NOT NULL,
      reporting_gap TEXT NOT NULL,
      buy TEXT NOT NULL,
      UNIQUE(politician_name, trade_issuer, traded_date, price, size, buy)
    )",
    [],
  )?;
  Ok(())
}

// insert a trade into the table
pub fn insert_trade(conn: &Connection, trade: &Trade) -> Result<()> {
  conn.execute(
    "INSERT OR IGNORE INTO trade_db
      (politician_name, politician_state, politician_position, politician_party, trade_issuer,
      publish_date, traded_date, price, size, reporting_gap, buy)
	VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    params![
      trade.politician.name,
      trade.politician.state,
      trade.politician.position,
      trade.politician.party,
      trade.trade_issuer,
      trade.publish_date,
      trade.traded_date,
      trade.price.to_string(),
      trade.size,
      trade.reporting_gap,
      trade.buy
    ],
  )?; 
  Ok(())
}
