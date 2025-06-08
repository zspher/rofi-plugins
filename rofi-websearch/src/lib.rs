use std::process::Command;

use rofi_mode::Event;
// use rofi_plugin_sys as ffi;

#[allow(dead_code)]
struct Mode<'rofi> {
    entries: Vec<String>,
    api: rofi_mode::Api<'rofi>,
}

fn parse_input(p: &str) -> (Option<&str>, Option<&str>) {
    if let Some(rest) = p.strip_prefix("#") {
        // Split at the first space
        let (prefix, search) = rest
            .split_once(' ')
            .map(|(a, b)| (Some(a), Some(b)))
            .unwrap_or((Some(rest), None)); // No space: treat whole thing as prefix, no search
        (prefix, search)
    } else {
        (None, Some(p))
    }
}

fn open_engine(url: &str, query: &str) {
    if let Err(why) = Command::new("sh")
        .arg("-c")
        .arg(format!("xdg-open \"{}\"", url.replace("{{{s}}}", query)))
        .spawn()
    {
        println!("Failed to perform websearch: {}", why);
    }
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "websearch\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("websearch");
        let entries = vec!["search".into()];

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
                let (command, search) = parse_input(input);
                let url = search_engines::SEARCH_ENTRIES
                    .get(command.unwrap_or("ddg"))
                    .map(|x| x.url)
                    .expect("Missing URL for search engine");

                let search = search.unwrap_or("");

                open_engine(url, search);
                return rofi_mode::Action::Exit;
            }
            Event::CustomInput {
                alt: _,
                selected: _,
            } => {
                return rofi_mode::Action::Exit;
            }
            Event::DeleteEntry { selected } => {
                self.entries.remove(selected);
            }
            Event::Complete {
                selected: Some(selected),
            } => {
                println!("test");
                input.clear();
                input.push_str(&self.entries[selected]);
            }
            Event::CustomCommand {
                number,
                selected: _,
            } => {
                return rofi_mode::Action::SetMode(number as u16);
            }
            Event::Complete { .. } => {}
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
