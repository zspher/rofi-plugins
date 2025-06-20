use std::collections::HashMap;
use std::process::Command;
mod search;

use rofi_mode::Event;
use search::Search;
// use rofi_plugin_sys as ffi;

#[allow(dead_code)]
struct Mode<'rofi> {
    entries: Vec<String>,
    api: rofi_mode::Api<'rofi>,
}

impl Mode<'_> {
    fn parse_input(&self, input: &str) -> Option<String> {
        let (tag, search) = if let Some(con) = input.trim().strip_prefix('#') {
            match con.split_once(' ') {
                Some((t, q)) => (Some(t).filter(|x| !x.is_empty()), Some(q)),
                None => ((Some(con).filter(|x| !x.is_empty())), None),
            }
        } else {
            (None, Some(input))
        };

        let Ok(engines) = search::get_engine_list() else {
            eprintln!("Could not read search engine list file");
            return None;
        };

        let Some(data) = engines.get(tag.unwrap_or("ddg")) else {
            eprintln!("search engine tag not found");
            return None;
        };

        Some(match search {
            Some(s) => data.url.replace("{{{s}}}", s),
            None => format!("https://{}", data.default_url),
        })
    }
    fn open_url(&self, url: &str) {
        if let Err(why) = Command::new("sh")
            .arg("-c")
            .arg(format!("xdg-open \"{}\"", url))
            .spawn()
        {
            println!("Failed to perform websearch: {}", why);
        }
    }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");

        let entries = vec!["Search".into()];

        Ok(Self { api, entries })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("{}", entry)
    }

    fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> rofi_mode::Action {
        match event {
            Event::Cancel { selected: _ } => return rofi_mode::Action::Exit,
            Event::Ok {
                alt: _,
                selected: _,
            } => {
                if let Some(url) = self.parse_input(input) {
                    self.open_url(url.as_str());
                }
                return rofi_mode::Action::Exit;
            }
            Event::CustomInput {
                alt: _,
                selected: _,
            } => {
                return rofi_mode::Action::Exit;
            }
            Event::DeleteEntry { selected } => {
                // self.entries.remove(selected);
            }
            Event::Complete {
                selected: Some(selected),
            } => {
                // input.clear();
                // input.push_str(&self.entries[selected]);
            }
            Event::CustomCommand {
                number,
                selected: _,
            } => {
                return rofi_mode::Action::SetMode(number as u16);
            }
            Event::Complete { .. } => (),
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        if line == 0 {
            return true;
        }
        matcher.matches(&self.entries[line])
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        input.into()
    }
}

rofi_mode::export_mode!(Mode);
