use rusqlite::Connection;
use actix_web:: {
  get,
  web,
  HttpResponse,
  Responder,
};

use crate::api::serialization::respond;
use crate::database::queries::{
  query_trades_by_politician_name,
  query_trades_by_publish_date,
  query_trades_by_trade_date,
  query_trades_by_price_over,
  query_trades_by_price_under,
  query_trades_by_price_na,
  query_trades_by_price_range,
  query_trades_by_size, 
  query_trades_by_issuer_name,
  query_trades_by_type,
};

#[get("/api/politician/{politician_name}")]
pub async fn by_politician(path: web::Path<String>) -> impl Responder {
  // establish the connection to the database (globalize this across the API?)
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  // extract the politician name from the path extractor
  let politician_name = path.into_inner();

  // query and serialize the politician at the path '/by_politician/(politician name)'
  match query_trades_by_politician_name(&conn, &politician_name) {
    // if returned successfully serialize the vector of trades
    Ok(trades) => respond(trades),
    // otherwise return an error
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)), 
  }
}

#[get("/api/publish_date/recent")]
pub async fn recent_published() -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  
  match query_trades_by_publish_date(&conn, 2) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/published_within/{x}")]
pub async fn published_within(path: web::Path<i64>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let num_weeks = path.into_inner();
 
  match query_trades_by_publish_date(&conn, num_weeks) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}


#[get("/api/traded_date/recent")]
pub async fn recent_traded() -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  
  match query_trades_by_trade_date(&conn, 2) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/traded_within/{x}")]
pub async fn traded_within(path: web::Path<i64>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let num_weeks = path.into_inner();
 
  match query_trades_by_trade_date(&conn, num_weeks) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/price/over/{x}")]
pub async fn price_over(path: web::Path<String>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let price = path.into_inner();

  match query_trades_by_price_over(&conn, price) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/price/under/{x}")]
pub async fn price_under(path: web::Path<String>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let price = path.into_inner();

  match query_trades_by_price_under(&conn, price) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/price/na")]
pub async fn price_na() -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  
  match query_trades_by_price_na(&conn) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)),
  }
}

#[get("/api/price/{l}-{h}")]
pub async fn price_range(path: web::Path<(f64, f64)>) -> impl Responder { 
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");

  let prices = path.into_inner();
  let price_low = prices.0;
  let price_high = prices.1;

  match query_trades_by_price_range(&conn, price_low, price_high) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)), 
  }
}

#[get("/api/size/{trade_size}")]
pub async fn trade_size(path: web::Path<i32>) -> impl Responder { 
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let trade_size = path.into_inner();

  let size = match trade_size {
    0 => "1K–15K",
    1 => "15K–50K",
    2 => "50K–100K",
    3 => "100K–250K",
    4 => "250K–500K",
    5 => "500K–1M",
    6 => "1M–5M",
    7 => "5M–25M",
    8 => "25M–50M",
    _ => "",
  };
  
  match query_trades_by_size(&conn, size) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)), 
  }
}

#[get("/api/issuer/{issuer_name}")]
pub async fn by_issuer(path: web::Path<String>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let issuer_name = path.into_inner();

  match query_trades_by_issuer_name(&conn, &issuer_name) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)), 
  }
}

#[get("/api/type/{type}")]
pub async fn by_type(path: web::Path<String>) -> impl Responder {
  let conn = Connection::open("trade_database.database").expect("Couldn't open database");
  let bos = path.into_inner();

  match query_trades_by_type(&conn, &bos) {
    Ok(trades) => respond(trades),
    Err(err) => HttpResponse::InternalServerError().body(format!("Database error: {}", err)), 
  }
}
