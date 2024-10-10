use headless_chrome::Browser;
use scraper::{Html, Selector};
use std::time::Duration;

pub async fn web_scrape(query: &String) -> Result<String, String> {
    let formatted_query = query.replace(' ', "+");
    let search_url = format!("https://www.youtube.com/results?search_query={formatted_query}");
    match Browser::default() {
        Ok(browser) => {
            match browser.new_tab() {
                Ok(tab) => {
                    tab.navigate_to(&search_url).map_err(|e| format!("Failed to access youtube: {e:?}"))?;
                    tab.wait_until_navigated().map_err(|e| format!("Failed to wait until navigation: {e:?}"))?;
                    tab.wait_for_element("ytd-video-renderer").map_err(|e| format!("Failed to load search results: {e:?}"))?;
                    std::thread::sleep(Duration::from_secs(2)); 
                    match tab.get_content() {
                        Ok(html) => Ok(html),
                        Err(e) => Err(format!("Failed to get source: {e:?}"))
                    }
                }
                Err(e) => Err(format!("Failed to create tab: {e:?}"))
            }
        }
        Err(e) => Err(format!("Failed to launch browser: {e:?}"))
    }
}

pub fn get_top_result(html: &String) -> Option<String> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse(r#"ytd-video-renderer"#).unwrap();
    match fragment.select(&selector).nth(0) {
        Some(element) => {
            let url_selector = Selector::parse("a#video-title").unwrap();
            if let Some(url_element) = element.select(&url_selector).next() {
                if let Some(href) = url_element.value().attr("href") {
                    return Some(format!("https://www.youtube.com{}", href.trim()));
                }
            }
            None
        }
        None => None
    }
}
