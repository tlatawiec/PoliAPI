mod data_scraper;
mod database;
mod models;
mod api;

use actix_web::{
  App,
  HttpServer,
};

use crate::api::response::{
  root,
  by_politician,
  recent_published,
  published_within,
  recent_traded,
  traded_within,
  price_over,
  price_under,
  price_na,
  price_range,
  trade_size,
  by_issuer,
  by_type,
};
use crate::data_scraper::scraper::scrape;

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
      .service(root)
      .service(by_politician)
      .service(recent_published)
      .service(published_within)
      .service(recent_traded)
      .service(traded_within)
      .service(price_over)
      .service(price_under)
      .service(price_na)
      .service(price_range)
      .service(trade_size)
      .service(by_issuer)
      .service(by_type)
  })

  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
