use reqwest;
use std::env;
use dotenv::dotenv;
use scraper::{Html, Selector};
/*
  Struct for a trade:
  
  struct trade {
    Politician p;
    string traded issuer;
    (type?) publish_date;
    (type?) traded_date;
    u16 size: size;
    u32: price_per_share;
    char buy:		    // 'b' - Buy | 's' - Sell
  };

  Struct for a politician:

  struct politician {
    string Name
    string state;
    char position;	    // 'h' - House | 's' - Senate
    char party;		    // 'r' - Republican | 'd' - Democrat | 'o' - Other
  };
*/

fn fetch_html(url: String) -> Html {
  // send a request to fetch the url page
  let response = reqwest::blocking::get(url).expect("Could not load url.");
 
  // parse the get request into raw html
  let raw_document = response.text().unwrap();
  Html::parse_document(&raw_document)
}

fn process_trade_fragment(fragment: &Html) -> [String; 11] {
  
  // create selectors for the politician fragment
  // NEED TO FIGURE OUT HOW TO DISINGUISH BETWEEN PUBLISHED/TRADED/FILED AFTER
  let politician_selector = Selector::parse("a.text-txt-interactive")
    .expect("Failed to parse name.");

  let issuer_selector = Selector::parse("a.hover\\:no-underline.text-txt-interactive")
    .expect("Failed to parse issuer.");

  let type_selector = Selector::parse("span.q-field.tx-type")
    .expect("Failed to parse type.");

  let date_top_selector = Selector::parse("div.text-size-3.font-medium")
    .expect("Failed to parse date published (top)");

  let date_bottom_selector = Selector::parse("div.text-size-2.text-txt-dimmer")
    .expect("Failed to parse date published (bottom).");
  
  let filed_days_selector = Selector::parse("span[class^='reporting-gap-tier--']")
    .expect("Failed to parse reporting gap.");
  
  let size_selector = Selector::parse("span[class='mt-1 text-size-2 text-txt-dimmer hover:text-foreground']").expect("Failed to parse trade size.");
  
  let price_selector = Selector::parse("span.q-field.trade-price")
    .expect("Failed to parse price selector");
  
  let price_selector_na = Selector::parse("span.no-price")
    .expect("Failed to parse price selector na"); 
  // Extract inner text from the selectors
  let politician_name = fragment.select(&politician_selector).next().expect("No name").inner_html();
  let issuer_name = fragment.select(&issuer_selector).next().expect("No issuer").inner_html();
  let type_name = fragment.select(&type_selector).next().expect("No type").inner_html();
  let date_top = fragment.select(&date_top_selector);
  let date_bottom = fragment.select(&date_bottom_selector);
  let filed_days = fragment.select(&filed_days_selector).next().expect("No filed days").inner_html();
  let size = fragment.select(&size_selector).next().expect("No trade size").inner_html();
  let price = fragment.select(&price_selector).next()
    .or_else(|| fragment.select(&price_selector_na).next())
    .expect("No price").inner_html();     
  // iterate over the dates in a loop because there are multiple with the same selector 
  let top_dates: Vec<String> = date_top
    .map(|date| date.inner_html())
    .collect();
  
  let bottom_dates: Vec<String> = date_bottom
    .map(|date| date.inner_html())
    .collect();

  // return array of scraped data
  [
    politician_name,
    issuer_name,
    type_name,
    top_dates.get(0).cloned().unwrap_or("".to_string()),
    bottom_dates.get(0).cloned().unwrap_or("".to_string()),
    top_dates.get(1).cloned().unwrap_or("".to_string()),
    bottom_dates.get(1).cloned().unwrap_or("".to_string()),
    filed_days,
    "days".to_string(),
    size,
    price,
      
  ]
}

fn main() {
  // fetch the current page
  dotenv().ok();
  
  let url = env::var("WEBSITE_URL").expect("WEBSITE_URL not set.");
  let document = fetch_html(url);
  
  // create the table selector and table entry selector
  let table_entry_selector = Selector::parse("tr.q-tr").expect("Failed to parse out tr.q-tr");
  
  // find the table body
  let table_entries = document.select(&table_entry_selector).collect::<Vec<_>>();
      // Turn each selection into a fragment and collect in a vector
  let table_entry_fragments = table_entries
    .iter()
    .map(|entry| {
      let entry_html = entry.html();
      Html::parse_fragment(&entry_html)
    })
    .collect::<Vec<Html>>();

  // Debug output to check the fragments
  let results: Vec<[String; 11]> = table_entry_fragments.iter().enumerate().filter_map(|(index, fragment)| {
    if index != 0 {
      Some(process_trade_fragment(fragment))
    } else {
      None
    }
  }).collect();

  for (index, result) in results.iter().enumerate() {
    println!("Result {}: {:?}", index + 1, result);
  }       
}
