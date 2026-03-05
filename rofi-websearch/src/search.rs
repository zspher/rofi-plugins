use std::collections::HashMap;
use std::fs;
use std::io;

use curl::easy::Easy;
use serde::Deserialize;
use serde_json::from_str;
use urlencoding::encode;

pub struct SearchSitesData(HashMap<Box<str>, SearchSite>);

impl Default for SearchSitesData {
    fn default() -> Self {
        Self(HashMap::from([(
            "dax".into(),
            SearchSite {
                title: "DuckDuckGo test".into(),
                default_url: "duckduckgo.com".into(),
                url: "https://duckduckgo.com/?q={{{s}}}".into(),
            },
        )]))
    }
}

impl SearchSitesData {
    const DATA_FILE: &str = "rofi-websearch-data.json";

    pub fn init() -> Self {
        match Self::get_sites_data() {
            Err(e) => {
                eprintln!("unable to load sites data file: {e}");
                Self::default()
            }
            Ok(d) => d,
        }
    }

    fn download_data() -> Result<(), io::Error> {
        let data_path = dirs::data_dir()
            .ok_or(io::Error::from(io::ErrorKind::NotFound))?
            .join(Self::DATA_FILE);

        let mut file = fs::File::create(data_path)?;

        let mut easy = Easy::new();
        easy.url(
            "https://raw.githubusercontent.com/kagisearch/bangs/refs/heads/main/data/bangs.json",
        )?;

        easy.write_function(move |data| match io::Write::write(&mut file, data) {
            Ok(_) => Ok(data.len()),
            Err(_) => Err(curl::easy::WriteError::Pause),
        })?;

        easy.perform()?;

        let status = easy.response_code()?;
        if status != 200 {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        Ok(())
    }

    fn get_sites_data() -> Result<Self, io::Error> {
        let data_path = dirs::data_dir()
            .ok_or(io::Error::from(io::ErrorKind::NotFound))?
            .join(Self::DATA_FILE);

        if !data_path.exists() {
            Self::download_data()?;
            println!("downloaded sites data file");
        }

        let file = fs::read_to_string(data_path)?;
        let data: Vec<SearchSiteRaw> = from_str(&file)?;

        let mut sites: HashMap<Box<str>, SearchSite> = HashMap::new();
        for site in data {
            let mut tag = site.t;
            if let Some(ts) = site.ts {
                tag = ts.iter().fold(tag, |acc, item| {
                    if item.len() < acc.len() {
                        item.clone()
                    } else {
                        acc
                    }
                });
            }
            sites.insert(
                tag,
                SearchSite {
                    title: site.s,
                    default_url: site.d,
                    url: site.u,
                },
            );
        }

        Ok(Self(sites))
    }

    pub fn get_title_from_input(&self, input: &str) -> Option<&str> {
        let query = SearchQuery::get(input);
        let data = self.0.get(&query.tag)?;
        Some(&data.title)
    }

    pub fn get_url_from_input(&self, input: &str) -> Option<String> {
        let query = SearchQuery::get(input);
        let Some(data) = self.0.get(&query.tag) else {
            eprintln!("search engine tag not found");
            return None;
        };

        Some(match query.query {
            Some(s) => {
                let url = &encode(&s).replace("%2F", "/");
                data.url.replace("{{{s}}}", url)
            }
            None => format!("https://{}", data.default_url),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct SearchQuery {
    pub tag: Box<str>,
    pub query: Option<Box<str>>,
}

impl SearchQuery {
    fn get(input: &str) -> Self {
        let (tag, query) = if let Some(start) = input.find('#') {
            let rest = &input[start..];
            let end = start + rest.find([' ', '\n']).unwrap_or(rest.len());
            let tag = &input[start + 1..end];

            let before = input[..start].trim();
            let after = input[end..].trim();
            let query: Option<Box<str>> = match (before.is_empty(), after.is_empty()) {
                (true, true) => None,
                (true, false) => Some(after.into()),
                (false, true) => Some(before.into()),
                (false, false) => Some(format!("{before} {after}").into()),
            };

            (Some(tag), query)
        } else {
            (None, Some(input.trim().into()))
        };

        SearchQuery {
            tag: tag.unwrap_or("dax").into(),
            query,
        }
    }
}

#[derive(Deserialize, Debug)]
struct SearchSiteRaw {
    pub s: Box<str>,
    pub d: Box<str>,
    pub t: Box<str>,
    pub u: Box<str>,
    pub ts: Option<Box<[Box<str>]>>,
}

#[derive(Debug)]
pub struct SearchSite {
    pub title: Box<str>,
    pub default_url: Box<str>,
    pub url: Box<str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_query_parse_1() {
        let data = SearchQuery::get("#ddg test");
        let expected = SearchQuery {
            tag: "ddg".into(),
            query: Some("test".into()),
        };
        assert_eq!(data, expected);
    }

    #[test]
    fn search_query_parse_2() {
        let data = SearchQuery::get("a text #ddg test");
        let expected = SearchQuery {
            tag: "ddg".into(),
            query: Some("a text test".into()),
        };
        assert_eq!(data, expected);
    }
}
