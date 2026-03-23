use std::process::Command;

use rofi_mode::{Action, Event};

struct Mode<'rofi> {
    entries: Vec<String>,
    #[allow(dead_code)]
    api: rofi_mode::Api<'rofi>,
}

fn get_clipboard_history() -> Vec<String> {
    let mut cmd = Command::new("cliphist");
    cmd.arg("list");
    let binding = cmd.output().unwrap();
    let a = match str::from_utf8(&binding.stdout) {
        Ok(v) => v,
        Err(_) => panic!("got non UTF-8 data from git"),
    };
    let mut ret: Vec<String> = vec![];
    for l in a.lines() {
        ret.push(l.replace('\0', ""));
    }
    ret
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "clipboard\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("clipboard");
        Ok(Self {
            entries: get_clipboard_history(),
            api,
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("{}", entry)
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            Event::Ok {
                alt: _,
                selected: _,
            } => {
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
        matcher.matches(&self.entries[line])
    }
}

rofi_mode::export_mode!(Mode);
