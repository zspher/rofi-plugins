mod search;
use search::SearchSitesData;

use rofi_mode::{Action, Event};
use rofi_plugin_sys as ffi;
use std::process::Command;

struct Mode<'rofi> {
    previous_input: String,
    sites: SearchSitesData,
    entries: Vec<String>,
    #[allow(dead_code)]
    api: rofi_mode::Api<'rofi>,
}

fn open_url(url: &str) {
    if let Err(why) = Command::new("xdg-open").arg(url).output() {
        println!("Failed to perform websearch: {why}");
    }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");

        let sites = SearchSitesData::init();
        let entries = vec!["Search".into()];

        Ok(Self {
            previous_input: String::new(),
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
        unsafe {
            ffi::view::reload();
        }
        self.previous_input = input.into();
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        self.sites
            .get_title_from_input(&self.previous_input)
            .unwrap_or(" ")
            .into()
    }
}

rofi_mode::export_mode!(Mode);
