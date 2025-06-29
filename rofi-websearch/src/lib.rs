mod search;
use search::SearchSitesData;

// use rofi_plugin_sys as ffi;
use rofi_mode::{Action, Event};
use std::process::Command;

#[allow(dead_code)]
struct Mode<'rofi> {
    sites: SearchSitesData,
    entries: Vec<String>,
    api: rofi_mode::Api<'rofi>,
}

fn open_url(url: &str) {
    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!("xdg-open \"{}\"", url))
        .spawn()
    {
        println!("Failed to perform websearch: {}", why);
    }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");

        let sites = SearchSitesData::init();
        let entries = vec!["Search".into()];

        Ok(Self {
            api,
            entries,
            sites,
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("{}", entry)
    }

    fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> Action {
        match event {
            Event::Ok {
                alt: _,
                selected: _,
            } => {
                if let Some(url) = self.sites.get_url_from_input(input) {
                    open_url(&url);
                }
                return Action::Exit;
            }
            Event::CustomCommand {
                number,
                selected: _,
            } => {
                return Action::SetMode(number as u16);
            }
            _ => (),
        }
        Action::Exit
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
