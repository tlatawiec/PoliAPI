mod data_scraper;
mod database;
mod models;

use std::env;
use std::error::Error;
use dotenv::dotenv;
use scraper::{Html, Selector};
use rusqlite::Connection;

use crate::{
  database::{
    db::create_table,
    db::insert_trade,
    db::query_trades_by_politician_name,
  },
  data_scraper::{
    html_processing::{
      process_trade_fragment,
      get_num_table_pages,
      gather_table_entries,
    },
  web_fetch::fetch_html,
  },
};

fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();	      // environment variables

  let conn = Connection::open("trade_database.database")?; // initialize database
  let _ = create_table(&conn);

  let mut page_number = 1;    // current page number being fetched
  let pages = 4;	      // number of pages to be fetched      

  // fetch initial webpage
  let url = env::var("WEBSITE_URL").expect("WEBSITE_URL not set.");
  let document = fetch_html(&url);

  // obtain the page number fragment
  let table_next_page_selector = Selector::parse("p.hidden.leading-7.sm\\:block").expect("Failed to pparse out next page selector"); 
  let table_next_page = document.select(&table_next_page_selector).next().expect("Failed to parse page selector");

  // calculate the maximum number of pages available to parse 
  get_num_table_pages(&(Html::parse_fragment(&table_next_page.html())));

  // iterate over the pages
  while page_number <= pages {  
    // format the string so that it is in the form url/page=x to access all table pages
    let page_url = format!("{}?page={}", url, page_number);
    println!("{}", page_url);

    // fetch next page url
    let page_html = fetch_html(&page_url);

    // gather the table contents
    let html_fragments = gather_table_entries(&page_html);

    // iterate over the table entries and process the fragments
    for fragment in html_fragments.iter().skip(1) {
      let trade = process_trade_fragment(fragment);
      insert_trade(&conn, &trade)?; 
    }

    page_number += 1;
  }  

  let qtrades = query_trades_by_politician_name(&conn, "Don Beyer")?;
  
  for trade in qtrades {
    trade.print();
  }

  Ok(())
}
