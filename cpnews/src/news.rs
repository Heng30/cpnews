use super::util;
use anyhow::{anyhow, Result};
use serde_json::Value;

const MAX_NEWS_ITEM: usize = 30;

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CryptoCompareNews {
    #[serde(default, rename(deserialize = "Type"))]
    pub r#type: i32,

    #[serde(default, rename(deserialize = "Data"))]
    pub data: Vec<Value>,
}

pub fn fetch_cryptocompare() -> Result<Vec<NewsItem>> {
    const NEWS_API: &str = "https://min-api.cryptocompare.com/data/v2/news/?lang=EN";
    let resp = reqwest::blocking::get(NEWS_API)?.json::<CryptoCompareNews>()?;

    if resp.r#type != 100i32 {
        return Err(anyhow!("remove server error"));
    }

    let mut news_items = vec![];
    for item in resp.data.into_iter() {
        if news_items.len() >= MAX_NEWS_ITEM {
            break;
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

        let summary = match item.get("body") {
            Some(Value::String(v)) => {
                if v.is_empty() {
                    continue;
                }

                let words: Vec<&str> = v.split_whitespace().collect();
                if words.len() > 100 {
                    let v = words.into_iter().take(100).collect::<Vec<_>>().join(" ");
                    format!("{v}...)
                } else {
                    v.clone()
                }
            }
            _ => continue,
        };

        let date = match item.get("published_on") {
            Some(Value::Number(v)) => {
                if !v.is_i64() {
                    continue;
                }
                let v = v.as_i64().unwrap();
                util::time_from_utc_seconds(v)
            }
            _ => continue,
        };

        let link = match item.get("url") {
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

pub fn fetch_odaily() -> Result<Vec<NewsItem>> {
    const NEWS_API: &str = "https://www.odaily.news/v1/openapi/feeds";
    let resp = reqwest::blocking::get(NEWS_API)?.json::<OdailyNews>()?;

    if resp.code != 0i32 {
        return Err(anyhow!("remove server error"));
    }

    let mut news_items = vec![];

    for item in resp.data.arr_news.into_iter() {
        if news_items.len() >= MAX_NEWS_ITEM {
            break;
        }

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
