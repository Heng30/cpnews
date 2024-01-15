use super::util;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::{fs, path::Path};

const MAX_NEWS_ITEM: usize = 30;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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

pub fn fetch_cryptocompare(path: &Path) -> Result<Vec<NewsItem>> {
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
                    format!("{v}...")
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

    if !news_items.is_empty() {
        let _ = save(path, &news_items);
    }

    Ok(news_items)
}

pub fn fetch_odaily(path: &Path) -> Result<Vec<NewsItem>> {
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

    if !news_items.is_empty() {
        let _ = save(path, &news_items);
    }

    Ok(news_items)
}

pub fn load(cache_dir: &Path) -> (Vec<NewsItem>, Vec<NewsItem>) {
    let cn_items = {
        let text = fs::read_to_string(cache_dir.join("news-cn.json")).unwrap_or(String::default());
        serde_json::from_str::<Vec<NewsItem>>(&text).unwrap_or(vec![])
    };

    let en_items = {
        let text = fs::read_to_string(cache_dir.join("news-en.json")).unwrap_or(String::default());
        serde_json::from_str::<Vec<NewsItem>>(&text).unwrap_or(vec![])
    };

    (cn_items, en_items)
}

fn save(path: &Path, items: &Vec<NewsItem>) -> Result<()> {
    match serde_json::to_string_pretty(items) {
        Ok(text) => Ok(fs::write(path, text)?),
        Err(e) => Err(anyhow!("{e:?}")),
    }
}
