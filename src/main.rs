mod data_scraper;
mod database;
mod models;
mod api;

use actix_web::{
  App,
  HttpServer,
};

use crate::api::response::{
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

use tokio::time::{interval, Duration};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Schedule the scraper to run every 15 minutes
  let scraper_handle = tokio::spawn(async {
  let mut interval = interval(Duration::from_secs(900)); // 15 minutes

    loop {
      interval.tick().await;
      tokio::task::spawn_blocking(|| {
        if let Err(err) = scrape(3) {
          eprintln!("Error scraping and populating database: {}", err);
        }
      }).await.unwrap();
    }
  });

  // Run the Actix web server
  let server_handle = HttpServer::new(|| {
    App::new()
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
  .run();

    // Run both the scraper and the server concurrently
    tokio::select! {
        res = scraper_handle => {
            if let Err(e) = res {
                eprintln!("Scraper task failed: {:?}", e);
            }
        },
        res = server_handle => {
            if let Err(e) = res {
                eprintln!("Server task failed: {:?}", e);
            }
        },
    }
  Ok(())
}
