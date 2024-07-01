use scraper::{Html, Selector};
use crate::politician::Party;
use crate::politician::Politician;
use crate::politician::Position;
use crate::trade::Trade;  


pub fn get_num_table_pages(fragment: &Html) {
  let number_of_pages: u32;
  
  let number_selector = Selector::parse("b")
    .expect("Failed to parse number of pages.");
  
  let number = fragment.select(&number_selector).nth(1).expect("No number.").inner_html();

  println!("NUMBER OF PAGES: {}", number);
}

pub fn gather_table_entries(document: &Html) -> Vec<Html> {
  let table_entry_selector = Selector::parse("tr.q-tr").expect("Failed to parse table entries");
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

  table_entry_fragments
}

pub fn process_trade_fragment(fragment: &Html) -> Trade {

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

pub fn process_politician(fragment: &Html) -> Politician {

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
    Party::Republican
  } else {
    Party::Democrat
  };

  let position = if position == "House" {
    Position::House
  } else {
    Position::Senate
  };

  Politician::new(name, state, position, party)
}

