use std::collections::HashMap;
use std::fs;
use std::io;

use curl::easy::Easy;
use serde::Deserialize;
use serde_json::from_str;
use urlencoding::encode;

pub struct SearchSitesData(HashMap<String, SearchSite>);

impl Default for SearchSitesData {
    fn default() -> Self {
        Self(HashMap::from([(
            "ddg".into(),
            SearchSite {
                title: "DuckDuckGo".into(),
                default_url: "duckduckgo.com".into(),
                id: "ddg".into(),
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

        easy.write_function(move |data| {
            let _ = io::Write::write(&mut file, data).map_err(|_| curl::easy::WriteError::Pause);
            Ok(data.len())
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
        let data: Vec<SearchSite> = from_str(&file)?;

        Ok(Self(data.into_iter().map(|x| (x.id.clone(), x)).collect()))
    }

    pub fn get_title_from_input(&self, input: &str) -> Option<&str> {
        let query = SearchQuery::get(input);
        let data = self.0.get(query.tag)?;
        Some(&data.title)
    }

    pub fn get_url_from_input(&self, input: &str) -> Option<String> {
        let query = SearchQuery::get(input);
        let Some(data) = self.0.get(query.tag) else {
            eprintln!("search engine tag not found");
            return None;
        };

        Some(match query.query {
            Some(s) => data.url.replace("{{{s}}}", &encode(s)),
            None => format!("https://{}", data.default_url),
        })
    }
}

pub struct SearchQuery<'a> {
    pub tag: &'a str,
    pub query: Option<&'a str>,
}

impl<'a> SearchQuery<'a> {
    fn get(input: &'a str) -> Self {
        let (tag, search) = if let Some(con) = input.trim().strip_prefix('#') {
            match con.split_once(' ') {
                Some((t, q)) => (Some(t).filter(|x| !x.is_empty()), Some(q)),
                None => ((Some(con).filter(|x| !x.is_empty())), None),
            }
        } else {
            (None, Some(input))
        };
        SearchQuery {
            tag: tag.unwrap_or("ddg"),
            query: search,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchSite {
    #[serde(rename = "s")]
    pub title: String,
    #[serde(rename = "d")]
    pub default_url: String,
    #[serde(rename = "t")]
    pub id: String,
    #[serde(rename = "u")]
    pub url: String,
}
