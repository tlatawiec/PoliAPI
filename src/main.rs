mod data_scraper;
mod database;
mod models;

use crate::data_scraper::scraper::scrape;
use rusqlite::Connection;

use actix_web:: {
  get,
  web,
  App, 
  HttpResponse,
  HttpServer,
  Responder,
};

use crate::database::db::query_trades_by_politician_name;

#[get("/by_politician/{politician_name}")]
async fn by_politician(path: web::Path<String>) -> impl Responder {
  // establish the connection to the database (globalize this across the API?
  let conn = Connection::open("trade_database.database").expect("Error opening database"); 
  // extract the politician name from the path extractor
  let politician_name = path.into_inner();
  
  // query and serialize the politician at the path '/by_politician/(politician name)'
  match query_trades_by_politician_name(&conn, &politician_name) {
    // if returned successfully serialize the vector of trades
    Ok(trades) => {
      let mut serialized_trades = Vec::new();

      for trade in &trades {
	let serialized_trade = serde_json::json!({
	  "Politician Name:": trade.politician.name,
	  "Politician State:": trade.politician.state,
	  "Politician Position:": format!("{}", trade.politician.position),
	  "Politician Party:": format!("{}", trade.politician.party),
	  "Trade Issuer:": trade.trade_issuer,
	  "Publish Date:": trade.publish_date,
	  "Traded Date:": trade.traded_date,
	  "Price:": trade.price,
	  "Size:": trade.size,
	  "Reporting Gap:": trade.reporting_gap,
	  "Type:": trade.buy,
	});
	serialized_trades.push(serialized_trade);	
      } 
      HttpResponse::Ok().json(serialized_trades)
    }
    // otherwise return an error
    Err(err) => {
      // fix this error handling
      HttpResponse::Ok().body(format!("Database error: {}", err))
    }
  }   
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // scrape top 3 pages of the site and populate database with new entries
  tokio::task::spawn_blocking(move || {
    scrape(1).unwrap_or_else(|err| {
      eprintln!("Error scraping and populating database: {}", err);
    });
  });

  // BUILD WEB SERVICE HERE
  HttpServer::new(|| {
    App::new()
      .service(by_politician)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
