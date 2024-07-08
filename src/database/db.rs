use rusqlite::{ params, Connection, Result };

use crate::models::trade::Trade;
use crate::models::politician::Politician;

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
      trade.price,
      trade.size,
      trade.reporting_gap,
      trade.buy
    ],
  )?;
  Ok(())
}

pub fn query_trades_by_politician_name(conn: &Connection, politician_name: &str) -> Result<Vec<Trade>> {
  let mut sel_statement = conn.prepare("SELECT politician_name, politician_state,
    politician_position, politician_party, trade_issuer, publish_date, traded_date, price, size,
    reporting_gap, buy FROM trade_db WHERE politician_name = ?1")?;

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

  let mut trades = Vec::new();

  for trade in trade_iter {
    trades.push(trade?);
  }

  Ok(trades)
}
