use crate::models::trade::Trade;
use actix_web::HttpResponse;


pub fn serialize_trade(trade: &Trade) -> serde_json::Value {
  serde_json::json!({
    "Politician Name:": trade.politician.name,
    "Politician State:": trade.politician.state,
    "Politician Position:": format!("{}", trade.politician.position),
    "Politician Party:": format!("{}", trade.politician.party),
    "Trade Issuer:": trade.trade_issuer,
    "Publish Date:": trade.publish_date,
    "Traded Date:": trade.traded_date,
    "Price:": format!("{}", trade.price),
    "Size:": trade.size,
    "Reporting Gap:": trade.reporting_gap,
    "Type:": trade.buy,
  })
}

pub fn respond(trades: Vec<Trade>) -> HttpResponse {
  let mut serialized_trades = Vec::new();

  for trade in &trades {
    let serialized_trade = serialize_trade(trade);
    serialized_trades.push(serialized_trade);
  }

  HttpResponse::Ok().json(serialized_trades)
}
