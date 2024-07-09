use std::error::Error;
use scraper::{Html, Selector};
use rusqlite::Connection;

use std::env;
use dotenv::dotenv;

use crate::{
  database::{
    db::create_table,
    db::insert_trade,
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

// scrape the webpage and extract necessary information for the API to provide
pub fn scrape(pages: u32) -> Result<(), Box<dyn Error>> {
   
  dotenv().ok();
  let url = env::var("WEBSITE_URL").expect("WEBSITE_URL not set.");
  
  // initialize database
  let conn = Connection::open("trade_database.database")?; 
  let _ = create_table(&conn);
  
  // current page number being fetched
  let mut page_number = 1;    
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
      trade.print();
      insert_trade(&conn, &trade)?;
    }

    page_number += 1;
  }  
    
  //let qtrades = query_trades_by_politician_name(&conn, "Don Beyer")?;

  //for trade in qtrades {
  //  trade.print();
  //}

  Ok(())
}
