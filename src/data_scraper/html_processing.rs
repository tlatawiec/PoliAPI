use scraper::{Html, Selector};

use crate::{
  models:: {
    politician::{
      Politician,
      Position,
      Party,
    },
    trade::Trade,
  }
};

use chrono::{
  NaiveTime,
  NaiveDate,
  Timelike,
  Duration,
  Utc,
};

// return the total number of table pages available for parsing
pub fn get_num_table_pages(fragment: &Html) -> u32 { 
  // get the number from the html
  let number_selector = Selector::parse("b")
    .expect("Failed to parse number of pages.");  
  let number = fragment.select(&number_selector).nth(1).expect("No number.").inner_html();
  
  // parse the number into an integer
  let number = match number.parse::<u32>() {
    Ok(value) => value,
    Err(_) => {
      eprintln!("Error parsing total number of pages.");
      0
    }
  };
  
  // returrn the number
  number  
}

// convert dates to a standard 'YYYY-MM-DD'
pub fn convert_date(date: &str) -> Option<String> {
  // get todays and yesterdays date  
  let today = Utc::now().date_naive();
  let yesterday = today - Duration::days(1);
  
  // if the date is in form "HH:MM Today" convert it to today's date
  if date.contains("Today") {
    return Some(today.format("%Y-%m-%d").to_string());
  } 
  
  // if the date is in form "HH:MM Yesterday" convert it to yesterday's date
  else if date.contains("Yesterday") {
    return Some(yesterday.format("%Y-%m-%d").to_string());
  }
  
  // otherwise convert "DD MM YYYY" to standard format
  if let Ok(date) = NaiveDate::parse_from_str(date, "%d %b %Y") {

    return Some(date.format("%Y-%m-%d").to_string());
  }

  None
}

// converts the time from UTC to EST
pub fn convert_utc_to_eastern(utc_str: &str) -> String {
  
  // turn the string into "HH:MM" NaiveTime object
  if let Ok(time) = NaiveTime::parse_from_str(utc_str, "%H:%M") {
    // calculate the hour
    let eastern_time = (time.hour() - 4 + 24) % 24;
    // format the time "HH:MM" with the newly calculated hour
    return format!("{:02}:{:02}", eastern_time, time.minute());
  }
  // if the format is not originally "HH:MM" return the date
  utc_str.to_string()
}

// grabs all table entries and stores them in a vector of Html fragments
pub fn gather_table_entries(document: &Html) -> Vec<Html> {
  // select all table entries
  let table_entry_selector = Selector::parse("tr.q-tr").expect("Failed to parse table entries");
  let table_entries = document.select(&table_entry_selector).collect::<Vec<_>>();

  // Turn each selection into a fragment and collect in a vector
  let table_entry_fragments = table_entries
    .iter()
    .map(|entry| {
      let entry_html = entry.html();
      Html::parse_fragment(&entry_html)
    })
    .collect::<Vec<Html>>();
  
  // return vector of html fragments
  table_entry_fragments
}

// process and entire trade fragment and create a trade struct with the parsed information
pub fn process_trade_fragment(fragment: &Html) -> Trade { 
  // initialize selectors 
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

  // extract information from the selectors
  let issuer_name = fragment.select(&issuer_selector).next().expect("No issuer").inner_html();
  let type_name = fragment.select(&type_selector).next().expect("No type").inner_html();
  let date_top = fragment.select(&date_top_selector);
  let date_bottom = fragment.select(&date_bottom_selector);
  let filed_days = fragment.select(&filed_days_selector).next().expect("No filed days").inner_html();
  let size = fragment.select(&size_selector).next().expect("No trade size").inner_html();
  let price = fragment.select(&price_selector).next()
    .or_else(|| fragment.select(&price_selector_na).next())
    .expect("No price").inner_html()
;
  // iterate over the dates in a loop because there are multiple with the same selector
  let top_dates: Vec<String> = date_top
    .map(|date| convert_utc_to_eastern(&date.inner_html()))
    .collect();

  let bottom_dates: Vec<String> = date_bottom
    .map(|date| date.inner_html())
    .collect();
  
  // parse the dates and convert them to a standar 'YYYY-MM-DD'  
  let published_date;
  let traded_date;

  if let Some(date) = convert_date(&format!("{} {}", top_dates[0], bottom_dates[0])) {
    published_date = date;  
  } else {
    published_date = "FAILED TO PARSE".to_string();
  }

  if let Some(date) = convert_date(&format!("{} {}", top_dates[1], bottom_dates[1])) {
    traded_date = date;
  } else {
    traded_date = "FAILED TO PARSE".to_string();
  }

  // create politician object
  let politician = process_politician(fragment);
  
  // return trade object
  Trade::new(politician, issuer_name, published_date, traded_date, filed_days, size, price, type_name)
}

// process the politician fragment of the trade
pub fn process_politician(fragment: &Html) -> Politician {
  
  // initialize selectors
  let name_selector = Selector::parse("a.text-txt-interactive")
    .expect("Failed to parse name.");

  let party_selector = Selector::parse("span.q-field.party")
    .expect("Failed to parse party.");

  let position_selector = Selector::parse("span.q-field.chamber")
    .expect("Failed to parse position.");

  let state_selector = Selector::parse("span.q-field.us-state-compact")
    .expect("Failed to parse state.");
    
  // grab information from selectors
  let name = fragment.select(&name_selector).next().expect("No name").inner_html();
  let party = fragment.select(&party_selector).next().expect("No party").inner_html();
  let position = fragment.select(&position_selector).next().expect("No position").inner_html();
  let state = fragment.select(&state_selector).next().expect("No state").inner_html();
  
  // set politician enums
  let party = if party == "Republican" {
    Party::Republican
  } else if party == "Democrat" {
    Party::Democrat
  } else {
    Party::Independent
  };

  let position = if position == "House" {
    Position::House
  } else {
    Position::Senate
  };

  // return politician object
  Politician::new(name, state, position, party)
}
