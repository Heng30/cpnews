use anyhow::{anyhow, Result};
use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct NewsItem {
    pub title: String,
    pub summary: String,
    pub date: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OdailyNews {
    pub code: i32,
    pub data: OdailyNewsData,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OdailyNewsData {
    pub arr_news: Vec<Value>,
}

pub fn fetch() -> Result<Vec<NewsItem>> {
    const ODAILY_NEWS_API: &str = "https://www.odaily.news/v1/openapi/feeds";
    let resp = reqwest::blocking::get(ODAILY_NEWS_API)?.json::<OdailyNews>()?;

    if resp.code != 0i32 {
        return Err(anyhow!("remove server error"));
    }

    let mut news_items = vec![];

    for item in resp.data.arr_news.into_iter() {
        match item.get("type") {
            Some(Value::String(v)) => {
                if v != "newsflashes" {
                    continue;
                }
            }
            _ => continue,
        }

        let title = match item.get("title") {
            Some(Value::String(v)) => {
                if v.is_empty() {
                    continue;
                }
                v.clone()
            }
            _ => continue,
        };

        let summary = match item.get("description") {
            Some(Value::String(v)) => {
                if v.is_empty() {
                    continue;
                }
                v.trim().to_string()
            }
            _ => continue,
        };

        let date = match item.get("published_at") {
            Some(Value::String(v)) => {
                if v.is_empty() {
                    continue;
                }
                v.clone()
            }
            _ => continue,
        };

        let link = match item.get("link") {
            Some(Value::String(v)) => {
                if v.is_empty() {
                    continue;
                }
                v.clone()
            }
            _ => continue,
        };

        news_items.push(NewsItem {
            title,
            summary,
            date,
            link,
        });
    }

    Ok(news_items)
}
