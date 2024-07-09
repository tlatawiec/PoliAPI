mod data_scraper;
mod database;
mod models;
mod api;

use actix_web::{
  App,
  HttpServer,
};

use crate::api::response::by_politician;
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
      .service(by_politician)
  })

  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
