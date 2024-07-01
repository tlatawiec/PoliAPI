use reqwest;
use std::env;
use dotenv::dotenv;
use scraper::{Html, Selector};

/*

  NEXT -- Implement fracture function to return trade objects
	    * create new function that also parses out the politician information and returns them as politician objects

*/



struct Politician {
  name: String,	    // name of politician
  state: String,    // state of politician
  position: char,   // posititon of politician ('H' - House | 'S' - Senate)
  party: char,	    // party of politician ('R' - Republican | 'D' - Democrat | 'O' - Other)
}

impl Politician {
  // politician contructor
  fn new(name: String, state: String, position: char, party: char) -> Politician {
    Politician { name, state, position, party }
  }
  
  fn print(&self) {
 
    println!("Name: {} [\n\tState: {} Position: {} Party: {}", self.name, self.state, self.position, self.party);
  }
}

struct Trade {
  politician: Politician, // politician executing the trade
  trade_issuer: String,	  // trade issuer
  publish_date: String,	  // date trade was published
  traded_date: String,	  // date trade was conducted
  price: String,	  // price per share
  size: String,		  // size of trade  
  reporting_gap: String,  // gap between trade and publishing  
  buy: String,		  // buy or sell (true - buy | false - sell)
}

impl Trade {
  // trade constructor
  fn new(politician: Politician, trade_issuer: String, publish_date: String, traded_date: String, reporting_gap: String, size: String, price: String, buy: String) -> Trade {

    Trade { politician, trade_issuer, publish_date, traded_date, reporting_gap, size, price, buy }
  }
  
  fn print(&self) {
    self.politician.print();
    println!("\tIssuer: {}\n\tPublished: {}\n\tTraded: {}", self.trade_issuer, self.publish_date, self.traded_date);
    println!("\tPrice: {}\n\tSize: {}\n\tReported After: {} days\n\tType: {}", self.price, self.size, self.reporting_gap, self.buy);
    println!("]\n");
  }
}

fn fetch_html(url: String) -> Html {
  // send a request to fetch the url page
  let response = reqwest::blocking::get(url).expect("Could not load url.");
 
  // parse the get request into raw html
  let raw_document = response.text().unwrap();
  Html::parse_document(&raw_document)
}

fn process_trade_fragment(fragment: &Html) -> Trade {
  
  // create selectors for the politician fragment
  // NEED TO FIGURE OUT HOW TO DISINGUISH BETWEEN PUBLISHED/TRADED/FILED AFTER

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

  let published_date = format!("{} {}", top_dates[0], bottom_dates[0]); 
  let traded_date = format!("{} {}", top_dates[1], bottom_dates[1]); 
   
  // create trade object with data
  let politician = process_politician(fragment);  

  Trade::new(politician, issuer_name, published_date, traded_date, filed_days, size, price, type_name)
}

fn process_politician(fragment: &Html) -> Politician {
  
  let name_selector = Selector::parse("a.text-txt-interactive")
    .expect("Failed to parse name.");
  
  let party_selector = Selector::parse("span.q-field.party")
    .expect("Failed to parse party.");
  
  let position_selector = Selector::parse("span.q-field.chamber")
    .expect("Failed to parse position.");

  let state_selector = Selector::parse("span.q-field.us-state-compact")
    .expect("Failed to parse state."); 
  
  let name = fragment.select(&name_selector).next().expect("No name").inner_html();
  let party = fragment.select(&party_selector).next().expect("No party").inner_html();
  let position = fragment.select(&position_selector).next().expect("No position").inner_html();
  let state = fragment.select(&state_selector).next().expect("No state").inner_html();
    
  let party = if party == "Republican" {
    'R'
  } else {
    'D'
  }; 

  let position = if position == "House" {
    'H'
  } else {
    'S'
  };

  Politician::new(name, state, position, party)
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
  let trades: Vec<Trade> = table_entry_fragments.iter().skip(1).map(|fragment| {
    process_trade_fragment(fragment)
  })
  .collect();

  for trade in trades {
    trade.print();
  }      
}
