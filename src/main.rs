mod data_scraper;
mod models;

use std::env;
use dotenv::dotenv;
use scraper::{Html, Selector};

use crate::{
  data_scraper::{
    html_processing::{
      process_trade_fragment,
      get_num_table_pages,
      gather_table_entries,
    },
  web_fetch::fetch_html,
  },
  
  models::trade::Trade,
  models::trade_data::Data,
};

fn main() {
  dotenv().ok();	      // environment variables
  let mut page_number : u32;  // current page number being fetched
  let pages : u32;	      // number of pages to be fetched   
  
  let data = Data::new();  
  
  pages = 5;
  page_number = 1; 
 
  // fetch initial webpage
  let url = env::var("WEBSITE_URL").expect("WEBSITE_URL not set.");
  let document = fetch_html(&url);
  
  // obtain the page number fragment
  let table_next_page_selector = Selector::parse("p.hidden.leading-7.sm\\:block").expect("Failed to pparse out next page selector"); 
  let table_next_page = document.select(&table_next_page_selector).next().expect("Failed to parse page selector");
  
  // calculate the maximum number of pages available to parse 
  get_num_table_pages(&(Html::parse_fragment(&table_next_page.html())));
  
  // iterate over the pages
  while page_number < pages {  

    // format the string so that it is in the form url/page=x to access all table pages
    let page_url = format!("{}?page={}", url, page_number);
    println!("{}", page_url);

    // fetch next page url
    let page_html = fetch_html(&page_url);

    // gather the table contents
    let html_fragments = gather_table_entries(&page_html);

    // iterate over the table entries and process the fragments
    let trades: Vec<Trade> = html_fragments.iter().skip(1).map(|fragment| {
      process_trade_fragment(fragment)
    })
    .collect();

    // print the trades
    for trade in &trades {
      trade.print();      
    }
    page_number += 1;
  }  
}
