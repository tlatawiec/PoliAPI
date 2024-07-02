use reqwest;
use scraper::Html;

// fetch the html contents of the page at "url"
pub fn fetch_html(url: &str) -> Html {
  // send a request to fetch the url page
  let response = reqwest::blocking::get(url).expect("Could not load url.");

  // parse the get request into raw html
  let raw_document = response.text().unwrap();
  Html::parse_document(&raw_document)
}
