use rusqlite::Connection;
use actix_web:: {
  get,
  web,
  HttpResponse,
  Responder,
};

use crate::api::serialization::respond;
use crate::database::queries::query_trades_by_politician_name;

#[get("/politician/{politician_name}")]
pub async fn by_politician(path: web::Path<String>) -> impl Responder {
  // establish the connection to the database (globalize this across the API?)
  let conn = Connection::open("trade_database.database").expect("Error opening database");
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
